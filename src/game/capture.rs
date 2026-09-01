use crate::data::{GameData, PlanetType};
use crate::state::Planet;

pub(super) fn gallery_planets(data: &GameData) -> Vec<Planet> {
    let specs = [
        (
            "Cinder Bloom",
            "Magma",
            112.0,
            0.65,
            0.06,
            0.9,
            1.4,
            0.0,
            "#B83B27",
        ),
        (
            "Pelagic Crown",
            "Water World",
            18.0,
            1.6,
            0.93,
            1.0,
            0.08,
            0.1,
            "#1D7CA6",
        ),
        (
            "Dust Reliquary",
            "Barren",
            -4.0,
            0.15,
            0.05,
            0.7,
            0.9,
            0.0,
            "#90714D",
        ),
        (
            "Verdant Relay",
            "Forested",
            22.0,
            1.1,
            0.64,
            1.0,
            0.18,
            1.0,
            "#50A95A",
        ),
        (
            "Frost Meridian",
            "Ice",
            -56.0,
            0.6,
            0.48,
            1.0,
            0.12,
            0.1,
            "#8FC9E8",
        ),
        (
            "Twilight Mosaic",
            "Mixed",
            38.0,
            0.85,
            0.38,
            1.2,
            0.42,
            0.38,
            "#7D8DD6",
        ),
    ];

    specs
        .into_iter()
        .enumerate()
        .map(
            |(
                index,
                (name, kind, temperature, atmosphere, water, gravity, radiation, biosphere, color),
            )| {
                Planet {
                    id: format!("gallery-{}", index + 1),
                    planet_type: gallery_type(data, kind, color),
                    name: name.to_owned(),
                    temperature,
                    atmosphere,
                    water,
                    gravity,
                    radiation,
                    biosphere,
                    purchase_price: 1_800 + index as i64 * 240,
                    invested_cost: 1_800 + index as i64 * 240,
                    color: color.to_owned(),
                }
            },
        )
        .collect()
}

fn gallery_type(data: &GameData, kind: &str, color: &str) -> PlanetType {
    let mut planet_type = data
        .planet_types
        .first()
        .cloned()
        .unwrap_or_else(|| PlanetType {
            name: kind.to_owned(),
            base_temp: 20.0,
            base_atmo: 1.0,
            base_water: 0.5,
            base_grav: 1.0,
            base_rad: 0.2,
            color: color.to_owned(),
        });
    planet_type.name = kind.to_owned();
    planet_type.color = color.to_owned();
    planet_type
}
