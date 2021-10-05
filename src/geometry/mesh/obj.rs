use super::Face;
use crate::{Float, Vec3};
use core::str::SplitWhitespace;

pub struct ObjFile {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub faces: Vec<Face>,
}

impl ObjFile {
    pub fn load(path: &str) -> Result<Self, String> {
        let content = std::fs::read_to_string(path).unwrap();

        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut faces = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            let mut iter = line.splitn(2, ' ');

            let id = iter
                .next()
                .ok_or(format!("line {}: invalid line length", line_num))?;
            let part = iter
                .next()
                .ok_or(format!("line {}: missing part of id [{}]", line_num, id))?;

            match id {
                c @ ("v" | "vn") => {
                    let vec =
                        Self::parse_vec3(part).map_err(|e| format!("line {}: {}", line_num, e))?;

                    if c == "v" {
                        vertices.push(vec);
                    } else {
                        normals.push(vec);
                    }
                }
                "f" => {
                    let face =
                        Self::parse_face(part).map_err(|e| format!("line {}: {}", line_num, e))?;
                    faces.push(face);
                }
                _ => {
                    return Err(format!(
                        "line {}: unknown. we only know 'v', 'vn' or 'f'",
                        line_num
                    ))
                }
            }
        }

        Ok(Self {
            vertices,
            normals,
            faces,
        })
    }

    fn parse_float(iter: &mut SplitWhitespace) -> Result<Float, String> {
        match iter.next() {
            None => Err("expecting a float, none given".into()),
            Some(s) => match s.parse::<Float>() {
                Ok(float) => Ok(float),
                Err(e) => Err(e.to_string()),
            },
        }
    }

    fn parse_vec3(part: &str) -> Result<Vec3, String> {
        let mut iter = part.split_whitespace();
        let x = Self::parse_float(&mut iter)?;
        let y = Self::parse_float(&mut iter)?;
        let z = Self::parse_float(&mut iter)?;

        Ok(Vec3::new(x, y, z))
    }

    fn parse_face_component(part: &str) -> Result<(u32, Option<u32>), String> {
        let mut split = part.splitn(3, '/');

        let v = split
            .next()
            .ok_or_else(|| "missing vertex".to_string())?
            .parse::<u32>()
            .map_err(|e| e.to_string())?;

        // skip texture coordinate
        let mut split = split.skip(1);

        let vn = match split.next() {
            Some(s) => Some(s.parse::<u32>().map_err(|e| e.to_string())?),
            None => None,
        };

        Ok((v, vn))
    }

    fn parse_face(part: &str) -> Result<Face, String> {
        let mut iter = part.split_whitespace();
        let (v0, n0) = Self::parse_face_component(
            iter.next()
                .ok_or_else(|| "missing face component".to_string())?,
        )?;
        let (v1, n1) = Self::parse_face_component(
            iter.next()
                .ok_or_else(|| "missing face component".to_string())?,
        )?;
        let (v2, n2) = Self::parse_face_component(
            iter.next()
                .ok_or_else(|| "missing face component".to_string())?,
        )?;

        let v = (v0 - 1, v1 - 1, v2 - 1);
        let vn = match (n0, n1, n2) {
            (None, None, None) => None,
            (Some(a), Some(b), Some(c)) => Some((a - 1, b - 1, c - 1)),
            _ => return Err("invalid vertex normal mixup".into()),
        };

        Ok(Face::new(v, vn))
    }
}
