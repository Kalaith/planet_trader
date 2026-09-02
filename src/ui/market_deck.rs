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

const WORLD: Rect = Rect::new(18.0, 150.0, 350.0, 540.0);
const BUYERS: Rect = Rect::new(384.0, 150.0, 500.0, 540.0);
const DEAL: Rect = Rect::new(900.0, 150.0, 362.0, 540.0);

pub(super) fn draw_market(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_world(ctx, mouse, actions);
    draw_buyers(ctx, mouse, actions);
    draw_deal(ctx, mouse, actions);
}

fn draw_world(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_panel(WORLD, "WORLD FOR SALE");
    let Some(planet) = ctx.session.current_planet() else {
        draw_text_centered_in_box(
            "NO ACTIVE WORLD",
            WORLD.x + 30.0,
            WORLD.y + 170.0,
            WORLD.w - 60.0,
            38.0,
            21.0,
            dark::TEXT_DIM,
        );
        draw_text_centered_in_box(
            "Select a world in the Workshop before approaching buyers.",
            WORLD.x + 48.0,
            WORLD.y + 220.0,
            WORLD.w - 96.0,
            64.0,
            13.0,
            dark::TEXT_DIM,
        );
        if button(
            Rect::new(WORLD.x + 52.0, WORLD.bottom() - 70.0, WORLD.w - 104.0, 44.0),
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
        WORLD.x + 18.0,
        WORLD.y + 76.0,
        TextStyle::new(19.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        &planet.planet_type.name,
        WORLD.x + 20.0,
        WORLD.y + 98.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );
    draw_planet_orb(planet, vec2(WORLD.center().x, WORLD.y + 230.0), 112.0);
    let stats = [
        ("T", format!("{:.0} C", planet.temperature)),
        ("A", format!("{:.2} ATM", planet.atmosphere)),
        ("W", format!("{:.0}%", planet.water * 100.0)),
        ("G", format!("{:.2} G", planet.gravity)),
        ("R", format!("{:.2}", planet.radiation)),
        ("B", format!("{:.1}", planet.biosphere)),
    ];
    for (index, (label, value)) in stats.iter().enumerate() {
        let column = index % 2;
        let row = index / 2;
        let card = Rect::new(
            WORLD.x + 14.0 + column as f32 * 162.0,
            WORLD.y + 358.0 + row as f32 * 42.0,
            154.0,
            34.0,
        );
        draw_surface(
            card,
            &SurfaceStyle::new(Color::new(0.06, 0.12, 0.16, 1.0))
                .with_border(1.0, Color::new(0.16, 0.32, 0.39, 1.0)),
        );
        draw_ui_text_ex(
            label,
            card.x + 8.0,
            card.y + 22.0,
            TextStyle::new(10.0, dark::TEXT_DIM).params(),
        );
        draw_text_right(
            value,
            card.right() - 8.0,
            card.y + 22.0,
            TextStyle::new(12.0, dark::TEXT_BRIGHT),
        );
    }
    draw_ui_text_ex(
        &format!("Investment  {} CR", planet.invested_cost),
        WORLD.x + 18.0,
        WORLD.bottom() - 26.0,
        TextStyle::new(11.0, Color::new(1.0, 0.75, 0.38, 1.0)).params(),
    );
}

fn draw_buyers(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_panel(BUYERS, "LIVE BUYER OFFERS");
    draw_ui_text_ex(
        "Tap a buyer to compare all six requirements.",
        BUYERS.x + 14.0,
        BUYERS.y + 61.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
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
        let card = Rect::new(
            BUYERS.x + 12.0,
            BUYERS.y + 76.0 + index as f32 * 108.0,
            BUYERS.w - 24.0,
            98.0,
        );
        draw_surface(
            card,
            &SurfaceStyle::new(if selected {
                Color::new(0.08, 0.23, 0.31, 1.0)
            } else {
                Color::new(0.065, 0.12, 0.16, 1.0)
            })
            .with_border(
                1.0,
                if selected {
                    Color::new(0.32, 0.82, 1.0, 1.0)
                } else {
                    Color::new(0.18, 0.36, 0.44, 1.0)
                },
            ),
        );
        draw_circle(
            card.x + 18.0,
            card.y + 21.0,
            8.0,
            hex_to_color(&buyer.color),
        );
        draw_ui_text_ex(
            &buyer.name,
            card.x + 34.0,
            card.y + 25.0,
            TextStyle::new(14.0, dark::TEXT_BRIGHT).params(),
        );
        if best_id == Some(buyer.id) {
            draw_text_right(
                "BEST PAYOUT",
                card.right() - 12.0,
                card.y + 22.0,
                TextStyle::new(9.0, Color::new(0.48, 1.0, 0.64, 1.0)),
            );
        }
        draw_ui_text_ex(
            &format!(
                "{} / 6 MATCH  //  {} CR  //  {:+.0}% trend",
                matches,
                price,
                market_trend_percent(buyer)
            ),
            card.x + 14.0,
            card.y + 51.0,
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
        draw_ui_text_ex(
            &format!(
                "Rewards: {} RP  //  +{} {} knowledge",
                projected_research_points(price, score),
                knowledge_award_for_matches(matches),
                buyer.expertise
            ),
            card.x + 14.0,
            card.y + 76.0,
            TextStyle::new(10.0, dark::TEXT_DIM).params(),
        );
        if card.contains_point(mouse) && is_mouse_button_released(MouseButton::Left) {
            actions.push(UiAction::ToggleBuyer(buyer.id));
        }
    }
}

fn draw_deal(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_panel(DEAL, "DEAL ANALYSIS");
    let Some(planet) = ctx.session.current_planet() else {
        return;
    };
    let selected = ctx
        .expanded_buyer
        .and_then(|id| ctx.session.alien_buyers.iter().find(|buyer| buyer.id == id));
    let Some(buyer) = selected else {
        draw_text_block("Select a live buyer offer. A deal becomes sellable at four matching requirements; five and six matches earn better science and expertise.", DEAL.x + 18.0, DEAL.y + 76.0, DEAL.w - 36.0, 112.0, 13.0, 5.0, dark::TEXT_DIM);
        if button(
            Rect::new(DEAL.x + 18.0, DEAL.bottom() - 62.0, DEAL.w - 36.0, 44.0),
            "HOLD / RETURN TO WORKSHOP",
            true,
            ButtonTone::Muted,
            mouse,
        ) {
            actions.push(UiAction::SetMode(GameplayMode::Workshop));
        }
        return;
    };
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
    draw_ui_text_ex(
        &buyer.name,
        DEAL.x + 16.0,
        DEAL.y + 72.0,
        TextStyle::new(18.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        &format!("{} expertise", buyer.expertise),
        DEAL.x + 18.0,
        DEAL.y + 93.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );
    for (index, (label, value, range, unit)) in requirements.iter().enumerate() {
        let matched = *value >= range[0] && *value <= range[1];
        let row = Rect::new(
            DEAL.x + 14.0,
            DEAL.y + 112.0 + index as f32 * 45.0,
            DEAL.w - 28.0,
            37.0,
        );
        draw_surface(
            row,
            &SurfaceStyle::new(if matched {
                Color::new(0.06, 0.20, 0.14, 1.0)
            } else {
                Color::new(0.18, 0.10, 0.10, 1.0)
            })
            .with_border(
                1.0,
                if matched {
                    Color::new(0.25, 0.65, 0.40, 1.0)
                } else {
                    Color::new(0.62, 0.25, 0.25, 1.0)
                },
            ),
        );
        draw_ui_text_ex(
            &format!("{}  {}", if matched { "OK" } else { "MISS" }, label),
            row.x + 9.0,
            row.y + 15.0,
            TextStyle::new(
                10.0,
                if matched {
                    Color::new(0.48, 1.0, 0.64, 1.0)
                } else {
                    Color::new(1.0, 0.52, 0.48, 1.0)
                },
            )
            .params(),
        );
        draw_text_right(
            &format!(
                "{:.1}{}  target {:.1}-{:.1}{}",
                value, unit, range[0], range[1], unit
            ),
            row.right() - 9.0,
            row.y + 29.0,
            TextStyle::new(9.0, dark::TEXT_DIM),
        );
    }
    let score = compatibility(planet, buyer);
    let matches = compatibility_matches(score);
    let price = sale_price(planet, buyer);
    draw_ui_text_ex(
        &format!(
            "{} / 6  //  {} CR  //  {:+} CR margin",
            matches,
            price,
            potential_profit(planet, buyer)
        ),
        DEAL.x + 16.0,
        DEAL.y + 406.0,
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
    if button(
        Rect::new(DEAL.x + 14.0, DEAL.bottom() - 106.0, DEAL.w - 28.0, 48.0),
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
        Rect::new(DEAL.x + 14.0, DEAL.bottom() - 48.0, DEAL.w - 28.0, 34.0),
        "HOLD / RETURN TO WORKSHOP",
        true,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::SetMode(GameplayMode::Workshop));
    }
}
