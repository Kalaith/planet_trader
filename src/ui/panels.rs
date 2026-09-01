use super::*;

pub(super) fn draw_workshop_mode(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_tools_panel(ctx, mouse, actions);
    draw_center_column(ctx, mouse, actions);
    draw_workshop_brief(ctx, mouse, actions);
}

fn draw_workshop_brief(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_panel(MARKET_PANEL, "Workshop Brief");
    let body = Rect::new(
        MARKET_PANEL.x + 14.0,
        MARKET_PANEL.y + 58.0,
        MARKET_PANEL.w - 28.0,
        112.0,
    );
    draw_text_block(
        "Engineer one stat at a time. Every tool spends credits and may create side effects. Biosphere is a sixth buyer requirement shaped by biological technology.",
        body.x,
        body.y,
        body.w,
        body.h,
        13.0,
        4.0,
        Color::new(0.62, 0.78, 0.84, 1.0),
    );
    if let Some(planet) = ctx.session.current_planet() {
        draw_ui_text_ex(
            "BEST LIVE OPPORTUNITY",
            MARKET_PANEL.x + 16.0,
            MARKET_PANEL.y + 190.0,
            TextStyle::new(11.0, dark::TEXT_DIM).params(),
        );
        if let Some((buyer, score, estimate, profit)) =
            best_market_route(planet, &ctx.session.alien_buyers)
        {
            draw_ui_text_ex(
                &buyer.name,
                MARKET_PANEL.x + 16.0,
                MARKET_PANEL.y + 222.0,
                TextStyle::new(19.0, dark::TEXT_BRIGHT).params(),
            );
            draw_text_block(
                &format!(
                    "{:.0}% compatible  //  {} CR sale\n{:+} CR projected margin",
                    score * 100.0,
                    estimate,
                    profit
                ),
                MARKET_PANEL.x + 16.0,
                MARKET_PANEL.y + 238.0,
                MARKET_PANEL.w - 32.0,
                54.0,
                12.0,
                3.0,
                if score >= 0.6 {
                    Color::new(0.46, 0.94, 0.62, 1.0)
                } else {
                    Color::new(1.0, 0.72, 0.36, 1.0)
                },
            );
        }
        draw_ui_text_ex(
            "ACTIVE INVESTMENT",
            MARKET_PANEL.x + 16.0,
            MARKET_PANEL.y + 312.0,
            TextStyle::new(11.0, dark::TEXT_DIM).params(),
        );
        draw_ui_text_ex(
            &format!("{} CR", planet.invested_cost),
            MARKET_PANEL.x + 16.0,
            MARKET_PANEL.y + 346.0,
            TextStyle::new(27.0, dark::TEXT_BRIGHT).params(),
        );
    }
    if button(
        Rect::new(
            MARKET_PANEL.x + 16.0,
            MARKET_PANEL.bottom() - 116.0,
            MARKET_PANEL.w - 32.0,
            42.0,
        ),
        "COMPARE ALIEN DEMAND",
        ctx.session.current_planet().is_some(),
        ButtonTone::Primary,
        mouse,
    ) {
        actions.push(UiAction::SetMode(GameplayMode::Market));
    }
    if button(
        Rect::new(
            MARKET_PANEL.x + 16.0,
            MARKET_PANEL.bottom() - 62.0,
            MARKET_PANEL.w - 32.0,
            42.0,
        ),
        &ctx.session
            .current_planet()
            .map(|planet| format!("SALVAGE FOR {} CR", salvage_value(planet)))
            .unwrap_or_else(|| "SALVAGE WORLD".to_owned()),
        ctx.session.current_planet().is_some(),
        ButtonTone::Secondary,
        mouse,
    ) {
        actions.push(UiAction::ScrapPlanet);
    }
}

