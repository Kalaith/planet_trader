use super::*;

#[cfg(test)]
#[path = "market/tests.rs"]
mod tests;

#[cfg(test)]
fn market_refresh_label(game_started: bool, elapsed: f32, refresh_seconds: f32) -> String {
    if !game_started {
        "Market offline".to_owned()
    } else if refresh_seconds <= 0.0 {
        "No refresh".to_owned()
    } else {
        format!(
            "Refresh in {:.0}s",
            (refresh_seconds - elapsed).max(0.0).ceil()
        )
    }
}

const WORLD: Rect = Rect::new(36.0, 140.0, 330.0, 548.0);
const BUYERS: Rect = Rect::new(390.0, 140.0, 450.0, 548.0);
const DEAL: Rect = Rect::new(866.0, 140.0, 374.0, 548.0);

pub(super) fn draw_market(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_market_floor();
    draw_world(ctx, mouse, actions);
    draw_buyers(ctx, mouse, actions);
    draw_deal(ctx, mouse, actions);
}

fn draw_market_floor() {
    draw_rectangle(
        18.0,
        120.0,
        1244.0,
        586.0,
        Color::new(0.013, 0.034, 0.050, 1.0),
    );
    draw_rectangle(
        18.0,
        120.0,
        354.0,
        586.0,
        Color::new(0.018, 0.058, 0.073, 0.95),
    );
    draw_rectangle(
        852.0,
        120.0,
        410.0,
        586.0,
        Color::new(0.030, 0.060, 0.071, 0.97),
    );
    draw_line(
        374.0,
        142.0,
        374.0,
        684.0,
        1.0,
        Color::new(0.18, 0.52, 0.58, 0.24),
    );
    draw_line(
        852.0,
        142.0,
        852.0,
        684.0,
        1.0,
        Color::new(0.18, 0.52, 0.58, 0.24),
    );
}

