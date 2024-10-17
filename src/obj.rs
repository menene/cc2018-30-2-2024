use tobj;
use nalgebra_glm::{Vec2, Vec3};
use crate::vertex::Vertex;

pub struct Obj {
    meshes: Vec<Mesh>,
}

struct Mesh {
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
    texcoords: Vec<Vec2>,
    indices: Vec<u32>,
}

impl Obj {
    pub fn load(filename: &str) -> Result<Self, tobj::LoadError> {
        let (models, _) = tobj::load_obj(filename, &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        })?;

        let meshes = models.into_iter().map(|model| {
            let mesh = model.mesh;
            Mesh {
                vertices: mesh.positions.chunks(3)
                    .map(|v| Vec3::new(v[0], v[1], v[2]))
                    .collect(),
                normals: mesh.normals.chunks(3)
                    .map(|n| Vec3::new(n[0], n[1], n[2]))
                    .collect(),
                texcoords: mesh.texcoords.chunks(2)
                    .map(|t| Vec2::new(t[0], 1.0 - t[1]))
                    .collect(),
                indices: mesh.indices,
            }
        }).collect();

        Ok(Obj { meshes })
    }

    pub fn get_vertex_array(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new();

        for mesh in &self.meshes {
            for &index in &mesh.indices {
                let position = mesh.vertices[index as usize];
                let normal = mesh.normals.get(index as usize)
                    .cloned()
                    .unwrap_or(Vec3::new(0.0, 1.0, 0.0));
                let tex_coords = mesh.texcoords.get(index as usize)
                    .cloned()
                    .unwrap_or(Vec2::new(0.0, 0.0));

                vertices.push(Vertex::new(position, normal, tex_coords));
            }
        }

        vertices
    }
}
