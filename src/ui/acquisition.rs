use super::*;

const SCAN_BUTTON: Rect = Rect::new(214.0, 613.0, 456.0, 62.0);

pub(super) fn draw_acquisition(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_brokerage_floor();
    draw_scan_bay(ctx);
    draw_market_signals(ctx);
    draw_scan_console(ctx, mouse, actions);
}

fn draw_brokerage_floor() {
    let area = Rect::new(18.0, 120.0, 1244.0, 586.0);
    draw_rectangle(
        area.x,
        area.y,
        area.w,
        area.h,
        Color::new(0.012, 0.035, 0.057, 1.0),
    );
    draw_rectangle(
        area.x,
        area.y,
        840.0,
        area.h,
        Color::new(0.008, 0.046, 0.074, 0.86),
    );
    draw_rectangle(
        858.0,
        area.y,
        404.0,
        area.h,
        Color::new(0.026, 0.054, 0.074, 0.94),
    );
    for index in 0..54 {
        let value = index as f32;
        let x = 38.0 + (value * 137.71 + 17.0) % 790.0;
        let y = 142.0 + (value * 73.37 + (value * 0.81).sin() * 29.0) % 390.0;
        let alpha = 0.10 + (value * 1.31).sin().abs() * 0.22;
        draw_circle(
            x,
            y,
            0.7 + (index % 3) as f32 * 0.3,
            Color::new(0.38, 0.82, 1.0, alpha),
        );
    }
    draw_line(
        858.0,
        142.0,
        858.0,
        681.0,
        1.0,
        Color::new(0.24, 0.62, 0.72, 0.24),
    );
    draw_line(
        866.0,
        161.0,
        866.0,
        662.0,
        3.0,
        Color::new(0.20, 0.76, 0.86, 0.08),
    );
}

