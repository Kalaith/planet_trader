# Planet Trader Concept Art Direction

This folder is a non-runtime reference library for future environments, buyer
dossiers, events, character animation, and technology-deployment art. The
production game assets remain in `assets/art/`; these images are intentionally
excluded from `asset_registry.json` and published packages.

## Visual language

- Dark graphite frontier infrastructure with sparse amber practical lights.
- Cyan holography communicates brokerage, analysis, and precise control.
- Alien and research disciplines own a restrained accent: ember volcanology,
  ice-blue cryogenics, teal hydrology, sand-gold atmospherics, iron-red
  high-gravity industry, violet radiation, and green ecology.
- Machinery should look civilian, modular, repairable, and expensive. Avoid
  military silhouettes and pristine white interiors.
- Planetary operations must show cause and effect at believable scale rather
  than reading as weapons or magic.

## Environment anchors

- `frontier_brokerage_station.png`: exterior silhouette and module hierarchy.
- `alien_exchange_concourse.png`: physical setting behind the alien market.
- `frontier_acquisition_observatory.png`: contract-scanning environment.
- `terraforming_workshop_bay.png`: full-scale engineering cradle.
- `xenoscience_research_lab.png`: research progression made physical.

## Species anchors

Each species has one homeland/deal scene and one three-view turnaround:

- Pyrothane Lizards: `pyrothane_exchange.png`, `pyrothane_turnaround.png`.
- Cryophyte Crystals: `cryophyte_conclave.png`, `cryophyte_turnaround.png`.
- Aquatic Molluscoids: `aquatic_collective_market.png`,
  `aquatic_molluscoid_turnaround.png`.
- Desert Nomads: `desert_nomad_caravan_exchange.png`,
  `desert_nomad_turnaround.png`.
- High-Gravity Hunters: `high_gravity_foundry_hall.png`,
  `high_gravity_hunter_turnaround.png`.
- Energy Feeders: `energy_feeder_magnetosphere.png`,
  `energy_feeder_turnaround.png`.

`frontier_broker_turnaround.png` defines the player's civilian silhouette, and
`broker_shuttle_turnaround.png` defines their compact survey courier.

## Technology anchors

- `hydrology_deployment.png`: ice delivery and cloud formation.
- `thermal_balancer_deployment.png`: controlled heat transfer.
- `cryo_atmos_converter_deployment.png`: gas extraction and atmosphere growth.
- `harsh_world_field_array.png`: gravity and magnetic shielding systems.
- `ecology_seeding_deployment.png`: Bio-Seeder Pods and Radiant Bloom.
- `precision_climate_grid_deployment.png`: exact late-game climate control.

All images were generated with the built-in image generation workflow using the
provided UI references and the implemented sprite atlases as visual anchors.
Prompts consistently requested polished cinematic 2D hard-sci-fi concept art,
grounded industrial realism, no readable text, no logos, and no watermark.
