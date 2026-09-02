use super::*;

#[test]
fn authored_species_map_to_all_six_portraits() {
    let mappings = [
        alien_index("Pyrothane Lizards", "volcanology"),
        alien_index("Cryophyte Crystals", "harsh-world"),
        alien_index("Aquatic Molluscoids", "hydrology"),
        alien_index("Desert Nomads", "atmospherics"),
        alien_index("High-Gravity Hunters", "harsh-world"),
        alien_index("Energy Feeders", "ecology"),
    ];
    assert_eq!(mappings, [0, 1, 2, 3, 4, 5]);
}

#[test]
fn every_research_node_has_an_atlas_cell() {
    for name in [
        "Atmospheric Chemistry",
        "Planetary Systems Modeling",
        "Precision Climate Grid",
        "Ice Comet Bombardment",
        "Cloud Seeder",
        "Thermal Balancer",
        "Cryo-Atmos Converter",
        "Gravity Intensifier",
        "Gravity Stabilizer",
        "Magnetic Field Generator",
        "Radiation Enhancer",
        "Bio-Seeder Pods",
        "Radiant Bloom",
    ] {
        assert!(technology_index(name) < 12, "missing atlas cell for {name}");
    }
}
