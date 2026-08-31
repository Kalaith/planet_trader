//! Embedded Planet Trader content and configuration.

use macroquad_toolkit::data_loader::load_embedded_json_labeled;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const GAME_CONFIG_JSON: &str =
    macroquad_toolkit::include_json_str!("../assets/data/game_config.json");
const PLANET_TYPES_JSON: &str =
    macroquad_toolkit::include_json_str!("../assets/data/planet_types.json");
const TOOLS_JSON: &str =
    macroquad_toolkit::include_json_str!("../assets/data/terraforming_tools.json");
const SPECIES_JSON: &str =
    macroquad_toolkit::include_json_str!("../assets/data/alien_species.json");
const SPECIES_TYPES_JSON: &str =
    macroquad_toolkit::include_json_str!("../assets/data/alien_species_types.json");
const PLANET_NAMES_JSON: &str =
    macroquad_toolkit::include_json_str!("../assets/data/planet_names.json");
const RESEARCH_JSON: &str =
    macroquad_toolkit::include_json_str!("../assets/data/tool_research.json");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub game_name: String,
    pub display_name: String,
    pub save_slot: String,
    pub version: String,
    pub starting_credits: i64,
    pub buyer_refresh_seconds: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlanetType {
    pub name: String,
    #[serde(rename = "baseTemp")]
    pub base_temp: f32,
    #[serde(rename = "baseAtmo")]
    pub base_atmo: f32,
    #[serde(rename = "baseWater")]
    pub base_water: f32,
    #[serde(rename = "baseGrav")]
    pub base_grav: f32,
    #[serde(rename = "baseRad")]
    pub base_rad: f32,
    #[serde(default)]
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub category: String,
    pub effect: HashMap<String, f32>,
    #[serde(rename = "sideEffects", default)]
    pub side_effects: HashMap<String, f32>,
    pub cost: i64,
    #[serde(default)]
    pub tier: u32,
    #[serde(default = "default_true")]
    pub unlocked: bool,
    #[serde(rename = "upgradeRequired", default)]
    pub upgrade_required: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Species {
    pub name: String,
    pub description: String,
    #[serde(rename = "tempRange")]
    pub temp_range: [f32; 2],
    #[serde(rename = "atmoRange")]
    pub atmo_range: [f32; 2],
    #[serde(rename = "waterRange")]
    pub water_range: [f32; 2],
    #[serde(rename = "gravRange")]
    pub grav_range: [f32; 2],
    #[serde(rename = "radRange")]
    pub rad_range: [f32; 2],
    #[serde(rename = "basePrice")]
    pub base_price: i64,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeciesTemplate {
    pub prefixes: Vec<String>,
    pub suffixes: Vec<String>,
    pub desc: String,
    pub temp: [f32; 2],
    pub atmo: [f32; 2],
    pub water: [f32; 2],
    pub grav: [f32; 2],
    pub rad: [f32; 2],
    pub colors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchDef {
    pub name: String,
    pub category: String,
    #[serde(rename = "rpCost")]
    pub rp_cost: i64,
    #[serde(rename = "unlocksTool")]
    pub unlocks_tool: bool,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct GameData {
    pub config: GameConfig,
    pub planet_types: Vec<PlanetType>,
    pub terraforming_tools: Vec<Tool>,
    pub alien_species: Vec<Species>,
    pub alien_species_types: Vec<SpeciesTemplate>,
    pub planet_names: Vec<String>,
    pub research: Vec<ResearchDef>,
}

impl GameData {
    pub fn load() -> Result<Self, String> {
        let config = load_embedded_json_labeled("game_config", GAME_CONFIG_JSON)?;
        let mut planet_types: Vec<PlanetType> =
            load_embedded_json_labeled("planet_types", PLANET_TYPES_JSON)?;
        let mut terraforming_tools: Vec<Tool> =
            load_embedded_json_labeled("terraforming_tools", TOOLS_JSON)?;

        for (index, planet_type) in planet_types.iter_mut().enumerate() {
            if planet_type.color.trim().is_empty() {
                planet_type.color = planet_color(index);
            }
        }
        for tool in &mut terraforming_tools {
            if tool.id.trim().is_empty() {
                tool.id = slugify(&tool.name);
            }
        }

        Ok(Self {
            config,
            planet_types,
            terraforming_tools,
            alien_species: load_embedded_json_labeled("alien_species", SPECIES_JSON)?,
            alien_species_types: load_embedded_json_labeled(
                "alien_species_types",
                SPECIES_TYPES_JSON,
            )?,
            planet_names: load_embedded_json_labeled("planet_names", PLANET_NAMES_JSON)?,
            research: load_embedded_json_labeled("tool_research", RESEARCH_JSON)?,
        })
    }
}

fn default_true() -> bool {
    true
}

pub fn slugify(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn planet_color(index: usize) -> String {
    const COLORS: [&str; 15] = [
        "#8B7355", "#8EC5E8", "#C7503D", "#D69B3C", "#8E6BC4", "#7CB9A6", "#36B8C7", "#E46B2A",
        "#7D8DD6", "#A7C7E7", "#709C3A", "#D69A36", "#777B83", "#50A95A", "#D2C13D",
    ];
    COLORS[index % COLORS.len()].to_owned()
}

#[cfg(test)]
mod tests;
