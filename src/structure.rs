//! Renderable
//!
//! Trait and Implementations for Generating Representations.
//!
//!

// use bevy::prelude::*;
use super::ColorScheme;
use bevy::prelude::{
    default, Assets, Color, ColorToComponents, Commands, Component, Mesh, MeshBuilder, Meshable,
    PbrBundle, ResMut, Sphere, StandardMaterial, Transform, Vec3,
};
use bon::Builder;
use pdbtbx::PDB;

/// Enum representing various rendering options.
pub enum RenderOptions {
    Wireframe,
    Solid,
    Textured(String),
    LevelOfDetail(u32),
    // Custom(Box<dyn Fn() -> bevy::prelude::Mesh>),
}

// Core Structure for Rendering.
#[derive(Builder, Component)]
pub struct Structure {
    pdb: PDB,
    #[builder(default = RenderOptions::Solid)]
    rendertype: RenderOptions,
    #[builder(default = ColorScheme::Solid(Color::WHITE))]
    color_scheme: ColorScheme,
}

impl Structure {
    pub fn render(&self) -> Mesh {
        match &self.rendertype {
            RenderOptions::Wireframe => {
                todo!()
            }
            RenderOptions::Solid => {
                let mut meshes = Vec::new();
                for atom in self.pdb.atoms() {
                    let (x, y, z) = atom.pos();
                    let center = Vec3::new(x as f32, y as f32, z as f32);
                    let radius =
                        atom.element()
                            .expect("Atom Element not Defined")
                            .atomic_radius()
                            .van_der_waals
                            .expect("Van der waals not defined") as f32;
                    let color = self.color_scheme.get_color(atom).to_srgba();
                    let mut sphere_mesh = Sphere::new(radius).mesh().build();
                    let vertex_count = sphere_mesh.count_vertices();
                    let color_array = vec![color.to_vec4(); vertex_count];
                    sphere_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, color_array);
                    sphere_mesh = sphere_mesh.translated_by(center);
                    sphere_mesh.compute_smooth_normals();
                    meshes.push(sphere_mesh);
                }

                // combine all the meshes together
                meshes
                    .into_iter()
                    .reduce(|mut acc, mesh| {
                        acc.merge(&mesh);
                        acc
                    })
                    .unwrap()
            }
            RenderOptions::Textured(_) => {
                todo!()
            }
            RenderOptions::LevelOfDetail(_) => {
                todo!()
            }
        }
    }
}

fn spawn_protein(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    protein: &Structure,
) {
    let mesh = protein.render();
    let mesh_handle = meshes.add(mesh);

    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.6),
        metallic: 0.1,
        perceptual_roughness: 0.5,
        reflectance: 0.5,
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: mesh_handle,
        material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use pdbtbx::StrictnessLevel;

    #[test]
    fn test_pdb_to_mesh() {
        let (pdb, _errors) = pdbtbx::open("examples/1fap.cif", StrictnessLevel::Medium).unwrap();
        let structure = Structure::builder().pdb(pdb).build();
        assert_eq!(structure.pdb.atom_count(), 2154);
        let mesh = structure.render();
        assert_eq!(mesh.count_vertices(), 779748);
    }
}
