//! Colors
//!
//! This moduel defines the color mapping used for rendering.
use bevy::prelude::{
    default, Assets, Color, ColorToComponents, Commands, Component, Mesh, MeshBuilder, Meshable,
    PbrBundle, ResMut, Sphere, StandardMaterial, Transform, Vec3,
};
use bon::Builder;
use pdbtbx::{Atom, Chain, Residue, PDB};

pub enum ColorScheme {
    Solid(Color),
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