fn draw_scan_bay(ctx: &UiContext<'_>) {
    draw_ui_text_ex(
        "FRONTIER CONTRACT SCANNER",
        52.0,
        165.0,
        TextStyle::new(13.0, Color::new(0.36, 0.78, 0.90, 1.0)).params(),
    );
    draw_ui_text_ex(
        "Find your next world",
        50.0,
        199.0,
        TextStyle::new(27.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        "Scan the frontier against live alien demand.",
        52.0,
        222.0,
        TextStyle::new(11.0, dark::TEXT_DIM).params(),
    );

    let center = vec2(442.0, 390.0);
    draw_projector_field(center);
    draw_holographic_planet(center, ctx.settings.reduced_motion);

    draw_ui_text_ex(
        "DEEP-RANGE ARRAY",
        685.0,
        289.0,
        TextStyle::new(9.0, Color::new(0.32, 0.65, 0.72, 1.0)).params(),
    );
    draw_ui_text_ex(
        &format!(
            "{} SIGNALS IN REACH",
            contract_option_count(ctx.session.reputation)
        ),
        685.0,
        309.0,
        TextStyle::new(12.0, Color::new(0.60, 0.91, 1.0, 1.0)).params(),
    );
    draw_line(
        685.0,
        319.0,
        798.0,
        319.0,
        1.0,
        Color::new(0.28, 0.70, 0.80, 0.30),
    );
    draw_ui_text_ex(
        "LIVE MARKET LINK",
        685.0,
        346.0,
        TextStyle::new(9.0, Color::new(0.32, 0.65, 0.72, 1.0)).params(),
    );
    draw_ui_text_ex(
        "BUYER FILTER READY",
        685.0,
        366.0,
        TextStyle::new(12.0, Color::new(0.42, 1.0, 0.68, 1.0)).params(),
    );
}

fn draw_projector_field(center: Vec2) {
    for ring in (1..=6).rev() {
        draw_circle(
            center.x,
            center.y + 10.0,
            178.0 + ring as f32 * 12.0,
            Color::new(0.03, 0.42, 0.56, 0.012 + (7 - ring) as f32 * 0.006),
        );
    }
    draw_orbit(
        center,
        226.0,
        82.0,
        -0.13,
        Color::new(0.24, 0.80, 0.94, 0.28),
    );
    draw_orbit(
        center,
        190.0,
        128.0,
        0.38,
        Color::new(0.32, 0.93, 1.0, 0.18),
    );
    draw_orbit(
        center,
        242.0,
        48.0,
        0.16,
        Color::new(0.18, 0.60, 0.72, 0.20),
    );
    draw_line(
        center.x,
        225.0,
        center.x,
        556.0,
        1.0,
        Color::new(0.28, 0.83, 0.92, 0.14),
    );
    draw_line(
        263.0,
        center.y,
        619.0,
        center.y,
        1.0,
        Color::new(0.28, 0.83, 0.92, 0.12),
    );
    for y in [262.0, 326.0, 454.0, 518.0] {
        draw_line(405.0, y, 479.0, y, 1.0, Color::new(0.35, 0.92, 1.0, 0.22));
    }
}

fn draw_holographic_planet(center: Vec2, reduced_motion: bool) {
    let phase = if reduced_motion {
        0.0
    } else {
        get_time() as f32 * 0.55
    };
    let radius = 145.0 + phase.sin() * 3.0;
    for ring in (1..=7).rev() {
        draw_circle(
            center.x,
            center.y,
            radius + ring as f32 * 5.0,
            Color::new(0.08, 0.70, 0.86, 0.016 + (8 - ring) as f32 * 0.006),
        );
    }
    draw_circle(
        center.x,
        center.y,
        radius,
        Color::new(0.025, 0.24, 0.31, 0.96),
    );
    draw_circle(
        center.x - 20.0,
        center.y - 18.0,
        radius * 0.86,
        Color::new(0.04, 0.38, 0.44, 0.78),
    );
    draw_circle(
        center.x + 42.0,
        center.y + 34.0,
        radius * 0.72,
        Color::new(0.005, 0.055, 0.09, 0.60),
    );
    draw_hologram_land(
        center + vec2(-49.0, -38.0),
        52.0,
        Color::new(0.20, 0.86, 0.74, 0.54),
    );
    draw_hologram_land(
        center + vec2(44.0, 18.0),
        61.0,
        Color::new(0.14, 0.70, 0.66, 0.44),
    );
    draw_hologram_land(
        center + vec2(-16.0, 72.0),
        34.0,
        Color::new(0.23, 0.91, 0.77, 0.34),
    );
    for offset in [-76.0, -38.0, 0.0, 38.0, 76.0] {
        let half_width = (radius * radius - offset * offset).max(0.0).sqrt() * 0.90;
        draw_line(
            center.x - half_width,
            center.y + offset,
            center.x + half_width,
            center.y + offset,
            1.0,
            Color::new(0.48, 0.96, 1.0, 0.16),
        );
    }
    draw_circle_lines(
        center.x,
        center.y,
        radius,
        2.0,
        Color::new(0.48, 0.96, 1.0, 0.76),
    );
    draw_circle_lines(
        center.x - 7.0,
        center.y - 5.0,
        radius - 8.0,
        1.0,
        Color::new(0.70, 1.0, 0.92, 0.28),
    );
    let scan_y = center.y - radius + ((phase * 0.7).sin() * 0.5 + 0.5) * radius * 2.0;
    let half_width = (radius * radius - (scan_y - center.y).powi(2))
        .max(0.0)
        .sqrt();
    draw_line(
        center.x - half_width,
        scan_y,
        center.x + half_width,
        scan_y,
        3.0,
        Color::new(0.55, 1.0, 0.86, 0.42),
    );
}

fn draw_hologram_land(center: Vec2, radius: f32, color: Color) {
    draw_circle(center.x, center.y, radius, color);
    draw_circle(
        center.x + radius * 0.55,
        center.y - radius * 0.18,
        radius * 0.58,
        color,
    );
    draw_circle(
        center.x - radius * 0.42,
        center.y + radius * 0.36,
        radius * 0.46,
        color,
    );
}

fn draw_orbit(center: Vec2, width: f32, height: f32, tilt: f32, color: Color) {
    let segments = 72;
    let mut previous = center + rotate_point(vec2(width, 0.0), tilt);
    for index in 1..=segments {
        let angle = index as f32 / segments as f32 * std::f32::consts::TAU;
        let current = center + rotate_point(vec2(angle.cos() * width, angle.sin() * height), tilt);
        draw_line(previous.x, previous.y, current.x, current.y, 1.4, color);
        previous = current;
    }
}

fn rotate_point(point: Vec2, angle: f32) -> Vec2 {
    vec2(
        point.x * angle.cos() - point.y * angle.sin(),
        point.x * angle.sin() + point.y * angle.cos(),
    )
}

fn draw_market_signals(ctx: &UiContext<'_>) {
    draw_ui_text_ex(
        "MARKET PULSE",
        900.0,
        164.0,
        TextStyle::new(13.0, Color::new(0.96, 0.70, 0.34, 1.0)).params(),
    );
    draw_ui_text_ex(
        "What buyers want now",
        898.0,
        196.0,
        TextStyle::new(22.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        "Strongest offers shape the contract scan.",
        900.0,
        219.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );
    let mut buyers: Vec<_> = ctx.session.alien_buyers.iter().collect();
    buyers.sort_by_key(|buyer| std::cmp::Reverse(buyer.current_price));
    for (index, buyer) in buyers.into_iter().take(3).enumerate() {
        let y = 260.0 + index as f32 * 91.0;
        let accent = hex_to_color(&buyer.color);
        draw_circle(
            909.0,
            y - 4.0,
            11.0,
            Color::new(accent.r, accent.g, accent.b, 0.14),
        );
        draw_circle(909.0, y - 4.0, 5.0, accent);
        draw_ui_text_ex(
            &buyer.name,
            932.0,
            y,
            TextStyle::new(14.0, dark::TEXT_BRIGHT).params(),
        );
        draw_text_right(
            &format!("{} CR", buyer.current_price),
            1230.0,
            y,
            TextStyle::new(13.0, Color::new(0.50, 1.0, 0.66, 1.0)),
        );
        draw_ui_text_ex(
            &short_demand(&buyer.description),
            932.0,
            y + 23.0,
            TextStyle::new(10.0, dark::TEXT_DIM).params(),
        );
        draw_line(
            900.0,
            y + 52.0,
            1230.0,
            y + 52.0,
            1.0,
            Color::new(0.22, 0.42, 0.48, 0.30),
        );
    }
    draw_ui_text_ex(
        "AVAILABLE CAPITAL",
        900.0,
        562.0,
        TextStyle::new(10.0, Color::new(0.42, 0.67, 0.72, 1.0)).params(),
    );
    draw_ui_text_ex(
        &format!("{} CR", ctx.session.credits),
        898.0,
        603.0,
        TextStyle::new(31.0, dark::TEXT_BRIGHT).params(),
    );
    draw_line(
        900.0,
        620.0,
        1230.0,
        620.0,
        1.0,
        Color::new(0.24, 0.55, 0.60, 0.34),
    );
    draw_ui_text_ex(
        "SCAN FEE",
        900.0,
        649.0,
        TextStyle::new(10.0, Color::new(0.42, 0.67, 0.72, 1.0)).params(),
    );
    draw_text_right(
        "FREE",
        1230.0,
        649.0,
        TextStyle::new(15.0, Color::new(0.48, 1.0, 0.68, 1.0)),
    );
    draw_ui_text_ex(
        "Credits commit only when you acquire a world.",
        900.0,
        676.0,
        TextStyle::new(9.0, dark::TEXT_DIM).params(),
    );
}

fn short_demand(description: &str) -> String {
    let first = description
        .split(['.', ';'])
        .next()
        .unwrap_or(description)
        .trim();
    let mut words = first.split_whitespace();
    let mut result = String::new();
    for _ in 0..7 {
        let Some(word) = words.next() else { break };
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(word);
    }
    if words.next().is_some() {
        result.push_str("...");
    }
    result
}

fn draw_scan_console(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let top_left = vec2(124.0, 548.0);
    let top_right = vec2(758.0, 548.0);
    let bottom_left = vec2(78.0, 702.0);
    let bottom_right = vec2(804.0, 702.0);
    let surface = Color::new(0.025, 0.095, 0.125, 0.97);
    draw_triangle(top_left, top_right, bottom_left, surface);
    draw_triangle(top_right, bottom_right, bottom_left, surface);
    draw_line(
        top_left.x,
        top_left.y,
        top_right.x,
        top_right.y,
        2.0,
        Color::new(0.33, 0.85, 0.92, 0.52),
    );
    draw_line(
        bottom_left.x,
        bottom_left.y,
        bottom_right.x,
        bottom_right.y,
        2.0,
        Color::new(0.08, 0.31, 0.39, 0.85),
    );
    draw_line(
        124.0,
        562.0,
        758.0,
        562.0,
        1.0,
        Color::new(0.18, 0.55, 0.62, 0.20),
    );
    draw_ui_text_ex(
        "ARRAY ONLINE",
        122.0,
        591.0,
        TextStyle::new(9.0, Color::new(0.40, 0.96, 0.68, 1.0)).params(),
    );
    draw_ui_text_ex(
        &format!(
            "{} CONTRACTS",
            contract_option_count(ctx.session.reputation)
        ),
        704.0,
        591.0,
        TextStyle::new(9.0, Color::new(0.45, 0.76, 0.82, 1.0)).params(),
    );
    if button(
        SCAN_BUTTON,
        "SCAN FOR PLANET CONTRACTS",
        ctx.session.game_started,
        ButtonTone::Primary,
        mouse,
    ) {
        actions.push(UiAction::OpenPurchase);
    }
}
