//! Embedded sprite atlases and stable mappings from game data to artwork.

use macroquad::prelude::*;

const ALIEN_ATLAS_BYTES: &[u8] = include_bytes!("../assets/art/alien_species_atlas.png");
const TECHNOLOGY_ATLAS_BYTES: &[u8] = include_bytes!("../assets/art/technology_icons_atlas.png");

pub struct Artwork {
    alien_atlas: Texture2D,
    technology_atlas: Texture2D,
}

impl Artwork {
    pub fn load() -> Self {
        let alien_atlas =
            Texture2D::from_file_with_format(ALIEN_ATLAS_BYTES, Some(ImageFormat::Png));
        let technology_atlas =
            Texture2D::from_file_with_format(TECHNOLOGY_ATLAS_BYTES, Some(ImageFormat::Png));
        alien_atlas.set_filter(FilterMode::Linear);
        technology_atlas.set_filter(FilterMode::Linear);
        Self {
            alien_atlas,
            technology_atlas,
        }
    }

    pub fn draw_alien_portrait(
        &self,
        name: &str,
        expertise: &str,
        center: Vec2,
        height: f32,
        tint: Color,
    ) {
        let index = alien_index(name, expertise) as f32;
        let columns = 3.0;
        let rows = 2.0;
        let cell_w = self.alien_atlas.width() / columns;
        let cell_h = self.alien_atlas.height() / rows;
        let width = height * cell_w / cell_h;
        draw_texture_ex(
            &self.alien_atlas,
            center.x - width * 0.5,
            center.y - height * 0.5,
            tint,
            DrawTextureParams {
                dest_size: Some(vec2(width, height)),
                source: Some(Rect::new(
                    (index % columns) * cell_w,
                    (index / columns).floor() * cell_h,
                    cell_w,
                    cell_h,
                )),
                ..Default::default()
            },
        );
    }

    pub fn draw_technology_icon(
        &self,
        research_name: &str,
        center: Vec2,
        height: f32,
        tint: Color,
    ) {
        let index = technology_index(research_name) as f32;
        let columns = 4.0;
        let rows = 3.0;
        let cell_w = self.technology_atlas.width() / columns;
        let cell_h = self.technology_atlas.height() / rows;
        let width = height * cell_w / cell_h;
        draw_texture_ex(
            &self.technology_atlas,
            center.x - width * 0.5,
            center.y - height * 0.5,
            tint,
            DrawTextureParams {
                dest_size: Some(vec2(width, height)),
                source: Some(Rect::new(
                    (index % columns) * cell_w,
                    (index / columns).floor() * cell_h,
                    cell_w,
                    cell_h,
                )),
                ..Default::default()
            },
        );
    }
}

fn alien_index(name: &str, expertise: &str) -> usize {
    let name = name.to_ascii_lowercase();
    if name.contains("pyro")
        || name.contains("inferno")
        || name.contains("magma")
        || name.contains("blaze")
    {
        0
    } else if name.contains("cryo")
        || name.contains("frost")
        || name.contains("glacial")
        || name.contains("arctic")
        || name.contains("polar")
    {
        1
    } else if expertise == "hydrology" {
        2
    } else if expertise == "atmospherics" {
        3
    } else if expertise == "harsh-world" {
        4
    } else if expertise == "ecology" {
        5
    } else {
        0
    }
}

fn technology_index(name: &str) -> usize {
    match name {
        "Atmospheric Chemistry" => 0,
        "Planetary Systems Modeling" => 1,
        "Precision Climate Grid" => 2,
        "Ice Comet Bombardment" => 3,
        "Cloud Seeder" => 4,
        "Thermal Balancer" => 5,
        "Cryo-Atmos Converter" => 6,
        "Gravity Intensifier" => 7,
        "Gravity Stabilizer" => 8,
        "Magnetic Field Generator" => 9,
        "Radiation Enhancer" => 10,
        "Bio-Seeder Pods" | "Radiant Bloom" => 11,
        _ => 1,
    }
}

#[cfg(test)]
#[path = "artwork/tests.rs"]
mod tests;
