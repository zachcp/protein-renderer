//! Renderable
//!
//! Trait and Implementations for Generating Representations.
//!
//!

// use bevy::prelude::*;
use super::ColorScheme;
use bevy::asset::Assets;
use bevy::prelude::{
    default, info, Color, ColorToComponents, Component, Cylinder, Mesh, MeshBuilder, Meshable,
    PbrBundle, Sphere, StandardMaterial, Vec3,
};
use bon::Builder;
use glam::{EulerRot, Quat};
use pdbtbx::PDB;

/// Enum representing various rendering options.
///
/// Each of these enums represents a rendering path that can be used by a `Structure`
///
/// Donw the Line: allow passing an arbitrary function that maps PDB to mesh.
///
#[derive(Clone)]
pub enum RenderOptions {
    Wireframe,
    Cartoon,
    BallAndStick,
    Solid,
}

// https://github.com/nglviewer/ngl/blob/master/src/representation/ballandstick-representation.ts
// https://github.com/molstar/molstar/blob/master/src/mol-repr/structure/representation/ball-and-stick.ts
// sphere_detail: u32,
// radial_segments: u32,
// open_ended: bool,
// disable_impostor: bool,
// aspect_ratio: f32,
// line_only: bool,
// line_width: f32,
// cylinder_only: bool,
// multiple_bond: MultipleBondType,
// bond_spacing: f32,
// bond_scale: f32,
// linewidth: f32,
pub struct BallandStickOpts {
    linewidth: f32,
    molsphere_size: f32,
}
impl Default for BallandStickOpts {
    fn default() -> Self {
        Self {
            linewidth: 1.0,
            molsphere_size: 0.25,
        }
    }
}

pub enum MultipleBondType {
    Off,
    Symmetric,
    Offset,
}

/// Define Everything Needed to render
/// We want to add the
#[derive(Builder, Component)]
pub struct Structure {
    pdb: PDB,
    #[builder(default = RenderOptions::Solid)]
    rendertype: RenderOptions,
    #[builder(default = ColorScheme::Solid(Color::WHITE))]
    color_scheme: ColorScheme,
    #[builder(default = StandardMaterial::default())]
    material: StandardMaterial,
    #[builder(default = BallandStickOpts::default())]
    opts_ballandstick: BallandStickOpts,
}

impl Structure {
    pub fn to_mesh(&self) -> Mesh {
        match &self.rendertype {
            RenderOptions::Wireframe => self.render_wireframe(),
            RenderOptions::Cartoon => self.render_cartoon(),
            RenderOptions::BallAndStick => self.render_ballandstick(),
            RenderOptions::Solid => self.render_spheres(),
        }
    }
    // this is the onw we probably want
    pub fn to_pbr(
        &self,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> PbrBundle {
        let mesh = self.to_mesh();
        let material = self.material.clone();
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(material),
            // transform: Transform::from_xyz(x, y, z),
            ..default()
        }
    }
    fn render_wireframe(&self) -> Mesh {
        todo!()
    }
    fn render_cartoon(&self) -> Mesh {
        todo!()
    }
    fn render_ballandstick(&self) -> Mesh {
        let mut meshes = Vec::new();

        println!("Number of bonds: {}", self.pdb.bonds().count());
        let bonds: Vec<_> = self.pdb.bonds().collect();
        info!("Number of bonds: {}", bonds.len());
        println!("Structure summary:");
        println!("  Number of models: {}", self.pdb.models().count());
        println!("  Number of chains: {}", self.pdb.chains().count());
        println!("  Number of residues: {}", self.pdb.residues().count());
        println!("  Number of atoms: {}", self.pdb.atoms().count());

        // Render spheres (atoms)
        for atom in self.pdb.atoms() {
            let (x, y, z) = atom.pos();
            let center = Vec3::new(x as f32, y as f32, z as f32);
            let radius = self.opts_ballandstick.molsphere_size;
            let color = self.color_scheme.get_color(atom).to_srgba();
            let mut sphere_mesh = Sphere::new(radius).mesh().build();
            let vertex_count = sphere_mesh.count_vertices();
            let color_array = vec![color.to_vec4(); vertex_count];
            sphere_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, color_array);
            sphere_mesh = sphere_mesh.translated_by(center);
            sphere_mesh.compute_smooth_normals();
            meshes.push(sphere_mesh);
        }

        // Render lines (bonds)
        for (atom1, atom2, bond) in self.pdb.bonds() {
            println!("Bonds: {:?}, {:?}, {:?}", atom1, atom2, bond);
            let (x1, y1, z1) = atom1.pos();
            let (x2, y2, z2) = atom2.pos();
            let start = Vec3::new(x1 as f32, y1 as f32, z1 as f32);
            let end = Vec3::new(x2 as f32, y2 as f32, z2 as f32);
            let direction = end - start;
            // let normalized_direction = direction.normalize();
            let length = direction.length();
            let cylinder = Cylinder::new(self.opts_ballandstick.linewidth, length);

            let mut cylinder_mesh = cylinder.mesh().build();

            cylinder_mesh = cylinder_mesh.translated_by(start);
            let color = self.color_scheme.get_color(atom1).to_srgba(); // You might want to choose a color scheme for bonds
            let vertex_count = cylinder_mesh.count_vertices();
            // let color_array = vec![color.to_vec4(); vertex_count];
            let color_array = vec![[0.5, 0.5, 0.5, 1.0]; vertex_count];
            cylinder_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, color_array);
            cylinder_mesh.compute_smooth_normals();
            meshes.push(cylinder_mesh);
        }

        // Combine all meshes
        meshes
            .into_iter()
            .reduce(|mut acc, mesh| {
                acc.merge(&mesh);
                acc
            })
            .unwrap()
    }
    /// Internal fn for rendering spheres.
    fn render_spheres(&self) -> Mesh {
        let mut meshes = Vec::new();

        println!("Number of bonds: {}", self.pdb.bonds().count());
        let bonds: Vec<_> = self.pdb.bonds().collect();
        info!("Number of bonds: {}", bonds.len());
        println!("Structure summary:");
        println!("  Number of models: {}", self.pdb.models().count());
        println!("  Number of chains: {}", self.pdb.chains().count());
        println!("  Number of residues: {}", self.pdb.residues().count());
        println!("  Number of atoms: {}", self.pdb.atoms().count());

        for atom in self.pdb.atoms() {
            let (x, y, z) = atom.pos();
            let center = Vec3::new(x as f32, y as f32, z as f32);
            let radius = atom
                .element()
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
        let mesh = structure.to_mesh();
        assert_eq!(mesh.count_vertices(), 779748);
    }
}
