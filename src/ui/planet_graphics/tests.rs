use super::*;

#[test]
fn visual_rng_preserves_legacy_integer_and_float_streams() {
    for seed in [0, 1, u64::MAX, 0xA5A5_5A5A_D3C1_9E37, 0xDEAD_BEEF] {
        let mut state = seed ^ 0xA5A5_5A5A_D3C1_9E37;
        let mut integer = VisualRng::new(seed);
        let mut unit = VisualRng::new(seed);
        let mut range = VisualRng::new(seed);
        for _ in 0..128 {
            state ^= state >> 12;
            state ^= state << 25;
            state ^= state >> 27;
            let expected = state.wrapping_mul(0x2545_F491_4F6C_DD1D);
            let expected_unit = (expected as u32 as f32) / u32::MAX as f32;
            assert_eq!(integer.next_u64(), expected);
            assert_eq!(unit.unit().to_bits(), expected_unit.to_bits());
            assert_eq!(
                range.between(-4.0, 12.0).to_bits(),
                (-4.0 + 16.0 * expected_unit).to_bits()
            );
        }
    }
}

fn sample_planet(
    id: &str,
    temperature: f32,
    atmosphere: f32,
    water: f32,
    radiation: f32,
    biosphere: f32,
) -> Planet {
    Planet {
        id: id.to_owned(),
        planet_type: crate::data::PlanetType {
            name: "Test World".to_owned(),
            base_temp: temperature,
            base_atmo: atmosphere,
            base_water: water,
            base_grav: 1.0,
            base_rad: radiation,
            color: "#50A95A".to_owned(),
        },
        name: "Test World".to_owned(),
        temperature,
        atmosphere,
        water,
        gravity: 1.0,
        radiation,
        biosphere,
        purchase_price: 1_000,
        invested_cost: 1_000,
        color: "#50A95A".to_owned(),
    }
}

#[test]
fn environmental_profiles_cover_the_gallery_biomes() {
    let cases = [
        (
            sample_planet("magma", 100.0, 0.7, 0.1, 1.0, 0.0),
            BiomeKind::Magma,
        ),
        (
            sample_planet("ocean", 18.0, 1.0, 0.9, 0.1, 0.1),
            BiomeKind::Ocean,
        ),
        (
            sample_planet("barren", 18.0, 0.1, 0.05, 0.8, 0.0),
            BiomeKind::Barren,
        ),
        (
            sample_planet("forest", 22.0, 1.0, 0.6, 0.1, 1.0),
            BiomeKind::Forest,
        ),
        (
            sample_planet("ice", -40.0, 0.6, 0.5, 0.1, 0.1),
            BiomeKind::Ice,
        ),
        (
            sample_planet("mixed", 38.0, 0.8, 0.4, 0.4, 0.35),
            BiomeKind::Mixed,
        ),
    ];

    for (planet, expected) in cases {
        assert_eq!(biome_kind(&planet), expected);
    }
}

#[test]
fn planet_seed_is_stable_and_reacts_to_environmental_changes() {
    let planet = sample_planet("seeded", 22.0, 1.0, 0.6, 0.1, 1.0);
    let mut changed = planet.clone();
    changed.water = 0.8;

    assert_eq!(planet_seed(&planet), planet_seed(&planet));
    assert_ne!(planet_seed(&planet), planet_seed(&changed));
}
