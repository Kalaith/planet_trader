use super::*;

pub(super) fn draw_market_panel(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_panel(MARKET_PANEL, "Alien Market");
    draw_ui_text_ex(
        "Tap a buyer to see requirements.",
        MARKET_PANEL.x + 14.0,
        MARKET_PANEL.y + 58.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );
    draw_text_right(
        &market_refresh_label(
            ctx.session.game_started,
            ctx.market_elapsed,
            ctx.data.config.buyer_refresh_seconds,
        ),
        MARKET_PANEL.right() - 14.0,
        MARKET_PANEL.y + 58.0,
        TextStyle::new(10.0, Color::new(0.42, 0.72, 0.80, 1.0)),
    );
    let max_scroll = max_market_scroll(ctx);
    if button(
        Rect::new(
            MARKET_PANEL.right() - 64.0,
            MARKET_PANEL.y + 12.0,
            22.0,
            24.0,
        ),
        "UP",
        ctx.market_scroll > 0.0,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::ScrollMarket(-1));
    }
    if button(
        Rect::new(
            MARKET_PANEL.right() - 38.0,
            MARKET_PANEL.y + 12.0,
            22.0,
            24.0,
        ),
        "DN",
        ctx.market_scroll < max_scroll,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::ScrollMarket(1));
    }

    if ctx.session.current_planet().is_none() {
        draw_text_block(
            "Select a planet to compare its environment with the buyers.",
            MARKET_PANEL.x + 14.0,
            MARKET_PANEL.y + 78.0,
            MARKET_PANEL.w - 28.0,
            50.0,
            12.0,
            2.0,
            Color::new(0.45, 0.65, 0.72, 1.0),
        );
    }

    let list = Rect::new(
        MARKET_PANEL.x + 12.0,
        MARKET_PANEL.y + 132.0,
        MARKET_PANEL.w - 24.0,
        MARKET_PANEL.h - 144.0,
    );
    let mut y = list.y - ctx.market_scroll.min(max_scroll);
    for buyer in &ctx.session.alien_buyers {
        let expanded = ctx.expanded_buyer == Some(buyer.id);
        let height = if expanded { 220.0 } else { 102.0 };
        let rect = Rect::new(list.x, y, list.w, height);
        if rect.bottom() >= list.y && rect.y <= list.bottom() {
            draw_buyer_card(ctx, buyer, rect, mouse, actions);
        }
        y += height + 8.0;
    }
    draw_scroll_hint(list, ctx.market_scroll < max_scroll);
}

fn market_refresh_label(game_started: bool, elapsed: f32, refresh_seconds: f32) -> String {
    if !game_started {
        return "Market offline".to_owned();
    }
    if refresh_seconds <= 0.0 {
        return "No refresh".to_owned();
    }

    let remaining = (refresh_seconds - elapsed.max(0.0)).ceil().max(0.0) as i32;
    format!("Refresh in {}s", remaining)
}

fn draw_buyer_card(
    ctx: &UiContext<'_>,
    buyer: &AlienBuyer,
    rect: Rect,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let planet = ctx.session.current_planet();
    let score = planet.map(|planet| compatibility(planet, buyer));
    let hovered = rect.contains_point(mouse);
    draw_surface(
        rect,
        &SurfaceStyle::new(if hovered {
            Color::new(0.11, 0.19, 0.24, 1.0)
        } else {
            Color::new(0.075, 0.13, 0.18, 1.0)
        })
        .with_border(1.0, Color::new(0.23, 0.39, 0.46, 1.0)),
    );
    draw_circle(
        rect.x + 22.0,
        rect.y + 23.0,
        10.0,
        hex_to_color(&buyer.color),
    );
    draw_ui_text_ex(
        &buyer.name,
        rect.x + 40.0,
        rect.y + 24.0,
        TextStyle::new(14.0, dark::TEXT).params(),
    );
    let market_trend = market_trend_percent(buyer);
    draw_text_right(
        &format!(
            "{} CR {}{:.0}%",
            buyer.current_price,
            if market_trend >= 0.0 { "UP " } else { "DN " },
            market_trend.abs()
        ),
        rect.right() - 12.0,
        rect.y + 24.0,
        TextStyle::new(
            13.0,
            if market_trend >= 0.0 {
                Color::new(0.50, 0.96, 0.62, 1.0)
            } else {
                Color::new(1.0, 0.72, 0.34, 1.0)
            },
        ),
    );

    if let Some(planet) = planet {
        let score = score.expect("score is present with a planet");
        let estimated_sale = sale_price(planet, buyer);
        let profit = potential_profit(planet, buyer);
        draw_ui_text_ex(
            &format!("Compatibility: {:.0}%", score * 100.0),
            rect.x + 12.0,
            rect.y + 47.0,
            TextStyle::new(
                12.0,
                if score >= 0.8 {
                    Color::new(0.46, 1.0, 0.60, 1.0)
                } else if score >= 0.6 {
                    Color::new(1.0, 0.78, 0.30, 1.0)
                } else {
                    Color::new(1.0, 0.42, 0.42, 1.0)
                },
            )
            .params(),
        );
        draw_requirement_icons(planet, buyer, rect.x + 12.0, rect.y + 58.0);
        let estimate_y = if ctx.expanded_buyer == Some(buyer.id) {
            rect.bottom() - 6.0
        } else {
            rect.y + 91.0
        };
        draw_ui_text_ex(
            &format!(
                "Est. sale {} CR | {}{} CR",
                estimated_sale,
                if profit >= 0 { "+" } else { "" },
                profit
            ),
            rect.x + 12.0,
            estimate_y,
            TextStyle::new(
                10.0,
                if profit >= 0 {
                    Color::new(0.48, 0.94, 0.62, 1.0)
                } else {
                    Color::new(1.0, 0.46, 0.42, 1.0)
                },
            )
            .params(),
        );
        let sell_enabled = score >= 0.6;
        if button(
            Rect::new(rect.right() - 76.0, rect.y + 42.0, 62.0, 32.0),
            "SELL",
            sell_enabled,
            ButtonTone::Positive,
            mouse,
        ) {
            actions.push(UiAction::SellPlanet(buyer.id));
        }
    } else {
        draw_ui_text_ex(
            "Select a planet to compare",
            rect.x + 12.0,
            rect.y + 48.0,
            TextStyle::new(12.0, dark::TEXT_DIM).params(),
        );
        draw_ui_text_ex(
            "Requirements available on tap",
            rect.x + 12.0,
            rect.y + 70.0,
            TextStyle::new(11.0, Color::new(0.42, 0.64, 0.72, 1.0)).params(),
        );
    }

    if hovered && is_mouse_button_released(MouseButton::Left) {
        let sell_rect = Rect::new(rect.right() - 76.0, rect.y + 42.0, 62.0, 32.0);
        if !sell_rect.contains_point(mouse) {
            actions.push(UiAction::ToggleBuyer(buyer.id));
        }
    }

    if ctx.expanded_buyer == Some(buyer.id) {
        draw_text_block(
            &buyer.description,
            rect.x + 12.0,
            rect.y + 86.0,
            rect.w - 24.0,
            34.0,
            11.0,
            2.0,
            dark::TEXT_DIM,
        );
        if let Some(planet) = planet {
            draw_requirement_grid(planet, buyer, rect);
        } else {
            draw_range_grid(buyer, rect);
        }
    }
}

