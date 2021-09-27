pub mod obj;

use crate::geometry::bvh::Tree;
use crate::geometry::{abs, max2, max3, max_index, min2, min3, Aabb, Geometry, Intersection, Ray};
use crate::{Rot3, Vec3};
use cgmath::{Bounded, ElementWise, InnerSpace, Rotation};
use core::convert::TryFrom;
use core::mem;
use obj::ObjFile;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};

/// A triangle consists of vertex indices `(v0, v1, v2)`.
///
/// In order to query a triangle for an intersection, it is therefore needed to pass it the proper mesh.
#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Face {
    pub v: (u32, u32, u32),
    pub vn: Option<(u32, u32, u32)>,
}

impl Face {
    #[inline]
    pub const fn new(v: (u32, u32, u32), vn: Option<(u32, u32, u32)>) -> Self {
        Self { v, vn }
    }

    #[inline]
    pub fn get_vertices(&self, vertices: &[Vec3]) -> (Vec3, Vec3, Vec3) {
        (
            vertices[self.v.0 as usize],
            vertices[self.v.1 as usize],
            vertices[self.v.2 as usize],
        )
    }

    #[inline]
    pub fn get_normals(&self, normals: &[Vec3]) -> Option<(Vec3, Vec3, Vec3)> {
        self.vn.map(|n| {
            (
                normals[n.0 as usize],
                normals[n.1 as usize],
                normals[n.2 as usize],
            )
        })
    }

    pub fn bounds(&self, vertices: &[Vec3]) -> Aabb {
        let (v0, v1, v2) = self.get_vertices(vertices);
        Aabb::new(min3(v0, v1, v2), max3(v0, v1, v2))
    }

    #[allow(clippy::many_single_char_names)]
    fn intersect(&self, mesh: &Mesh, ray: Ray) -> Option<Intersection> {
        let (v0, v1, v2) = self.get_vertices(&mesh.vertices);

        let dir = ray.direction;
        // calculate dimension where the ray direction is maximal
        let kz = max_index(abs(dir));
        let mut kx = kz + 1;
        if kx == 3 {
            kx = 0
        }
        let mut ky = kx + 1;
        if ky == 3 {
            ky = 0
        }

        // swap dimension to preserve winding direction of triangles
        if dir[kz] < 0.0 {
            mem::swap(&mut kx, &mut ky);
        }

        // calculate shear constants
        let sx = dir[kx] / dir[kz];
        let sy = dir[ky] / dir[kz];
        let sz = 1.0 / dir[kz];

        // calculate vertices relative to ray origin
        let a = v0 - ray.origin;
        let b = v1 - ray.origin;
        let c = v2 - ray.origin;

        // perform shear and scale of vertices
        let ax = a[kx] - sx * a[kz];
        let ay = a[ky] - sy * a[kz];
        let bx = b[kx] - sx * b[kz];
        let by = b[ky] - sy * b[kz];
        let cx = c[kx] - sx * c[kz];
        let cy = c[ky] - sy * c[kz];

        // calculate scaled barycentric coordinates
        let u = cx * by - cy * bx;
        let v = ax * cy - ay * cx;
        let w = bx * ay - by * ax;

        // perform edge tests
        if u < 0.0 || v < 0.0 || w < 0.0 {
            return None;
        }

        // calculate determinant
        let det = u + v + w;
        if det == 0.0 {
            return None;
        }

        // for normalization
        let inv_det = 1.0 / det;

        // calculate scaled z-coordinates of vertices and use them to calculate the hit distance
        let az = sz * a[kz];
        let bz = sz * b[kz];
        let cz = sz * c[kz];
        let t = (u * az + v * bz + w * cz) * inv_det;

        if !ray.contains(t) {
            return None;
        }

        let point = ray.at(t);

        let normal = match mesh.shading_mode {
            ShadingMode::Flat => (v1 - v0).cross(v2 - v0),
            ShadingMode::Phong => match self.get_normals(&mesh.normals) {
                None => (v1 - v0).cross(v2 - v0),
                Some((n0, n1, n2)) => {
                    let beta = u * inv_det;
                    let gamma = v * inv_det;
                    let alpha = 1.0 - beta - gamma;

                    alpha * n0 + beta * n1 + gamma * n2
                }
            },
        }
        .normalize();

        Some(Intersection::new(point, normal, t))
    }

    #[allow(clippy::many_single_char_names)]
    fn intersects(&self, mesh: &Mesh, ray: Ray) -> bool {
        let (v0, v1, v2) = self.get_vertices(&mesh.vertices);

        let dir = ray.direction;
        // calculate dimension where the ray direction is maximal
        let kz = max_index(abs(dir));
        let mut kx = kz + 1;
        if kx == 3 {
            kx = 0
        }
        let mut ky = kx + 1;
        if ky == 3 {
            ky = 0
        }

        // swap dimension to preserve winding direction of triangles
        if dir[kz] < 0.0 {
            mem::swap(&mut kx, &mut ky);
        }

        // calculate shear constants
        let sx = dir[kx] / dir[kz];
        let sy = dir[ky] / dir[kz];
        let sz = 1.0 / dir[kz];

        // calculate vertices relative to ray origin
        let a = v0 - ray.origin;
        let b = v1 - ray.origin;
        let c = v2 - ray.origin;

        // perform shear and scale of vertices
        let ax = a[kx] - sx * a[kz];
        let ay = a[ky] - sy * a[kz];
        let bx = b[kx] - sx * b[kz];
        let by = b[ky] - sy * b[kz];
        let cx = c[kx] - sx * c[kz];
        let cy = c[ky] - sy * c[kz];

        // calculate scaled barycentric coordinates
        let u = cx * by - cy * bx;
        let v = ax * cy - ay * cx;
        let w = bx * ay - by * ax;

        // perform edge tests
        if u < 0.0 || v < 0.0 || w < 0.0 {
            return false;
        }

        // calculate determinant
        let det = u + v + w;
        if det == 0.0 {
            return false;
        }

        // for normalization
        let inv_det = 1.0 / det;

        // calculate scaled z-coordinates of vertices and use them to calculate the hit distance
        let az = sz * a[kz];
        let bz = sz * b[kz];
        let cz = sz * c[kz];
        let t = (u * az + v * bz + w * cz) * inv_det;

        ray.contains(t)
    }
}

