use super::*;

#[test]
fn embedded_planet_trader_data_loads() {
    let data = GameData::load().unwrap();

    assert_eq!(data.config.game_name, "planet_trader");
    assert_eq!(data.planet_types.len(), 15);
    assert_eq!(data.terraforming_tools.len(), 15);
    assert_eq!(data.alien_species_types.len(), 6);
    assert_eq!(data.planet_names.len(), 15);
    assert_eq!(data.research.len(), 10);
    assert!(data
        .alien_species
        .iter()
        .all(|species| species.bio_range[0] >= 0.0 && species.bio_range[1] <= 3.0));
    assert!(data
        .planet_types
        .iter()
        .all(|planet_type| !planet_type.color.is_empty()));
    assert!(data
        .terraforming_tools
        .iter()
        .all(|tool| !tool.id.is_empty()));
}

#[test]
fn tool_names_receive_stable_ids() {
    assert_eq!(slugify("Ice Comet Bombardment"), "ice-comet-bombardment");
    assert_eq!(slugify("Cryo-Atmos Converter"), "cryo-atmos-converter");
}
