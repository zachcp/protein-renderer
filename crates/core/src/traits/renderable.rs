//! Renderable
//!
//! Trait and Implementations for Generating Representations.
//!
//!
use bevy::prelude::*;
use bevy::prelude::{Color, Vec3};
use bevy::render::mesh::Mesh;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;
use bon::Builder;
use pdbtbx::{Atom, Chain, Residue, PDB};

/// Renderable
///

/// Enum representing various rendering options.
pub enum RenderOptions {
    Wireframe,
    Solid,
    Textured(String),
    LevelOfDetail(u32),
    // Custom(Box<dyn Fn() -> bevy::prelude::Mesh>),
}

pub enum ColorScheme {
    Solid(Color),
    // Rainbow,
    ByAtomType,
    // ByChain(Box<dyn Fn(&Chain) -> Color>),
    // BySecondaryStructure(Box<dyn Fn(&Residue) -> Color>),
    // ByResidueType(Box<dyn Fn(&Residue) -> Color>),
    // Custom(Box<dyn Fn(&Atom, &Residue, &Chain) -> Color>),
}

impl ColorScheme {
    pub fn get_color(&self, atom: &Atom) -> Color {
        match self {
            ColorScheme::Solid(color) => *color,
            ColorScheme::ByAtomType => {
                match atom.element().expect("expect atom").symbol() {
                    "C" => Color::srgb(0.5, 0.5, 0.5), // Carbon: Gray
                    "N" => Color::srgb(0.0, 0.0, 1.0), // Nitrogen: Blue
                    "O" => Color::srgb(1.0, 0.0, 0.0), // Oxygen: Red
                    "S" => Color::srgb(1.0, 1.0, 0.0), // Sulfur: Yellow
                    _ => Color::srgb(1.0, 1.0, 1.0),   // Other: White
                }
            } // ColorScheme::ByChain(func) => func(chain),
              // ColorScheme::BySecondaryStructure(func) => func(residue),
              // ColorScheme::ByResidueType(func) => func(residue),
              // ColorScheme::Custom(func) => func(atom, residue, chain),
        }
    }
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
                // TODO: Implement wireframe rendering
                todo!()
            }
            RenderOptions::Solid => {
                // Default solid rendering, continue with existing implementation
                // Here we would want to match on each render type and then dispatch against it.
                let mut positions = Vec::new();
                let mut indices = Vec::new();
                let mut colors = Vec::new();
                let resolution = 10;
                for atom in self.pdb.atoms() {
                    let (x, y, z) = atom.pos();
                    let (x, y, z) = (x as f32, y as f32, z as f32);
                    let center = Vec3::new(x, y, z);
                    let start_index = positions.len() as u32;
                    // let color = color_func(atom).to_srgba();
                    let color = self.color_scheme.get_color(atom).to_srgba();
                    // the conversion below is needed for compatibility with Blender.
                    let color_array: [f32; 4] = [color.red, color.green, color.blue, color.alpha];
                    let radius =
                        atom.element()
                            .expect("Atom Element not Defined")
                            .atomic_radius()
                            .van_der_waals
                            .expect("Van der waals not defined") as f32;
                    // Generate sphere vertices
                    for i in 0..=resolution {
                        for j in 0..=resolution {
                            let theta = i as f32 * std::f32::consts::PI / resolution as f32;
                            let phi = j as f32 * 2.0 * std::f32::consts::PI / resolution as f32;
                            let x = radius * theta.sin() * phi.cos();
                            let y = radius * theta.sin() * phi.sin();
                            let z = radius * theta.cos();
                            positions.push((center + Vec3::new(x, y, z)).to_array());
                            colors.push(color_array);
                        }
                    }
                    // Generate indices for triangles
                    for i in 0..resolution {
                        for j in 0..resolution {
                            let top_left = start_index + i * (resolution + 1) + j;
                            let top_right = top_left + 1;
                            let bottom_left = top_left + resolution + 1;
                            let bottom_right = bottom_left + 1;
                            indices.extend_from_slice(&[
                                top_left,
                                bottom_left,
                                top_right,
                                top_right,
                                bottom_left,
                                bottom_right,
                            ]);
                        }
                    }
                }
                let mut mesh = Mesh::new(
                    PrimitiveTopology::TriangleList,
                    RenderAssetUsages::default(),
                );
                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
                mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
                mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
                mesh.compute_smooth_normals();
                mesh
            }
            RenderOptions::Textured(_) => {
                // TODO: Implement textured rendering
                todo!()
            }
            RenderOptions::LevelOfDetail(_) => {
                // TODO: Implement level of detail rendering
                todo!()
            } // RenderOptions::Custom(_) => {
              //     // return custom_fn();
              //     todo!()
              // }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;
    use pdbtbx::StrictnessLevel;

    #[test]
    fn test_pdb_to_mesh() {
        let (pdb, _errors) = pdbtbx::open("examples/1fap.cif", StrictnessLevel::Medium).unwrap();

        let structure = Structure::builder.pdb(pdb).build();

        // TODO: Assert that the mesh has the expected number of vertices and indices
        assert_eq!(structure.pdb.atom_count(), 200);
    }

    #[test]
    fn test_empty_pdb() {
        // TODO: Create an empty PDB structure
        // let empty_pdb = PDB::new();

        // TODO: Call pdb_to_mesh function with empty PDB
        // let mesh = pdb_to_mesh(&empty_pdb);

        // TODO: Assert that the mesh is empty or has expected properties for an empty PDB
        // assert_eq!(mesh.count_vertices(), 0);
        // assert!(mesh.indices().is_none());
    }

    #[test]
    fn test_large_pdb() {
        // TODO: Load a large PDB file
        // let large_pdb = PDB::from_file("path/to/large/pdb/file.pdb").unwrap();

        // TODO: Call pdb_to_mesh function
        // let mesh = pdb_to_mesh(&large_pdb);

        // TODO: Assert that the mesh is created without errors and has expected properties
        // assert!(mesh.count_vertices() > 0);
        // assert!(mesh.indices().is_some());
    }
}