fn draw_tools_panel(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_panel_with_left_title(TOOLS_PANEL, "Terraform Tools");
    draw_ui_text_ex(
        &format!(
            "Tap a tool to inspect it, then tap USE.   RP: {}",
            ctx.session.research_points
        ),
        TOOLS_PANEL.x + 14.0,
        TOOLS_PANEL.y + 58.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );
    if button(
        Rect::new(
            TOOLS_PANEL.right() - 156.0,
            TOOLS_PANEL.y + 10.0,
            78.0,
            28.0,
        ),
        "RESEARCH",
        ctx.session.game_started,
        ButtonTone::Primary,
        mouse,
    ) {
        actions.push(UiAction::OpenResearch);
    }

    let max_scroll = max_tool_scroll(ctx.data, ctx.expanded_tool);
    if button(
        Rect::new(TOOLS_PANEL.right() - 72.0, TOOLS_PANEL.y + 9.0, 32.0, 30.0),
        "UP",
        ctx.tool_scroll > 0.0,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::ScrollTools(-1));
    }
    if button(
        Rect::new(TOOLS_PANEL.right() - 36.0, TOOLS_PANEL.y + 9.0, 32.0, 30.0),
        "DN",
        ctx.tool_scroll < max_scroll,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::ScrollTools(1));
    }

    let list = Rect::new(
        TOOLS_PANEL.x + 12.0,
        TOOLS_PANEL.y + 70.0,
        TOOLS_PANEL.w - 24.0,
        TOOLS_PANEL.h - 82.0,
    );
    let scroll = ctx.tool_scroll.min(max_scroll);
    let mut y = list.y - scroll;
    for tool in &ctx.data.terraforming_tools {
        let expanded = ctx.expanded_tool == Some(tool.id.as_str());
        let height = tool_card_height(expanded);
        let rect = Rect::new(list.x, y, list.w, height);
        if rect.bottom() >= list.y && rect.y <= list.bottom() {
            draw_tool_card(ctx, tool, rect, mouse, actions);
        }
        y += height + 8.0;
    }
    draw_scroll_hint(list, scroll < max_scroll);
}

fn draw_tool_card(
    ctx: &UiContext<'_>,
    tool: &Tool,
    rect: Rect,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let locked = tool_is_locked(tool, &ctx.session.completed_research);
    let affordable = ctx.session.credits >= tool.cost;
    let usable = ctx.session.current_planet().is_some() && !locked && affordable;
    let hovered = rect.contains_point(mouse);
    let fill = if hovered {
        Color::new(0.12, 0.19, 0.26, 1.0)
    } else {
        Color::new(0.075, 0.12, 0.17, 1.0)
    };
    draw_surface(
        rect,
        &SurfaceStyle::new(fill)
            .with_border(1.0, if locked { dark::TEXT_DIM } else { dark::ACCENT })
            .with_left_accent(
                4.0,
                if locked {
                    Color::new(0.46, 0.28, 0.30, 1.0)
                } else {
                    Color::new(0.20, 0.72, 0.90, 1.0)
                },
            ),
    );

    draw_ui_text_ex(
        &format!("{} {}", tool_icon(&tool.category), tool.name),
        rect.x + 12.0,
        rect.y + 22.0,
        TextStyle::new(15.0, if locked { dark::TEXT_DIM } else { dark::TEXT }).params(),
    );
    let status = if locked {
        "LOCKED"
    } else if !affordable {
        "LOW CREDITS"
    } else if ctx.session.current_planet().is_none() {
        "SELECT PLANET"
    } else {
        "READY"
    };
    draw_ui_text_ex(
        &format!("{} | {} CR", status, tool.cost),
        rect.x + 12.0,
        rect.y + 43.0,
        TextStyle::new(
            11.0,
            if usable {
                Color::new(0.46, 0.95, 0.60, 1.0)
            } else {
                dark::TEXT_DIM
            },
        )
        .params(),
    );

    let use_rect = Rect::new(rect.right() - 68.0, rect.y + 12.0, 54.0, 30.0);
    let used = button(use_rect, "USE", usable, ButtonTone::Positive, mouse);
    if used {
        actions.push(UiAction::ApplyTool(tool.id.clone()));
    } else if hovered && is_mouse_button_released(MouseButton::Left) {
        actions.push(UiAction::ToggleTool(tool.id.clone()));
    }

    if ctx.expanded_tool != Some(tool.id.as_str()) {
        draw_ui_text_ex(
            &format!("Effect: {}", effect_summary(&tool.effect)),
            rect.x + 12.0,
            rect.bottom() - 10.0,
            TextStyle::new(11.0, Color::new(0.52, 0.70, 0.78, 1.0)).params(),
        );
        return;
    }

    draw_line(
        rect.x + 10.0,
        rect.y + 58.0,
        rect.right() - 10.0,
        rect.y + 58.0,
        1.0,
        Color::new(0.3, 0.45, 0.52, 0.7),
    );
    draw_text_block(
        &tool.description,
        rect.x + 12.0,
        rect.y + 68.0,
        rect.w - 24.0,
        48.0,
        11.0,
        2.0,
        dark::TEXT_DIM,
    );
    draw_text_block(
        &format!(
            "Primary: {}\nSide effects: {}",
            effect_summary(&tool.effect),
            effect_summary(&tool.side_effects)
        ),
        rect.x + 12.0,
        rect.y + 120.0,
        rect.w - 24.0,
        34.0,
        11.0,
        2.0,
        Color::new(0.68, 0.82, 0.85, 1.0),
    );
    let detail_label = if locked {
        let research_name = ctx
            .data
            .research
            .iter()
            .find(|research| {
                research.name == tool.name
                    || tool.upgrade_required.as_deref() == Some(research.name.as_str())
            })
            .map(|research| research.name.as_str())
            .or(tool.upgrade_required.as_deref())
            .unwrap_or("research");
        format!("Research {}", research_name)
    } else if usable {
        format!("Use {} for {} CR", tool.name, tool.cost)
    } else {
        "Select a planet and check credits".to_owned()
    };
    draw_text_centered_in_box(
        &detail_label,
        rect.x + 10.0,
        rect.bottom() - 34.0,
        rect.w - 20.0,
        24.0,
        11.0,
        if usable {
            Color::new(0.50, 1.0, 0.62, 1.0)
        } else {
            dark::TEXT_DIM
        },
    );
}