fn draw_requirement_icons(planet: &Planet, buyer: &AlienBuyer, x: f32, y: f32) {
    let values = [
        (planet.temperature, buyer.temp_range, "T"),
        (planet.atmosphere, buyer.atmo_range, "A"),
        (planet.water, buyer.water_range, "W"),
        (planet.gravity, buyer.grav_range, "G"),
        (planet.radiation, buyer.rad_range, "R"),
    ];
    for (index, (value, range, icon)) in values.iter().enumerate() {
        let met = *value >= range[0] && *value <= range[1];
        draw_ui_text_ex(
            &format!("{} {}", icon, if met { "OK" } else { "NO" }),
            x + index as f32 * 34.0,
            y + 12.0,
            TextStyle::new(
                13.0,
                if met {
                    Color::new(0.42, 1.0, 0.56, 1.0)
                } else {
                    Color::new(1.0, 0.40, 0.40, 1.0)
                },
            )
            .params(),
        );
    }
}

fn draw_requirement_grid(planet: &Planet, buyer: &AlienBuyer, rect: Rect) {
    let requirements = [
        ("Temp", buyer.temp_range, planet.temperature),
        ("Atmo", buyer.atmo_range, planet.atmosphere),
        ("Water", buyer.water_range, planet.water),
        ("Grav", buyer.grav_range, planet.gravity),
        ("Rad", buyer.rad_range, planet.radiation),
    ];
    for (index, (label, range, value)) in requirements.iter().enumerate() {
        let x = rect.x + 12.0 + (index % 2) as f32 * (rect.w * 0.5 - 10.0);
        let y = rect.y + 124.0 + (index / 2) as f32 * 30.0;
        let cell = Rect::new(x, y, rect.w * 0.5 - 16.0, 24.0);
        let met = *value >= range[0] && *value <= range[1];
        draw_surface(
            cell,
            &SurfaceStyle::new(if met {
                Color::new(0.08, 0.28, 0.18, 1.0)
            } else {
                Color::new(0.28, 0.10, 0.12, 1.0)
            })
            .with_border(
                1.0,
                if met {
                    Color::new(0.28, 0.70, 0.42, 1.0)
                } else {
                    Color::new(0.74, 0.28, 0.32, 1.0)
                },
            ),
        );
        draw_ui_text_ex(
            &format!(
                "{} {} {:.2}-{:.2}",
                if met { "OK" } else { "NO" },
                label,
                range[0],
                range[1]
            ),
            cell.x + 6.0,
            cell.y + 16.0,
            TextStyle::new(10.0, dark::TEXT).params(),
        );
    }
}

fn draw_range_grid(buyer: &AlienBuyer, rect: Rect) {
    let requirements = [
        ("Temp", buyer.temp_range),
        ("Atmo", buyer.atmo_range),
        ("Water", buyer.water_range),
        ("Grav", buyer.grav_range),
        ("Rad", buyer.rad_range),
    ];
    for (index, (label, range)) in requirements.iter().enumerate() {
        let x = rect.x + 12.0 + (index % 2) as f32 * (rect.w * 0.5 - 10.0);
        let y = rect.y + 84.0 + (index / 2) as f32 * 28.0;
        draw_ui_text_ex(
            &format!("{}: {:.2} to {:.2}", label, range[0], range[1]),
            x,
            y,
            TextStyle::new(10.0, dark::TEXT_DIM).params(),
        );
    }
}

#[cfg(test)]
mod tests;
