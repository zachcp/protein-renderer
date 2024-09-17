//! A simple 3D scene with light shining over a cube sitting on a plane.
use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;
use pdbtbx::PDB;

/// Renderable
///

/// Enum representing various rendering options.
pub enum RenderOptions {
    Wireframe,
    Solid,
    Textured(String),
    LevelOfDetail(u32),
    Custom(Box<dyn Fn() -> bevy::prelude::Mesh>),
}

/// Renderable Train
pub trait Renderable {
    fn generate_mesh(&self, render_options: RenderOptions) -> bevy::prelude::Mesh;
}

impl Renderable for PDB {
    fn generate_mesh(&self, render_options: RenderOptions) -> bevy::prelude::Mesh {
        let mut positions = Vec::new();
        let mut indices = Vec::new();
        let resolution = 10;
        for atom in self.atoms() {
            let (x, y, z) = atom.pos();
            let (x, y, z) = (x as f32, y as f32, z as f32);
            let center = Vec3::new(x, y, z);
            let start_index = positions.len() as u32;
            let radius = atom
                .element()
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
        mesh.compute_smooth_normals();
        mesh
    }
}

// pub fn pdb_to_mesh<F>(pdb: &PDB, color_func: F) -> Mesh
// where
//     F: Fn(&pdbtbx::Atom) -> Color,
// {
//     let mut positions = Vec::new();
//     let mut indices = Vec::new();
//     let mut colors = Vec::new();
//     let resolution = 10; // Number of latitude and longitude divisions

//     for atom in pdb.atoms() {
//         // println!("{:?}", atom.element().expect("atom element").symbol());
//         let (x, y, z) = atom.pos();
//         let (x, y, z) = (x as f32, y as f32, z as f32);

//         let radius = atom
//             .element()
//             .expect("Atom Element not Defined")
//             .atomic_radius()
//             .van_der_waals
//             .expect("Van der waals not defined") as f32;
//         let center = Vec3::new(x, y, z);
//         let start_index = positions.len() as u32;
//         let color = color_func(atom).to_srgba();
//         let color_array: [f32; 4] = [color.red, color.green, color.blue, color.alpha];

//         // Generate sphere vertices
//         for i in 0..=resolution {
//             for j in 0..=resolution {
//                 let theta = i as f32 * std::f32::consts::PI / resolution as f32;
//                 let phi = j as f32 * 2.0 * std::f32::consts::PI / resolution as f32;

//                 let x = radius * theta.sin() * phi.cos();
//                 let y = radius * theta.sin() * phi.sin();
//                 let z = radius * theta.cos();

//                 positions.push((center + Vec3::new(x, y, z)).to_array());
//                 colors.push(color_array);
//             }
//         }

//         // Generate indices for triangles
//         for i in 0..resolution {
//             for j in 0..resolution {
//                 let top_left = start_index + i * (resolution + 1) + j;
//                 let top_right = top_left + 1;
//                 let bottom_left = top_left + resolution + 1;
//                 let bottom_right = bottom_left + 1;

//                 indices.extend_from_slice(&[
//                     top_left,
//                     bottom_left,
//                     top_right,
//                     top_right,
//                     bottom_left,
//                     bottom_right,
//                 ]);
//             }
//         }
//     }

//     let mut mesh = Mesh::new(
//         PrimitiveTopology::TriangleList,
//         RenderAssetUsages::default(),
//     );
//     mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
//     mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
//     mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));

//     // Calculate normals
//     mesh.compute_smooth_normals();
//     mesh
// }

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;
    use pdbtbx::StrictnessLevel;

    #[test]
    fn test_pdb_to_mesh() {
        let (mock_pdb, _errors) =
            pdbtbx::open("examples/1fap.cif", StrictnessLevel::Medium).unwrap();

        let color_func = |atom: &pdbtbx::Atom| {
            match atom.element().expect("expect atom").symbol() {
                "C" => Color::srgb(0.5, 0.5, 0.5), // Carbon: Gray
                "N" => Color::srgb(0.0, 0.0, 1.0), // Nitrogen: Blue
                "O" => Color::srgb(1.0, 0.0, 0.0), // Oxygen: Red
                "S" => Color::srgb(1.0, 1.0, 0.0), // Sulfur: Yellow
                _ => Color::srgb(1.0, 1.0, 1.0),   // Other: White
            }
        };

        let mesh = pdb_to_mesh(&mock_pdb, color_func);

        // TODO: Assert that the mesh has the expected number of vertices and indices
        assert_eq!(mesh.count_vertices(), 260634);
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