pub(super) fn draw_center_column(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_workshop(ctx, mouse, actions);
    draw_inventory(ctx, mouse, actions);
}

fn draw_workshop(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let rect = Rect::new(CENTER_PANEL.x, CENTER_PANEL.y, CENTER_PANEL.w, 330.0);
    draw_panel(rect, "Active Planet Workshop");
    let Some(planet) = ctx.session.current_planet() else {
        draw_text_centered_in_box(
            "PLANET",
            rect.x + 16.0,
            rect.y + 84.0,
            160.0,
            82.0,
            56.0,
            Color::new(0.35, 0.52, 0.66, 1.0),
        );
        draw_text_centered_in_box(
            "No active planet selected",
            rect.x + 18.0,
            rect.y + 174.0,
            156.0,
            26.0,
            16.0,
            dark::TEXT,
        );
        draw_text_centered_in_box(
            "Buy a planet, then tap it in your inventory.",
            rect.x + 24.0,
            rect.y + 205.0,
            144.0,
            54.0,
            12.0,
            dark::TEXT_DIM,
        );
        if button(
            Rect::new(rect.x + 42.0, rect.bottom() - 52.0, 148.0, 34.0),
            "Buy Planet",
            ctx.session.game_started,
            ButtonTone::Primary,
            mouse,
        ) {
            actions.push(UiAction::OpenPurchase);
        }
        if button(
            Rect::new(rect.x + 208.0, rect.bottom() - 52.0, 112.0, 34.0),
            "History",
            !ctx.session.trade_history.is_empty(),
            ButtonTone::Muted,
            mouse,
        ) {
            actions.push(UiAction::ToggleHistory);
        }
        return;
    };

    draw_planet_orb(planet, vec2(rect.x + 96.0, rect.y + 166.0), 82.0);
    draw_ui_text_ex(
        &format!("{} | {}", planet.name, planet.planet_type.name),
        rect.x + 188.0,
        rect.y + 76.0,
        TextStyle::new(18.0, Color::new(0.47, 0.82, 1.0, 1.0)).params(),
    );
    draw_ui_text_ex(
        &format!(
            "Invested {} CR  //  tune toward alien demand",
            planet.invested_cost
        ),
        rect.x + 188.0,
        rect.y + 98.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );

    let stats = [
        ("T  Temperature", format!("{:.0}°C", planet.temperature)),
        ("A  Atmosphere", format!("{:.2}x", planet.atmosphere)),
        ("W  Water", format!("{:.0}%", planet.water * 100.0)),
        ("G  Gravity", format!("{:.2}x", planet.gravity)),
        ("R  Radiation", format!("{:.2}x", planet.radiation)),
        ("B  Biosphere", format!("{:.1}", planet.biosphere)),
    ];
    for (index, (label, value)) in stats.iter().enumerate() {
        let row = Rect::new(
            rect.x + 188.0,
            rect.y + 128.0 + index as f32 * 25.0,
            382.0,
            21.0,
        );
        draw_surface(
            row,
            &SurfaceStyle::new(Color::new(0.08, 0.14, 0.19, 1.0))
                .with_border(1.0, Color::new(0.24, 0.37, 0.44, 0.7)),
        );
        draw_ui_text_ex(
            label,
            row.x + 10.0,
            row.y + 15.0,
            TextStyle::new(12.0, dark::TEXT_DIM).params(),
        );
        draw_text_right(
            value,
            row.right() - 10.0,
            row.y + 15.0,
            TextStyle::new(13.0, dark::TEXT_BRIGHT),
        );
    }
    if button(
        Rect::new(rect.right() - 246.0, rect.bottom() - 50.0, 84.0, 32.0),
        "History",
        !ctx.session.trade_history.is_empty(),
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::ToggleHistory);
    }
    if button(
        Rect::new(rect.right() - 154.0, rect.bottom() - 50.0, 138.0, 32.0),
        &format!("SALVAGE {} CR", salvage_value(planet)),
        true,
        ButtonTone::Secondary,
        mouse,
    ) {
        actions.push(UiAction::ScrapPlanet);
    }
}

