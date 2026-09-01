use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BiomeKind {
    Magma,
    Ocean,
    Barren,
    Forest,
    Ice,
    Mixed,
}

impl BiomeKind {
    fn label(self) -> &'static str {
        match self {
            Self::Magma => "MAGMA",
            Self::Ocean => "WATER WORLD",
            Self::Barren => "BARREN",
            Self::Forest => "FORESTED",
            Self::Ice => "ICEBOUND",
            Self::Mixed => "MIXED BIOSPHERE",
        }
    }

    fn accent(self) -> Color {
        match self {
            Self::Magma => Color::new(1.0, 0.43, 0.19, 1.0),
            Self::Ocean => Color::new(0.22, 0.84, 1.0, 1.0),
            Self::Barren => Color::new(0.88, 0.68, 0.38, 1.0),
            Self::Forest => Color::new(0.36, 1.0, 0.57, 1.0),
            Self::Ice => Color::new(0.70, 0.91, 1.0, 1.0),
            Self::Mixed => Color::new(0.78, 0.68, 1.0, 1.0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Palette {
    base: Color,
    deep: Color,
    water: Color,
    water_light: Color,
    land: Color,
    land_light: Color,
    hot: Color,
    cloud: Color,
}

#[derive(Debug, Clone, Copy)]
struct VisualRng {
    state: u64,
}

impl VisualRng {
    fn new(seed: u64) -> Self {
        Self {
            state: seed ^ 0xA5A5_5A5A_D3C1_9E37,
        }
    }

    fn next_u64(&mut self) -> u64 {
        self.state ^= self.state >> 12;
        self.state ^= self.state << 25;
        self.state ^= self.state >> 27;
        self.state.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn unit(&mut self) -> f32 {
        (self.next_u64() as u32 as f32) / u32::MAX as f32
    }

    fn between(&mut self, minimum: f32, maximum: f32) -> f32 {
        minimum + (maximum - minimum) * self.unit()
    }
}

pub(super) fn draw_planet_orb(planet: &Planet, center: Vec2, radius: f32) {
    let kind = biome_kind(planet);
    let palette = palette_for(kind, planet);
    let mut rng = VisualRng::new(planet_seed(planet));

    draw_planet_glow(center, radius, kind.accent());
    draw_circle(
        center.x + radius * 0.035,
        center.y + radius * 0.045,
        radius + 2.0,
        Color::new(0.008, 0.018, 0.03, 1.0),
    );
    draw_circle(center.x, center.y, (radius - 2.0).max(1.0), palette.base);

    draw_surface_features(kind, palette, center, radius, &mut rng);
    draw_planet_lighting(center, radius);
    draw_cloud_bands(planet, palette.cloud, center, radius, &mut rng);
    draw_atmosphere(planet, kind.accent(), center, radius);
}

fn biome_kind(planet: &Planet) -> BiomeKind {
    if planet.temperature >= 82.0 {
        BiomeKind::Magma
    } else if planet.temperature <= -20.0 && planet.water >= 0.28 {
        BiomeKind::Ice
    } else if planet.water >= 0.78 {
        BiomeKind::Ocean
    } else if planet.biosphere >= 0.65 && planet.water >= 0.34 && planet.atmosphere >= 0.45 {
        BiomeKind::Forest
    } else if planet.water <= 0.16 && planet.biosphere <= 0.2 {
        BiomeKind::Barren
    } else {
        BiomeKind::Mixed
    }
}

fn palette_for(kind: BiomeKind, planet: &Planet) -> Palette {
    let source = hex_to_color(&planet.color);
    match kind {
        BiomeKind::Magma => Palette {
            base: color(0.22, 0.035, 0.035),
            deep: color(0.10, 0.015, 0.025),
            water: color(0.36, 0.04, 0.025),
            water_light: color(0.70, 0.10, 0.025),
            land: color(0.34, 0.055, 0.04),
            land_light: color(0.58, 0.12, 0.035),
            hot: color(1.0, 0.48, 0.08),
            cloud: color(0.72, 0.72, 0.74),
        },
        BiomeKind::Ocean => Palette {
            base: color(0.025, 0.20, 0.35),
            deep: color(0.012, 0.075, 0.18),
            water: color(0.025, 0.31, 0.52),
            water_light: color(0.10, 0.66, 0.75),
            land: color(0.38, 0.48, 0.28),
            land_light: color(0.58, 0.70, 0.38),
            hot: color(0.92, 0.74, 0.34),
            cloud: color(0.82, 0.94, 1.0),
        },
        BiomeKind::Barren => Palette {
            base: color(0.40, 0.25, 0.15),
            deep: color(0.16, 0.08, 0.055),
            water: color(0.24, 0.16, 0.12),
            water_light: color(0.55, 0.34, 0.19),
            land: color(0.54, 0.37, 0.20),
            land_light: color(0.72, 0.52, 0.28),
            hot: color(0.94, 0.70, 0.38),
            cloud: color(0.74, 0.66, 0.56),
        },
        BiomeKind::Forest => Palette {
            base: color(0.025, 0.21, 0.27),
            deep: color(0.01, 0.075, 0.11),
            water: color(0.02, 0.31, 0.40),
            water_light: color(0.09, 0.61, 0.65),
            land: color(0.07, 0.35, 0.16),
            land_light: color(0.27, 0.70, 0.29),
            hot: color(0.72, 0.88, 0.48),
            cloud: color(0.84, 0.98, 0.94),
        },
        BiomeKind::Ice => Palette {
            base: color(0.37, 0.66, 0.82),
            deep: color(0.08, 0.23, 0.40),
            water: color(0.08, 0.35, 0.58),
            water_light: color(0.43, 0.82, 0.93),
            land: color(0.64, 0.82, 0.86),
            land_light: color(0.88, 0.96, 0.94),
            hot: color(0.78, 0.94, 1.0),
            cloud: color(0.94, 1.0, 1.0),
        },
        BiomeKind::Mixed => Palette {
            base: Color::new(source.r * 0.55, source.g * 0.55, source.b * 0.55, 1.0),
            deep: color(0.05, 0.07, 0.12),
            water: color(0.035, 0.27, 0.44),
            water_light: color(0.10, 0.62, 0.70),
            land: color(0.34, 0.36, 0.22),
            land_light: color(0.55, 0.64, 0.32),
            hot: color(0.86, 0.72, 0.40),
            cloud: color(0.82, 0.88, 0.92),
        },
    }
}

fn draw_planet_glow(center: Vec2, radius: f32, accent: Color) {
    for ring in (1..=7).rev() {
        let expansion = ring as f32 * radius * 0.035;
        draw_circle(
            center.x,
            center.y,
            radius + expansion,
            with_alpha(accent, 0.018 + (8 - ring) as f32 * 0.004),
        );
    }
}

fn draw_surface_features(
    kind: BiomeKind,
    palette: Palette,
    center: Vec2,
    radius: f32,
    rng: &mut VisualRng,
) {
    match kind {
        BiomeKind::Magma => draw_magma_surface(palette, center, radius, rng),
        BiomeKind::Ocean => draw_ocean_surface(palette, center, radius, rng),
        BiomeKind::Barren => draw_barren_surface(palette, center, radius, rng),
        BiomeKind::Forest => draw_forest_surface(palette, center, radius, rng),
        BiomeKind::Ice => draw_ice_surface(palette, center, radius, rng),
        BiomeKind::Mixed => draw_mixed_surface(palette, center, radius, rng),
    }
}

fn draw_magma_surface(palette: Palette, center: Vec2, radius: f32, rng: &mut VisualRng) {
    draw_blob(
        center + vec2(-radius * 0.10, -radius * 0.08),
        radius * 0.72,
        9,
        palette.deep,
        rng,
    );
    for _ in 0..4 {
        let offset = surface_offset(rng, radius * 0.40);
        draw_blob(
            center + offset,
            radius * rng.between(0.16, 0.28),
            7,
            palette.land,
            rng,
        );
    }
    for _ in 0..5 {
        let offset = surface_offset(rng, radius * 0.48);
        draw_blob(
            center + offset,
            radius * rng.between(0.035, 0.075),
            6,
            palette.land_light,
            rng,
        );
    }
    for index in 0..3 {
        let start = center
            + vec2(
                -radius * (0.55 - index as f32 * 0.08),
                radius * (0.25 + index as f32 * 0.14),
            );
        let end = start
            + vec2(
                radius * rng.between(0.35, 0.62),
                -radius * rng.between(0.25, 0.52),
            );
        draw_line(
            start.x,
            start.y,
            end.x,
            end.y,
            (radius * 0.035).max(1.0),
            palette.hot,
        );
    }
    draw_circle(
        center.x - radius * 0.25,
        center.y - radius * 0.26,
        radius * 0.11,
        with_alpha(palette.hot, 0.78),
    );
}

fn draw_ocean_surface(palette: Palette, center: Vec2, radius: f32, rng: &mut VisualRng) {
    draw_circle(center.x, center.y, radius * 0.86, palette.water);
    for _ in 0..5 {
        let offset = surface_offset(rng, radius * 0.40);
        draw_blob(
            center + offset,
            radius * rng.between(0.16, 0.30),
            8,
            palette.land,
            rng,
        );
        draw_blob(
            center + offset + vec2(-radius * 0.02, -radius * 0.02),
            radius * rng.between(0.08, 0.16),
            7,
            palette.land_light,
            rng,
        );
    }
    draw_latitude_lines(center, radius, palette.water_light, 3);
}

fn draw_barren_surface(palette: Palette, center: Vec2, radius: f32, rng: &mut VisualRng) {
    draw_blob(
        center + vec2(-radius * 0.10, -radius * 0.12),
        radius * 0.78,
        10,
        palette.land,
        rng,
    );
    for _ in 0..7 {
        let offset = surface_offset(rng, radius * 0.52);
        let crater = radius * rng.between(0.04, 0.105);
        draw_circle(
            center.x + offset.x,
            center.y + offset.y,
            crater,
            with_alpha(palette.deep, 0.75),
        );
        draw_circle(
            center.x + offset.x - crater * 0.18,
            center.y + offset.y - crater * 0.20,
            crater * 0.62,
            with_alpha(palette.water_light, 0.38),
        );
    }
    for _ in 0..5 {
        let offset = surface_offset(rng, radius * 0.54);
        draw_line(
            center.x + offset.x,
            center.y + offset.y,
            center.x + offset.x + radius * rng.between(0.08, 0.22),
            center.y + offset.y - radius * rng.between(0.015, 0.07),
            (radius * 0.018).max(0.7),
            with_alpha(palette.land_light, 0.48),
        );
    }
}

fn draw_forest_surface(palette: Palette, center: Vec2, radius: f32, rng: &mut VisualRng) {
    draw_circle(center.x, center.y, radius * 0.86, palette.water);
    for _ in 0..5 {
        let offset = surface_offset(rng, radius * 0.42);
        let land_radius = radius * rng.between(0.18, 0.31);
        draw_blob(center + offset, land_radius, 8, palette.land, rng);
        draw_blob(
            center + offset + vec2(-radius * 0.02, -radius * 0.025),
            land_radius * 0.72,
            8,
            palette.land_light,
            rng,
        );
        for _ in 0..7 {
            let tree = center + offset + surface_offset(rng, land_radius * 0.65);
            draw_circle(
                tree.x,
                tree.y,
                radius * rng.between(0.018, 0.038),
                with_alpha(palette.hot, 0.72),
            );
        }
    }
    draw_latitude_lines(center, radius, palette.water_light, 2);
}

fn draw_ice_surface(palette: Palette, center: Vec2, radius: f32, rng: &mut VisualRng) {
    draw_circle(center.x, center.y, radius * 0.86, palette.water);
    for _ in 0..3 {
        let offset = surface_offset(rng, radius * 0.38);
        draw_blob(
            center + offset,
            radius * rng.between(0.20, 0.32),
            8,
            palette.land,
            rng,
        );
    }
    draw_blob(
        center + vec2(-radius * 0.02, -radius * 0.48),
        radius * 0.42,
        9,
        palette.land_light,
        rng,
    );
    draw_blob(
        center + vec2(radius * 0.06, radius * 0.45),
        radius * 0.28,
        8,
        with_alpha(palette.land_light, 0.82),
        rng,
    );
    for index in 0..4 {
        let start = center
            + vec2(
                -radius * (0.36 - index as f32 * 0.11),
                -radius * 0.05 + index as f32 * radius * 0.08,
            );
        draw_line(
            start.x,
            start.y,
            start.x + radius * 0.20,
            start.y + radius * 0.16,
            (radius * 0.012).max(0.6),
            with_alpha(palette.hot, 0.64),
        );
    }
}

fn draw_mixed_surface(palette: Palette, center: Vec2, radius: f32, rng: &mut VisualRng) {
    draw_blob(
        center + vec2(radius * 0.03, radius * 0.02),
        radius * 0.80,
        10,
        palette.land,
        rng,
    );
    for _ in 0..4 {
        let offset = surface_offset(rng, radius * 0.46);
        draw_blob(
            center + offset,
            radius * rng.between(0.10, 0.22),
            8,
            palette.water,
            rng,
        );
        draw_blob(
            center + offset + vec2(-radius * 0.02, -radius * 0.02),
            radius * rng.between(0.04, 0.09),
            7,
            palette.water_light,
            rng,
        );
    }
    for _ in 0..5 {
        let offset = surface_offset(rng, radius * 0.50);
        draw_circle(
            center.x + offset.x,
            center.y + offset.y,
            radius * rng.between(0.018, 0.042),
            palette.land_light,
        );
    }
}

fn draw_blob(center: Vec2, radius: f32, sides: usize, fill: Color, rng: &mut VisualRng) {
    if radius <= 0.5 || sides < 3 {
        return;
    }
    let mut points = Vec::with_capacity(sides);
    for index in 0..sides {
        let angle = index as f32 / sides as f32 * std::f32::consts::TAU;
        let wobble = rng.between(0.76, 1.18);
        points.push(center + vec2(angle.cos(), angle.sin()) * radius * wobble);
    }
    for index in 1..points.len() - 1 {
        draw_triangle(points[0], points[index], points[index + 1], fill);
    }
}

fn draw_latitude_lines(center: Vec2, radius: f32, color: Color, count: usize) {
    for index in 0..count {
        let y = center.y - radius * 0.42 + index as f32 * radius * 0.42;
        let width = (radius * radius * 0.004).sqrt().max(radius * 0.22);
        draw_line(
            center.x - width,
            y,
            center.x + width,
            y,
            (radius * 0.012).max(0.6),
            with_alpha(color, 0.28),
        );
    }
}

fn draw_planet_lighting(center: Vec2, radius: f32) {
    draw_circle(
        center.x + radius * 0.20,
        center.y + radius * 0.18,
        radius * 0.72,
        Color::new(0.005, 0.015, 0.03, 0.22),
    );
    draw_circle(
        center.x - radius * 0.34,
        center.y - radius * 0.35,
        radius * 0.17,
        Color::new(1.0, 1.0, 1.0, 0.14),
    );
}

fn draw_cloud_bands(planet: &Planet, cloud: Color, center: Vec2, radius: f32, rng: &mut VisualRng) {
    let count = ((planet.atmosphere * 2.0).round() as usize).clamp(1, 5);
    for index in 0..count {
        let offset = surface_offset(rng, radius * 0.42);
        let cloud_center = center + offset + vec2(0.0, -radius * 0.08);
        let puff_radius = radius * rng.between(0.025, 0.055);
        let alpha = (0.10 + planet.atmosphere * 0.08).clamp(0.10, 0.34);
        draw_line(
            cloud_center.x - radius * 0.16,
            cloud_center.y,
            cloud_center.x + radius * 0.16,
            cloud_center.y,
            puff_radius * 1.6,
            with_alpha(cloud, alpha),
        );
        draw_circle(
            cloud_center.x,
            cloud_center.y,
            puff_radius,
            with_alpha(cloud, alpha + 0.04),
        );
        if index % 2 == 0 {
            draw_circle(
                cloud_center.x + radius * 0.11,
                cloud_center.y - puff_radius * 0.5,
                puff_radius * 0.72,
                with_alpha(cloud, alpha + 0.02),
            );
        }
    }
}

fn draw_atmosphere(planet: &Planet, accent: Color, center: Vec2, radius: f32) {
    let atmosphere = (0.24 + planet.atmosphere * 0.18 + planet.water * 0.08).clamp(0.24, 0.72);
    draw_circle_lines(
        center.x,
        center.y,
        radius + 1.5,
        (radius * 0.035).max(1.0),
        with_alpha(accent, atmosphere),
    );
    draw_circle_lines(
        center.x - radius * 0.015,
        center.y - radius * 0.01,
        radius - 5.0,
        (radius * 0.012).max(0.6),
        Color::new(1.0, 1.0, 1.0, 0.22),
    );
    if planet.atmosphere > 0.5 {
        draw_circle_lines(
            center.x - radius * 0.12,
            center.y - radius * 0.14,
            radius + 4.0,
            (radius * 0.012).max(0.6),
            with_alpha(Color::new(0.65, 0.86, 1.0, 1.0), 0.26),
        );
    }
}

fn surface_offset(rng: &mut VisualRng, maximum: f32) -> Vec2 {
    let angle = rng.between(0.0, std::f32::consts::TAU);
    let distance = maximum * rng.unit().sqrt();
    vec2(angle.cos() * distance, angle.sin() * distance)
}

fn planet_seed(planet: &Planet) -> u64 {
    let mut seed = 0xCBF2_9CE4_8422_2325_u64;
    for byte in planet
        .id
        .bytes()
        .chain(planet.name.bytes())
        .chain(planet.planet_type.name.bytes())
        .chain(planet.color.bytes())
    {
        seed ^= byte as u64;
        seed = seed.wrapping_mul(0x1000_0000_01B3);
    }
    for value in [
        planet.temperature,
        planet.atmosphere,
        planet.water,
        planet.gravity,
        planet.radiation,
        planet.biosphere,
    ] {
        seed ^= value.to_bits() as u64;
        seed = seed.rotate_left(11).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    }
    seed
}

fn color(red: f32, green: f32, blue: f32) -> Color {
    Color::new(red, green, blue, 1.0)
}

fn with_alpha(value: Color, alpha: f32) -> Color {
    Color::new(value.r, value.g, value.b, alpha.clamp(0.0, 1.0))
}

pub(super) fn draw_planet_gallery(ctx: &UiContext<'_>) {
    draw_gallery_background();
    draw_ui_text_ex(
        "PLANETARY VISUAL GENERATOR",
        34.0,
        32.0,
        TextStyle::new(22.0, Color::new(0.55, 0.88, 1.0, 1.0)).params(),
    );
    draw_ui_text_ex(
        "Stable seeds turn environmental stats into living surfaces: magma, oceans, deserts, forests, ice, and hybrid worlds.",
        36.0,
        57.0,
        TextStyle::new(12.0, Color::new(0.55, 0.70, 0.78, 1.0)).params(),
    );

    for (index, planet) in ctx.session.planets.iter().take(6).enumerate() {
        let column = index % 3;
        let row = index / 3;
        let rect = Rect::new(
            28.0 + column as f32 * 410.0,
            78.0 + row as f32 * 304.0,
            394.0,
            286.0,
        );
        draw_gallery_card(planet, rect);
    }
    draw_ui_text_ex(
        "GENERATOR CHECK  //  same planet + same stats = same surface  //  terraforming stats reshape future scans",
        36.0,
        706.0,
        TextStyle::new(11.0, Color::new(0.38, 0.68, 0.76, 1.0)).params(),
    );
}

fn draw_gallery_card(planet: &Planet, rect: Rect) {
    let kind = biome_kind(planet);
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.035, 0.08, 0.12, 0.98))
            .with_border(1.0, Color::new(0.16, 0.38, 0.46, 1.0))
            .with_left_accent(4.0, kind.accent()),
    );
    draw_planet_orb(planet, vec2(rect.x + 94.0, rect.y + 137.0), 82.0);
    draw_ui_text_ex(
        &planet.name,
        rect.x + 190.0,
        rect.y + 54.0,
        TextStyle::new(17.0, Color::new(0.88, 0.96, 1.0, 1.0)).params(),
    );
    draw_ui_text_ex(
        kind.label(),
        rect.x + 190.0,
        rect.y + 78.0,
        TextStyle::new(11.0, kind.accent()).params(),
    );
    draw_text_block(
        &format!(
            "{}°C  //  {:.1}x atmo\n{:.0}% water  //  {:.1} bio",
            planet.temperature,
            planet.atmosphere,
            planet.water * 100.0,
            planet.biosphere
        ),
        rect.x + 190.0,
        rect.y + 105.0,
        166.0,
        48.0,
        12.0,
        3.0,
        Color::new(0.60, 0.76, 0.82, 1.0),
    );
    draw_ui_text_ex(
        "PROCEDURAL SURFACE",
        rect.x + 190.0,
        rect.bottom() - 28.0,
        TextStyle::new(10.0, Color::new(0.43, 0.61, 0.68, 1.0)).params(),
    );
}

fn draw_gallery_background() {
    clear_background(Color::new(0.008, 0.02, 0.035, 1.0));
    for index in 0..90 {
        let value = index as f32;
        let x = (value * 137.71 + 23.0) % LOGICAL_WIDTH;
        let y = (value * 71.37 + (value * 0.73).sin() * 38.0 + 70.0) % LOGICAL_HEIGHT;
        let alpha = 0.12 + (value * 1.17).sin().abs() * 0.25;
        draw_circle(
            x,
            y,
            0.6 + (index % 3) as f32 * 0.35,
            Color::new(0.35, 0.76, 0.96, alpha),
        );
    }
    draw_rectangle(
        0.0,
        68.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT - 68.0,
        Color::new(0.01, 0.04, 0.065, 0.62),
    );
}

#[cfg(test)]
mod tests;