/// The shading mode defines the shading of normals. In `Flat` mode, the surface of triangles will
/// appear flat. In `Phong` however, they will be interpolated to create a smooth looking surface.
#[derive(Debug, Serialize, Deserialize)]
pub enum ShadingMode {
    Flat,
    Phong,
}

#[derive(Serialize, Deserialize)]
enum MeshSerde {
    Obj(MeshObj),
    Mesh(Mesh),
}

#[derive(Serialize, Deserialize)]
struct MeshObj {
    /// The path of the mesh file
    path: String,
    /// Optional scaling (1st application)
    #[serde(default)]
    scale: Option<Vec3>,
    #[serde(default)]
    /// Optional rotation (2nd application)
    /// - params: (axis, angle)
    rotation: Option<Rot3>,
    #[serde(default)]
    /// Optional translation (3rd application)
    translation: Option<Vec3>,
    shading_mode: ShadingMode,
}

#[derive(Serialize)]
pub struct Mesh {
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
    faces: Vec<Face>,
    shading_mode: ShadingMode,
    #[serde(skip_serializing)]
    bounds: Aabb,
    #[serde(skip_serializing)]
    bvh: Tree<Face>,
}

impl Mesh {
    pub fn new(
        vertices: Vec<Vec3>,
        normals: Vec<Vec3>,
        faces: Vec<Face>,
        shading_mode: ShadingMode,
    ) -> Self {
        let min = vertices
            .iter()
            .fold(Vec3::max_value(), |acc, &next| min2(acc, next));
        let max = vertices
            .iter()
            .fold(Vec3::min_value(), |acc, &next| max2(acc, next));
        let bounds = Aabb::new(min, max);

        Self {
            vertices,
            normals,
            faces,
            shading_mode,
            bounds,
            bvh: Default::default(),
        }
    }

    pub fn translate(&mut self, translation: Vec3) -> &mut Self {
        for v in &mut self.vertices {
            *v += translation;
        }

        self.bounds.min += translation;
        self.bounds.max += translation;

        self
    }

    pub fn scale(&mut self, scale: Vec3) -> &mut Self {
        let scale_inv = Vec3::new(1.0, 1.0, 1.0).div_element_wise(scale);

        for v in &mut self.vertices {
            v.mul_assign_element_wise(scale);
        }

        for n in &mut self.normals {
            *n = n.mul_element_wise(scale_inv).normalize();
        }

        self.bounds.min.mul_assign_element_wise(scale);
        self.bounds.max.mul_assign_element_wise(scale);

        self
    }

    pub fn rotate(&mut self, rotation: Rot3) -> &mut Self {
        for v in &mut self.vertices {
            *v = rotation.rotate_vector(*v);
        }
        for n in &mut self.normals {
            *n = rotation.rotate_vector(*n);
        }

        self
    }

    pub fn build_tree(&mut self) {
        self.bvh = Tree::new(self.faces.clone(), |f| f.bounds(&self.vertices))
    }
}

impl TryFrom<MeshObj> for Mesh {
    type Error = String;

    fn try_from(mesh_obj: MeshObj) -> Result<Self, Self::Error> {
        let obj = ObjFile::load(&mesh_obj.path)?;
        let mut mesh = Mesh::new(obj.vertices, obj.normals, obj.faces, mesh_obj.shading_mode);

        if let Some(s) = mesh_obj.scale {
            mesh.scale(s);
        }
        if let Some(r) = mesh_obj.rotation {
            mesh.rotate(r);
        }
        if let Some(t) = mesh_obj.translation {
            mesh.translate(t);
        }

        Ok(mesh)
    }
}

impl<'de> Deserialize<'de> for Mesh {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let mut mesh = match MeshSerde::deserialize(deserializer)? {
            MeshSerde::Obj(obj) => Mesh::try_from(obj).map_err(D::Error::custom)?,
            MeshSerde::Mesh(m) => m,
        };

        mesh.build_tree();

        Ok(mesh)
    }
}

impl Geometry for Mesh {
    #[inline(always)]
    fn contains(&self, _point: Vec3) -> Option<bool> {
        None
    }

    #[inline]
    fn bounds(&self) -> Aabb {
        self.bounds
    }

    fn intersect(&self, mut ray: Ray) -> Option<Intersection> {
        let mut intersection = None;

        for hit in self.bvh.intersect(ray) {
            if let Some(i) = hit.intersect(self, ray) {
                ray.t_end = i.t;
                intersection = Some(i);
            }
        }

        intersection
    }

    fn intersects(&self, ray: Ray) -> bool {
        self.bvh
            .intersect(ray)
            .iter()
            .any(|t| t.intersects(self, ray))
    }
}