fn draw_inventory(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let rect = Rect::new(
        CENTER_PANEL.x,
        CENTER_PANEL.y + 344.0,
        CENTER_PANEL.w,
        196.0,
    );
    draw_panel(rect, "Planet Inventory");
    if button(
        Rect::new(rect.right() - 128.0, rect.y + 12.0, 112.0, 30.0),
        "Buy",
        ctx.session.game_started,
        ButtonTone::Primary,
        mouse,
    ) {
        actions.push(UiAction::OpenPurchase);
    }
    if button(
        Rect::new(rect.right() - 198.0, rect.y + 9.0, 34.0, 32.0),
        "UP",
        ctx.inventory_scroll > 0,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::ScrollInventory(-1));
    }
    if button(
        Rect::new(rect.right() - 160.0, rect.y + 9.0, 34.0, 32.0),
        "DN",
        ctx.inventory_scroll + 2 < ctx.session.planets.len(),
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::ScrollInventory(1));
    }

    if ctx.session.planets.is_empty() {
        draw_text_centered_in_box(
            "No planets owned yet",
            rect.x + 18.0,
            rect.y + 104.0,
            rect.w - 36.0,
            30.0,
            17.0,
            dark::TEXT_DIM,
        );
        return;
    }

    let start = ctx
        .inventory_scroll
        .min(ctx.session.planets.len().saturating_sub(1));
    for (row, planet) in ctx.session.planets.iter().skip(start).take(2).enumerate() {
        let card = Rect::new(
            rect.x + 16.0,
            rect.y + 58.0 + row as f32 * 54.0,
            rect.w - 32.0,
            46.0,
        );
        let selected = ctx.session.current_planet_id.as_deref() == Some(planet.id.as_str());
        let hovered = card.contains_point(mouse);
        draw_surface(
            card,
            &SurfaceStyle::new(if selected {
                Color::new(0.08, 0.26, 0.39, 1.0)
            } else if hovered {
                Color::new(0.11, 0.20, 0.28, 1.0)
            } else {
                Color::new(0.07, 0.13, 0.18, 1.0)
            })
            .with_border(
                1.0,
                if selected {
                    Color::new(0.30, 0.78, 1.0, 1.0)
                } else {
                    Color::new(0.22, 0.35, 0.42, 1.0)
                },
            ),
        );
        draw_circle(
            card.x + 25.0,
            card.center().y,
            13.0,
            hex_to_color(&planet.color),
        );
        draw_circle_lines(card.x + 25.0, card.center().y, 13.0, 1.0, dark::TEXT);
        draw_ui_text_ex(
            &planet.name,
            card.x + 50.0,
            card.y + 20.0,
            TextStyle::new(14.0, dark::TEXT).params(),
        );
        draw_ui_text_ex(
            &planet.planet_type.name,
            card.x + 50.0,
            card.y + 36.0,
            TextStyle::new(11.0, dark::TEXT_DIM).params(),
        );
        draw_text_right(
            &format!("Buy {} CR", planet.purchase_price),
            card.right() - 12.0,
            card.y + 28.0,
            TextStyle::new(11.0, Color::new(0.48, 0.82, 0.60, 1.0)),
        );
        if hovered && is_mouse_button_released(MouseButton::Left) {
            actions.push(UiAction::SelectPlanet(planet.id.clone()));
        }
    }
}