fn draw_world(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_ui_text_ex(
        "WORLD ON THE BLOCK",
        WORLD.x,
        164.0,
        TextStyle::new(13.0, Color::new(0.42, 0.82, 0.94, 1.0)).params(),
    );
    let Some(planet) = ctx.session.current_planet() else {
        draw_ui_text_ex(
            "NO ACTIVE WORLD",
            WORLD.x,
            252.0,
            TextStyle::new(22.0, dark::TEXT_DIM).params(),
        );
        draw_text_block(
            "Select a world in the Workshop before opening negotiations.",
            WORLD.x,
            275.0,
            WORLD.w,
            70.0,
            12.0,
            4.0,
            dark::TEXT_DIM,
        );
        if button(
            Rect::new(WORLD.x, 410.0, 230.0, 46.0),
            "OPEN WORKSHOP",
            true,
            ButtonTone::Primary,
            mouse,
        ) {
            actions.push(UiAction::SetMode(GameplayMode::Workshop));
        }
        return;
    };
    draw_ui_text_ex(
        &planet.name,
        WORLD.x,
        201.0,
        TextStyle::new(24.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        &planet.planet_type.name,
        WORLD.x + 2.0,
        224.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );
    let center = vec2(WORLD.center().x, 365.0);
    for radius in [142.0, 153.0] {
        draw_circle_lines(
            center.x,
            center.y,
            radius,
            1.0,
            Color::new(0.24, 0.70, 0.76, 0.14),
        );
    }
    draw_planet_orb(planet, center, 124.0);
    draw_ui_text_ex(
        "ENVIRONMENTAL PROFILE",
        WORLD.x,
        518.0,
        TextStyle::new(8.0, dark::TEXT_DIM).params(),
    );
    let stats = [
        ("TEMP", format!("{:.0} C", planet.temperature)),
        ("ATMO", format!("{:.2}", planet.atmosphere)),
        ("WATER", format!("{:.0}%", planet.water * 100.0)),
        ("GRAV", format!("{:.2} G", planet.gravity)),
        ("RAD", format!("{:.2}", planet.radiation)),
        ("BIO", format!("{:.1}", planet.biosphere)),
    ];
    for (index, (label, value)) in stats.iter().enumerate() {
        let column = index % 2;
        let row = index / 2;
        let x = WORLD.x + column as f32 * 166.0;
        let y = 547.0 + row as f32 * 38.0;
        draw_ui_text_ex(label, x, y, TextStyle::new(8.0, dark::TEXT_DIM).params());
        draw_text_right(value, x + 144.0, y, TextStyle::new(11.0, dark::TEXT_BRIGHT));
        draw_line(
            x,
            y + 10.0,
            x + 146.0,
            y + 10.0,
            1.0,
            Color::new(0.18, 0.40, 0.44, 0.30),
        );
    }
    draw_ui_text_ex(
        "TOTAL INVESTMENT",
        WORLD.x,
        673.0,
        TextStyle::new(8.0, dark::TEXT_DIM).params(),
    );
    draw_text_right(
        &format!("{} CR", planet.invested_cost),
        WORLD.right(),
        673.0,
        TextStyle::new(12.0, Color::new(1.0, 0.75, 0.38, 1.0)),
    );
}

fn draw_buyers(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_ui_text_ex(
        "LIVE NEGOTIATIONS",
        BUYERS.x,
        164.0,
        TextStyle::new(13.0, Color::new(0.96, 0.70, 0.34, 1.0)).params(),
    );
    draw_ui_text_ex(
        "Choose the strongest route",
        BUYERS.x,
        198.0,
        TextStyle::new(22.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        "Tap a buyer signal to compare all six requirements.",
        BUYERS.x,
        220.0,
        TextStyle::new(9.0, dark::TEXT_DIM).params(),
    );
    let Some(planet) = ctx.session.current_planet() else {
        return;
    };
    let best_id = ctx
        .session
        .alien_buyers
        .iter()
        .max_by_key(|buyer| sale_price(planet, buyer))
        .map(|buyer| buyer.id);
    for (index, buyer) in ctx.session.alien_buyers.iter().enumerate() {
        let score = compatibility(planet, buyer);
        let matches = compatibility_matches(score);
        let price = sale_price(planet, buyer);
        let selected = ctx.expanded_buyer == Some(buyer.id);
        let row = Rect::new(BUYERS.x, 240.0 + index as f32 * 102.0, BUYERS.w, 92.0);
        if selected {
            draw_rectangle(
                row.x,
                row.y,
                row.w,
                row.h,
                Color::new(0.07, 0.22, 0.27, 0.94),
            );
        }
        let accent = hex_to_color(&buyer.color);
        draw_circle(
            row.x + 12.0,
            row.y + 20.0,
            10.0,
            Color::new(accent.r, accent.g, accent.b, 0.18),
        );
        draw_circle(row.x + 12.0, row.y + 20.0, 5.0, accent);
        draw_ui_text_ex(
            &buyer.name,
            row.x + 34.0,
            row.y + 24.0,
            TextStyle::new(14.0, dark::TEXT_BRIGHT).params(),
        );
        if best_id == Some(buyer.id) {
            draw_text_right(
                "BEST PAYOUT",
                row.right() - 8.0,
                row.y + 21.0,
                TextStyle::new(8.0, Color::new(0.48, 1.0, 0.64, 1.0)),
            );
        }
        draw_ui_text_ex(
            &format!("{}/6 MATCH", matches),
            row.x + 34.0,
            row.y + 51.0,
            TextStyle::new(
                12.0,
                if matches >= 4 {
                    Color::new(0.46, 0.94, 0.62, 1.0)
                } else {
                    Color::new(1.0, 0.72, 0.36, 1.0)
                },
            )
            .params(),
        );
        draw_text_right(
            &format!("{} CR  /  {:+.0}%", price, market_trend_percent(buyer)),
            row.right() - 8.0,
            row.y + 51.0,
            TextStyle::new(11.0, dark::TEXT_BRIGHT),
        );
        draw_ui_text_ex(
            &format!(
                "{} RP  +{} {} knowledge",
                projected_research_points(price, score),
                knowledge_award_for_matches(matches),
                buyer.expertise
            ),
            row.x + 34.0,
            row.y + 75.0,
            TextStyle::new(9.0, dark::TEXT_DIM).params(),
        );
        draw_line(
            row.x,
            row.bottom(),
            row.right(),
            row.bottom(),
            1.0,
            if selected {
                Color::new(0.34, 0.82, 0.90, 0.72)
            } else {
                Color::new(0.18, 0.39, 0.43, 0.32)
            },
        );
        if row.contains_point(mouse) && is_mouse_button_released(MouseButton::Left) {
            actions.push(UiAction::ToggleBuyer(buyer.id));
        }
    }
}

fn draw_deal(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_ui_text_ex(
        "DEAL CONSOLE",
        DEAL.x,
        164.0,
        TextStyle::new(13.0, Color::new(0.42, 0.82, 0.94, 1.0)).params(),
    );
    let Some(planet) = ctx.session.current_planet() else {
        return;
    };
    let selected = ctx
        .expanded_buyer
        .and_then(|id| ctx.session.alien_buyers.iter().find(|buyer| buyer.id == id));
    let Some(buyer) = selected else {
        draw_ui_text_ex(
            "AWAITING BUYER",
            DEAL.x,
            222.0,
            TextStyle::new(21.0, dark::TEXT_DIM).params(),
        );
        draw_text_block("Select a live negotiation. Four matching requirements can close; stronger fits earn more research and expertise.", DEAL.x, 244.0, DEAL.w, 100.0, 12.0, 4.0, dark::TEXT_DIM);
        if button(
            Rect::new(DEAL.x, 640.0, DEAL.w, 40.0),
            "RETURN TO WORKSHOP",
            true,
            ButtonTone::Muted,
            mouse,
        ) {
            actions.push(UiAction::SetMode(GameplayMode::Workshop));
        }
        return;
    };
    draw_ui_text_ex(
        &buyer.name,
        DEAL.x,
        201.0,
        TextStyle::new(22.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        &format!("{} expertise", buyer.expertise),
        DEAL.x,
        224.0,
        TextStyle::new(9.0, dark::TEXT_DIM).params(),
    );
    let requirements = [
        ("TEMPERATURE", planet.temperature, buyer.temp_range, "C"),
        ("ATMOSPHERE", planet.atmosphere, buyer.atmo_range, "ATM"),
        (
            "WATER",
            planet.water * 100.0,
            [buyer.water_range[0] * 100.0, buyer.water_range[1] * 100.0],
            "%",
        ),
        ("GRAVITY", planet.gravity, buyer.grav_range, "G"),
        ("RADIATION", planet.radiation, buyer.rad_range, ""),
        ("BIOSPHERE", planet.biosphere, buyer.bio_range, ""),
    ];
    for (index, (label, value, range, unit)) in requirements.iter().enumerate() {
        let matched = *value >= range[0] && *value <= range[1];
        let y = 260.0 + index as f32 * 46.0;
        let color = if matched {
            Color::new(0.48, 1.0, 0.64, 1.0)
        } else {
            Color::new(1.0, 0.52, 0.48, 1.0)
        };
        draw_circle(DEAL.x + 5.0, y - 4.0, 4.0, color);
        draw_ui_text_ex(
            label,
            DEAL.x + 18.0,
            y,
            TextStyle::new(10.0, color).params(),
        );
        draw_text_right(
            &format!(
                "{:.1}{}  /  {:.1}-{:.1}{}",
                value, unit, range[0], range[1], unit
            ),
            DEAL.right(),
            y,
            TextStyle::new(9.0, dark::TEXT_DIM),
        );
        draw_line(
            DEAL.x + 18.0,
            y + 11.0,
            DEAL.right(),
            y + 11.0,
            1.0,
            Color::new(0.18, 0.38, 0.42, 0.28),
        );
    }
    let score = compatibility(planet, buyer);
    let matches = compatibility_matches(score);
    let price = sale_price(planet, buyer);
    draw_ui_text_ex(
        "PROPOSED SETTLEMENT",
        DEAL.x,
        548.0,
        TextStyle::new(8.0, dark::TEXT_DIM).params(),
    );
    draw_ui_text_ex(
        &format!("{} CR", price),
        DEAL.x,
        583.0,
        TextStyle::new(27.0, dark::TEXT_BRIGHT).params(),
    );
    draw_text_right(
        &format!(
            "{}/6  /  {:+} CR margin",
            matches,
            potential_profit(planet, buyer)
        ),
        DEAL.right(),
        582.0,
        TextStyle::new(
            11.0,
            if matches >= 4 {
                Color::new(0.46, 0.94, 0.62, 1.0)
            } else {
                Color::new(1.0, 0.72, 0.36, 1.0)
            },
        ),
    );
    if button(
        Rect::new(DEAL.x, 600.0, DEAL.w, 48.0),
        if matches >= 4 {
            "CLOSE DEAL"
        } else {
            "4 OF 6 REQUIRED"
        },
        matches >= 4,
        ButtonTone::Positive,
        mouse,
    ) {
        actions.push(UiAction::SellPlanet(buyer.id));
    }
    if button(
        Rect::new(DEAL.x, 654.0, DEAL.w, 32.0),
        "HOLD / RETURN TO WORKSHOP",
        true,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::SetMode(GameplayMode::Workshop));
    }
}
