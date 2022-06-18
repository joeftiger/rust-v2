pub mod obj;

use crate::geometry::bvh::Tree;
use crate::geometry::{max3, min3, Aabb, Geometry, Intersection, Ray};
use crate::{Float, Rot3, Vec3};
use cgmath::{ElementWise, InnerSpace, Rotation, Zero};
use core::convert::TryFrom;
use obj::ObjFile;
use serde::{Deserialize, Serialize, Serializer};

/// A triangle consists of vertex indices `(v0, v1, v2)`.
///
/// In order to query a triangle for an intersection, it is therefore needed to pass it the proper mesh.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Face {
    pub v: (u32, u32, u32),
    pub vn: (u32, u32, u32),
}

impl Face {
    #[inline]
    pub const fn new(v: (u32, u32, u32), vn: (u32, u32, u32)) -> Self {
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
    pub fn get_normals(&self, normals: &[Vec3]) -> (Vec3, Vec3, Vec3) {
        (
            normals[self.vn.0 as usize],
            normals[self.vn.1 as usize],
            normals[self.vn.2 as usize],
        )
    }

    pub fn face_normal(&self, vertices: &[Vec3]) -> Vec3 {
        let (v0, v1, v2) = self.get_vertices(vertices);
        (v1 - v0).cross(v2 - v0)
    }

    pub fn bounds(&self, vertices: &[Vec3]) -> Aabb {
        let (v0, v1, v2) = self.get_vertices(vertices);
        Aabb::new(min3(v0, v1, v2), max3(v0, v1, v2))
    }

    #[cfg(not(feature = "watertight-mesh"))]
    fn intersect(&self, mesh: &Mesh, ray: Ray) -> Option<Intersection> {
        use crate::util::floats;

        let (p0, p1, p2) = self.get_vertices(&mesh.vertices);

        let edge1 = p1 - p0;
        let edge2 = p2 - p0;

        let h = ray.direction.cross(edge2);
        let a = edge1.dot(h);

        // ray is parallel to triangle
        if floats::approx_eq(a, 0.0) {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin - p0;
        let beta = f * s.dot(h);
        #[allow(clippy::manual_range_contains)]
        if beta < 0.0 || 1.0 < beta {
            return None;
        }

        let q = s.cross(edge1);
        let gamma = f * ray.direction.dot(q);
        if gamma < 0.0 || 1.0 < beta + gamma {
            return None;
        }

        let t = f * edge2.dot(q);
        if !ray.contains(t) {
            return None;
        }

        let point = ray.at(t);

        let normal = match mesh.shading_mode {
            ShadingMode::Flat => edge1.cross(edge2),
            ShadingMode::Phong => {
                let (n0, n1, n2) = self.get_normals(&mesh.normals);
                let alpha = 1.0 - beta - gamma;

                alpha * n0 + beta * n1 + gamma * n2
            }
        }
        .normalize();

        Some(Intersection::new(point, normal, ray.direction, t))
    }

    #[cfg(not(feature = "watertight-mesh"))]
    fn intersects(&self, mesh: &Mesh, ray: Ray) -> bool {
        use crate::util::floats;

        let (p0, p1, p2) = self.get_vertices(&mesh.vertices);

        let edge1 = p1 - p0;
        let edge2 = p2 - p0;

        let h = ray.direction.cross(edge2);
        let a = edge1.dot(h);

        // ray is parallel to triangle
        if floats::approx_eq(a, 0.0) {
            return false;
        }

        let f = 1.0 / a;
        let s = ray.origin - p0;
        let beta = f * s.dot(h);
        #[allow(clippy::manual_range_contains)]
        if beta < 0.0 || 1.0 < beta {
            return false;
        }

        let q = s.cross(edge1);
        let gamma = f * ray.direction.dot(q);
        if gamma < 0.0 || 1.0 < beta + gamma {
            return false;
        }

        let t = f * edge2.dot(q);
        ray.contains(t)
    }

    #[cfg(feature = "watertight-mesh")]
    #[allow(clippy::many_single_char_names)]
    fn intersect(&self, mesh: &Mesh, ray: Ray) -> Option<Intersection> {
        use crate::geometry::{abs, max_index};

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
            core::mem::swap(&mut kx, &mut ky);
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
            ShadingMode::Phong => {
                let (n0, n1, n2) = self.get_normals(&mesh.normals);

                let beta = u * inv_det;
                let gamma = v * inv_det;
                let alpha = 1.0 - beta - gamma;

                alpha * n0 + beta * n1 + gamma * n2
            }
        }
        .normalize();

        Some(Intersection::new(point, normal, ray.direction, t))
    }

    #[cfg(feature = "watertight-mesh")]
    #[allow(clippy::many_single_char_names)]
    fn intersects(&self, mesh: &Mesh, ray: Ray) -> bool {
        use crate::geometry::{abs, max_index};

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
            core::mem::swap(&mut kx, &mut ky);
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
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShadingMode {
    Flat,
    Phong,
}

#[derive(Deserialize)]
#[serde(try_from = "MeshSerde")]
pub struct Mesh {
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
    faces: Vec<Face>,
    shading_mode: ShadingMode,
    bvh: Tree,
}

impl Mesh {
    pub fn new(
        vertices: Vec<Vec3>,
        normals: Vec<Vec3>,
        faces: Vec<Face>,
        shading_mode: ShadingMode,
    ) -> Self {
        Self {
            vertices,
            normals,
            faces,
            shading_mode,
            bvh: Default::default(),
        }
    }

    /// Determines the weights by which to scale triangle (p0, p1, p2)'s normal when
    /// accumulating the vertex normal for vertices 0, 1, 2.
    ///
    /// # Constraints
    /// * `p0` - All values should be finite (neither infinite nor `NaN`).
    /// * `p1` - All values should be finite.
    /// * `p2` - All values should be finite.
    ///
    /// # Arguments
    /// * `p0` - The position 0 of a triangle
    /// * `p1` - The position 1 of a triangle
    /// * `p2` - The position 2 of a triangle
    ///
    /// # Returns
    /// * `w0` - The weight for vertex 0
    /// * `w1` - The weight for vertex 1
    /// * `w2` - The weight for vertex 2
    pub fn angle_weights(p0: Vec3, p1: Vec3, p2: Vec3) -> (Float, Float, Float) {
        let e01 = (p1 - p0).normalize();
        let e12 = (p2 - p1).normalize();
        let e20 = (p0 - p2).normalize();

        let w0 = e01.dot(-e20).clamp(-1.0, 1.0);
        let w1 = e12.dot(-e01).clamp(-1.0, 1.0);
        let w2 = e20.dot(-e12).clamp(-1.0, 1.0);

        (w0, w1, w2)
    }

    pub fn translate(&mut self, translation: Vec3) -> &mut Self {
        for v in &mut self.vertices {
            *v += translation;
        }

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

    #[must_use]
    pub fn build(mut self) -> Self {
        if self.shading_mode == ShadingMode::Phong && self.normals.is_empty() {
            log::info!(target: "Mesh", "computing normals...");
            self.normals = vec![Vec3::zero(); self.vertices.len()];

            for face in &self.faces {
                let (v0, v1, v2) = face.get_vertices(&self.vertices);
                let normal = face.face_normal(&self.vertices);
                let (w0, w1, w2) = Self::angle_weights(v0, v1, v2);

                self.normals[face.vn.0 as usize] += w0 * normal;
                self.normals[face.vn.1 as usize] += w1 * normal;
                self.normals[face.vn.2 as usize] += w2 * normal;
            }

            for n in &mut self.normals {
                *n = n.normalize();
            }
            log::info!(target: "Mesh", "computed normals!");
        }

        log::info!(target: "Mesh", "computing BVH (might take a while)...");
        let values: Vec<u32> = (0..self.faces.len() as u32).collect();
        self.bvh = Tree::new(&values, |i| self.faces[i as usize].bounds(&self.vertices));
        log::info!(target: "Mesh", "computed BVH!");
        self
    }
}

#[typetag::serde]
impl Geometry for Mesh {
    #[inline(always)]
    fn contains(&self, _point: Vec3) -> Option<bool> {
        None
    }

    #[inline]
    fn bounds(&self) -> Aabb {
        self.bvh.bounds()
    }

    fn intersect(&self, mut ray: Ray) -> Option<Intersection> {
        let mut intersection = None;

        for face_hit in self.bvh.intersect(ray) {
            if let Some(i) = self.faces[face_hit as usize].intersect(self, ray) {
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
            .any(|&i| self.faces[i as usize].intersects(self, ray))
    }
}

impl Serialize for Mesh {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        MeshSerde::Checkpoint(MeshCheckpoint {
            vertices: self.vertices.clone(),
            normals: self.normals.clone(),
            faces: self.faces.clone(),
            shading_mode: self.shading_mode,
        })
        .serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
enum MeshSerde {
    Config(MeshConfig),
    Checkpoint(MeshCheckpoint),
}
impl TryFrom<MeshSerde> for Mesh {
    type Error = String;

    fn try_from(serde: MeshSerde) -> Result<Self, Self::Error> {
        match serde {
            MeshSerde::Config(c) => {
                let obj = ObjFile::load(&c.path)?;
                let mut mesh = Mesh::new(obj.vertices, obj.normals, obj.faces, c.shading_mode);

                if let Some(s) = c.scale {
                    mesh.scale(s);
                }
                if let Some(r) = c.rotation {
                    mesh.rotate(r);
                }
                if let Some(t) = c.translation {
                    mesh.translate(t);
                }
                Ok(mesh.build())
            }
            MeshSerde::Checkpoint(c) => {
                let mesh = Mesh::new(c.vertices, c.normals, c.faces, c.shading_mode);
                Ok(mesh.build())
            }
        }
    }
}

#[derive(Deserialize, Serialize)]
struct MeshConfig {
    /// The path of the mesh file
    path: String,
    /// Optional scaling (1st application)
    #[serde(default)]
    scale: Option<Vec3>,
    /// Optional rotation (2nd application)
    /// - params: (axis, angle)
    #[serde(default)]
    rotation: Option<Rot3>,
    /// Optional translation (3rd application)
    #[serde(default)]
    translation: Option<Vec3>,
    shading_mode: ShadingMode,
}

#[derive(Deserialize, Serialize)]
struct MeshCheckpoint {
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
    faces: Vec<Face>,
    shading_mode: ShadingMode,
}
