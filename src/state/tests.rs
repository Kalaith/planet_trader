use super::*;
use crate::data::{GameConfig, GameData, PlanetType, ResearchDef, Species, SpeciesTemplate, Tool};
use std::collections::HashMap;

fn test_data() -> GameData {
    GameData {
        config: GameConfig {
            game_name: "planet_trader_test".to_owned(),
            display_name: "Planet Trader".to_owned(),
            save_slot: "test".to_owned(),
            version: "1.0.0".to_owned(),
            starting_credits: 10_000,
            buyer_refresh_seconds: 30.0,
        },
        planet_types: vec![PlanetType {
            name: "Test World".to_owned(),
            base_temp: 20.0,
            base_atmo: 1.0,
            base_water: 0.5,
            base_grav: 1.0,
            base_rad: 0.1,
            color: "#336699".to_owned(),
        }],
        terraforming_tools: Vec::new(),
        alien_species: vec![Species {
            name: "Test Species".to_owned(),
            description: "An authored test species".to_owned(),
            temp_range: [0.0, 100.0],
            atmo_range: [0.0, 3.0],
            water_range: [0.0, 1.0],
            grav_range: [0.0, 5.0],
            rad_range: [0.0, 2.0],
            base_price: 6_000,
            color: "#FFFFFF".to_owned(),
        }],
        alien_species_types: vec![SpeciesTemplate {
            prefixes: vec!["Test".to_owned()],
            suffixes: vec!["Buyer".to_owned()],
            desc: "A test buyer".to_owned(),
            temp: [0.0, 100.0],
            atmo: [0.0, 3.0],
            water: [0.0, 1.0],
            grav: [0.0, 5.0],
            rad: [0.0, 2.0],
            colors: vec!["#FFFFFF".to_owned()],
        }],
        planet_names: vec!["Testia".to_owned()],
        research: vec![ResearchDef {
            name: "Cryo Engineering".to_owned(),
            category: "Hydrology".to_owned(),
            rp_cost: 20,
            unlocks_tool: true,
            description: "Unlocks cryogenic tools".to_owned(),
        }],
    }
}

#[test]
fn purchase_spends_credits_and_selects_owned_planet() {
    let data = test_data();
    let mut session = GameSession::new(&data);
    session.game_started = true;
    session.open_purchase_modal(&data).unwrap();
    let id = session.planet_options[0].id.clone();
    let starting_credits = session.credits;
    let price = session.planet_options[0].purchase_price;

    session.purchase_planet(&id).unwrap();

    assert_eq!(session.planets.len(), 1);
    assert_eq!(session.credits, starting_credits - price);
    session.select_planet(&id).unwrap();
    assert_eq!(session.current_planet_id.as_deref(), Some(id.as_str()));
}

#[test]
fn purchase_modal_requires_start_and_can_be_closed() {
    let data = test_data();
    let mut session = GameSession::new(&data);

    assert!(session.open_purchase_modal(&data).is_err());
    assert!(!session.planet_modal_open);

    session.game_started = true;
    session.open_purchase_modal(&data).unwrap();
    assert!(session.planet_modal_open);
    assert!(!session.planet_options.is_empty());

    session.close_purchase_modal();
    assert!(!session.planet_modal_open);
}

#[test]
fn unaffordable_purchase_preserves_offer_and_session() {
    let data = test_data();
    let mut session = GameSession::new(&data);
    session.game_started = true;
    session.open_purchase_modal(&data).unwrap();
    let planet_id = session.planet_options[0].id.clone();
    let price = session.planet_options[0].purchase_price;
    let options_before = session.planet_options.clone();
    session.credits = price - 1;

    assert!(session.purchase_planet(&planet_id).is_err());
    assert_eq!(session.planet_options, options_before);
    assert!(session.planets.is_empty());
    assert_eq!(session.credits, price - 1);
    assert!(session.trade_history.is_empty());
}

#[test]
fn tool_applies_primary_and_side_effects_with_bounds() {
    let data = test_data();
    let mut session = GameSession::new(&data);
    session.game_started = true;
    session.open_purchase_modal(&data).unwrap();
    let id = session.planet_options[0].id.clone();
    session.purchase_planet(&id).unwrap();
    session.select_planet(&id).unwrap();

    let mut effect = HashMap::new();
    effect.insert("temperature".to_owned(), 10.0);
    let mut side_effects = HashMap::new();
    side_effects.insert("water".to_owned(), -0.2);
    let tool = Tool {
        id: "heat-generator".to_owned(),
        name: "Heat Generator".to_owned(),
        category: "temperature".to_owned(),
        effect,
        side_effects,
        cost: 100,
        tier: 1,
        unlocked: true,
        upgrade_required: None,
        description: "Test tool".to_owned(),
    };
    let before = session.current_planet().unwrap().clone();

    session.apply_tool(&tool).unwrap();
    let after = session.current_planet().unwrap();

    assert!((after.temperature - (before.temperature + 10.0)).abs() < f32::EPSILON);
    assert!((after.water - (before.water - 0.2)).abs() < f32::EPSILON);
    assert_eq!(after.invested_cost, before.purchase_price + 100);
    assert_eq!(session.credits, 10_000 - before.purchase_price - 100);
    assert_eq!(session.stats.total_spend, before.purchase_price + 100);
}

#[test]
fn biological_tool_updates_and_persists_biosphere() {
    let data = test_data();
    let mut session = GameSession::new(&data);
    session.game_started = true;
    session.open_purchase_modal(&data).unwrap();
    let id = session.planet_options[0].id.clone();
    session.purchase_planet(&id).unwrap();
    session.select_planet(&id).unwrap();
    let mut effect = HashMap::new();
    effect.insert("biosphere".to_owned(), 1.0);
    let tool = Tool {
        id: "bio-seeder".to_owned(),
        name: "Bio-Seeder Pods".to_owned(),
        category: "biological".to_owned(),
        effect,
        side_effects: HashMap::new(),
        cost: 400,
        tier: 3,
        unlocked: true,
        upgrade_required: None,
        description: "Test biological tool".to_owned(),
    };

    assert_eq!(session.current_planet().unwrap().biosphere, 0.0);
    session.apply_tool(&tool).unwrap();
    assert_eq!(session.current_planet().unwrap().biosphere, 1.0);

    let loaded = GameSession::from_save(session.to_save("1.1.0"), &data);
    assert_eq!(loaded.current_planet().unwrap().biosphere, 1.0);
}

#[test]
fn tool_use_requires_an_active_planet() {
    let data = test_data();
    let mut session = GameSession::new(&data);
    session.game_started = true;
    let starting_credits = session.credits;
    let tool = Tool {
        id: "heat-generator".to_owned(),
        name: "Heat Generator".to_owned(),
        category: "temperature".to_owned(),
        effect: HashMap::new(),
        side_effects: HashMap::new(),
        cost: 100,
        tier: 1,
        unlocked: true,
        upgrade_required: None,
        description: "Test tool".to_owned(),
    };

    assert!(session.apply_tool(&tool).is_err());
    assert_eq!(session.credits, starting_credits);
}

#[test]
fn locked_tool_use_preserves_planet_and_credits() {
    let data = test_data();
    let mut session = GameSession::new(&data);
    session.game_started = true;
    session.open_purchase_modal(&data).unwrap();
    let id = session.planet_options[0].id.clone();
    session.purchase_planet(&id).unwrap();
    session.select_planet(&id).unwrap();
    let tool = Tool {
        id: "ice-comet".to_owned(),
        name: "Ice Comet Bombardment".to_owned(),
        category: "water".to_owned(),
        effect: HashMap::new(),
        side_effects: HashMap::new(),
        cost: 200,
        tier: 2,
        unlocked: false,
        upgrade_required: Some("Cryo Engineering".to_owned()),
        description: "Test locked tool".to_owned(),
    };
    let planet_before = session.current_planet().unwrap().clone();
    let credits_before = session.credits;

    assert!(session.apply_tool(&tool).is_err());
    assert_eq!(session.current_planet().unwrap(), &planet_before);
    assert_eq!(session.credits, credits_before);
}

#[test]
fn low_credit_tool_use_preserves_planet_and_credits() {
    let data = test_data();
    let mut session = GameSession::new(&data);
    session.game_started = true;
    session.open_purchase_modal(&data).unwrap();
    let id = session.planet_options[0].id.clone();
    session.purchase_planet(&id).unwrap();
    session.select_planet(&id).unwrap();
    let mut effect = HashMap::new();
    effect.insert("temperature".to_owned(), 10.0);
    let tool = Tool {
        id: "heat-generator".to_owned(),
        name: "Heat Generator".to_owned(),
        category: "temperature".to_owned(),
        effect,
        side_effects: HashMap::new(),
        cost: 100,
        tier: 1,
        unlocked: true,
        upgrade_required: None,
        description: "Test tool".to_owned(),
    };
    session.credits = tool.cost - 1;
    let planet_before = session.current_planet().unwrap().clone();
    let credits_before = session.credits;

    assert!(session.apply_tool(&tool).is_err());
    assert_eq!(session.current_planet().unwrap(), &planet_before);
    assert_eq!(session.credits, credits_before);
}

#[test]
fn sale_requires_three_matching_requirements_and_removes_planet() {
    let data = test_data();
    let mut session = GameSession::new(&data);
    session.game_started = true;
    session.open_purchase_modal(&data).unwrap();
    let id = session.planet_options[0].id.clone();
    session.purchase_planet(&id).unwrap();
    session.select_planet(&id).unwrap();
    let buyer_id = session.alien_buyers[0].id;
    let sale_price = sale_price(session.current_planet().unwrap(), &session.alien_buyers[0]);
    let before = session.credits;

    session.sell_planet(buyer_id).unwrap();

    assert!(session.planets.is_empty());
    assert!(session.current_planet_id.is_none());
    assert_eq!(session.credits, before + sale_price);
    assert_eq!(session.stats.planets_sold, 1);
    assert_eq!(session.stats.total_revenue, sale_price);
    assert!(session.research_points > 0);
    assert_eq!(session.trade_history.len(), 2);
    assert_eq!(session.trade_history[1].sale_price, sale_price);
}

#[test]
fn incompatible_sale_is_rejected_without_mutating_session() {
    let data = test_data();
    let mut session = GameSession::new(&data);
    session.game_started = true;
    session.open_purchase_modal(&data).unwrap();
    let id = session.planet_options[0].id.clone();
    session.purchase_planet(&id).unwrap();
    session.select_planet(&id).unwrap();
    session.alien_buyers[0].temp_range = [999.0, 1_000.0];
    session.alien_buyers[0].atmo_range = [999.0, 1_000.0];
    session.alien_buyers[0].water_range = [999.0, 1_000.0];
    let buyer_id = session.alien_buyers[0].id;
    let credits_before = session.credits;
    let planet_count_before = session.planets.len();
    let history_len_before = session.trade_history.len();

    assert!(session.sell_planet(buyer_id).is_err());
    assert_eq!(session.credits, credits_before);
    assert_eq!(session.planets.len(), planet_count_before);
    assert_eq!(session.trade_history.len(), history_len_before);
    assert_eq!(session.current_planet_id.as_deref(), Some(id.as_str()));
}

#[test]
fn authored_species_drive_market_prices() {
    let data = test_data();
    let session = GameSession::new(&data);
    let buyer = &session.alien_buyers[0];

    assert_eq!(buyer.name, "Test Species");
    assert_eq!(buyer.base_price, 6_000);
    assert!((4_500..=7_500).contains(&buyer.current_price));
}

#[test]
fn compatibility_changes_sale_price() {
    let data = test_data();
    let mut session = GameSession::new(&data);
    session.game_started = true;
    session.open_purchase_modal(&data).unwrap();
    let id = session.planet_options[0].id.clone();
    session.purchase_planet(&id).unwrap();
    session.select_planet(&id).unwrap();

    let planet = session.current_planet().unwrap().clone();
    let buyer = session.alien_buyers[0].clone();
    let full_price = sale_price(&planet, &buyer);
    let mut poor_buyer = buyer.clone();
    poor_buyer.temp_range = [999.0, 1_000.0];
    let poor_price = sale_price(&planet, &poor_buyer);

    assert!(full_price > poor_price);
    assert_eq!(
        full_price,
        (buyer.current_price as f32 * 2.0).round() as i64
    );
}

#[test]
fn salvage_recovers_part_of_investment_and_records_loss() {
    let data = test_data();
    let mut session = GameSession::new(&data);
    session.game_started = true;
    session.open_purchase_modal(&data).unwrap();
    let id = session.planet_options[0].id.clone();
    session.purchase_planet(&id).unwrap();
    session.select_planet(&id).unwrap();
    let invested = session.current_planet().unwrap().invested_cost;
    let before = session.credits;

    session.salvage_current_planet().unwrap();

    let salvage = ((invested as f32) * 0.25).round().max(100.0) as i64;
    assert_eq!(session.credits, before + salvage);
    assert!(session.planets.is_empty());
    assert_eq!(session.stats.planets_salvaged, 1);
    assert_eq!(session.trade_history[1].transaction_type, "salvage");
}

#[test]
fn save_round_trip_preserves_market_and_trade_state() {
    let data = test_data();
    let mut session = GameSession::new(&data);
    session.game_started = true;
    session.open_purchase_modal(&data).unwrap();
    let id = session.planet_options[0].id.clone();
    session.purchase_planet(&id).unwrap();
    session.select_planet(&id).unwrap();
    session.research_points = 27;
    session
        .completed_research
        .push("Cryo Engineering".to_owned());

    let saved = session.to_save("1.1.0");
    let loaded = GameSession::from_save(saved, &data);

    assert_eq!(loaded.current_planet_id, session.current_planet_id);
    assert_eq!(loaded.planets, session.planets);
    assert_eq!(loaded.stats, session.stats);
    assert_eq!(loaded.trade_history, session.trade_history);
    assert_eq!(loaded.alien_buyers, session.alien_buyers);
    assert_eq!(loaded.research_points, session.research_points);
    assert_eq!(loaded.completed_research, session.completed_research);
}

#[test]
fn loading_save_canonicalizes_research_progression() {
    let data = test_data();
    let session = GameSession::new(&data);
    let mut save = session.to_save("1.1.0");
    save.completed_research = vec![
        "Cryo Engineering".to_owned(),
        "Unknown Node".to_owned(),
        "Cryo Engineering".to_owned(),
    ];

    let loaded = GameSession::from_save(save, &data);

    assert_eq!(
        loaded.completed_research,
        vec!["Cryo Engineering".to_owned()]
    );
    assert!(loaded.research_is_complete("Cryo Engineering"));
}

#[test]
fn gameplay_actions_require_a_started_session() {
    let data = test_data();
    let mut session = GameSession::new(&data);
    let planet = Planet::from_type(
        "planet-test".to_owned(),
        data.planet_types[0].clone(),
        "Testia".to_owned(),
    );
    let planet_id = planet.id.clone();
    session.planets.push(planet);
    session.current_planet_id = Some(planet_id.clone());
    let tool = Tool {
        id: "test-tool".to_owned(),
        name: "Test Tool".to_owned(),
        category: "temperature".to_owned(),
        effect: HashMap::new(),
        side_effects: HashMap::new(),
        cost: 100,
        tier: 1,
        unlocked: true,
        upgrade_required: None,
        description: "Test tool".to_owned(),
    };
    let buyer_id = session.alien_buyers[0].id;
    let offer = Planet::from_type(
        "planet-offer".to_owned(),
        data.planet_types[0].clone(),
        "Offeria".to_owned(),
    );
    let offer_id = offer.id.clone();
    session.planet_options.push(offer);

    assert!(session.purchase_planet(&offer_id).is_err());
    assert!(session.select_planet(&planet_id).is_err());
    assert!(session.apply_tool(&tool).is_err());
    assert!(session.complete_research(&data.research[0]).is_err());
    assert!(session.sell_planet(buyer_id).is_err());
    assert!(session.salvage_current_planet().is_err());
    assert_eq!(session.credits, data.config.starting_credits);
    assert!(session.completed_research.is_empty());
    assert_eq!(session.planets.len(), 1);
    assert_eq!(session.planet_options.len(), 1);
}

#[test]
fn legacy_planets_receive_their_purchase_cost_as_investment() {
    let data = test_data();
    let mut session = GameSession::new(&data);
    session.game_started = true;
    session.open_purchase_modal(&data).unwrap();
    let id = session.planet_options[0].id.clone();
    session.purchase_planet(&id).unwrap();
    let mut legacy_planet = session.planets[0].clone();
    legacy_planet.invested_cost = 0;

    let migrated = migrate_save_value(
        Some("1.0.0".to_owned()),
        serde_json::json!({
            "game_started": true,
            "credits": session.credits,
            "planets": [legacy_planet]
        }),
        &data.config,
    )
    .unwrap();
    let loaded = GameSession::from_save(migrated, &data);

    assert_eq!(
        loaded.planets[0].invested_cost,
        loaded.planets[0].purchase_price
    );
}

#[test]
fn legacy_save_migrates_to_current_shape() {
    let data = test_data();
    let value = serde_json::json!({
        "credits": 42,
        "game_started": true
    });

    let migrated = migrate_save_value(Some("0.1.0".to_owned()), value, &data.config).unwrap();

    assert_eq!(migrated.version, "1.0.0");
    assert_eq!(migrated.credits, 42);
    assert!(migrated.game_started);
}

#[test]
fn research_points_unlock_matching_advanced_tool() {
    let data = test_data();
    let mut session = GameSession::new(&data);
    session.game_started = true;
    let tool = Tool {
        id: "ice-comet".to_owned(),
        name: "Ice Comet Bombardment".to_owned(),
        category: "water".to_owned(),
        effect: HashMap::new(),
        side_effects: HashMap::new(),
        cost: 200,
        tier: 2,
        unlocked: false,
        upgrade_required: Some("Cryo Engineering".to_owned()),
        description: "Test locked tool".to_owned(),
    };

    assert!(tool_is_locked(&tool, &session.completed_research));
    session.research_points = 20;
    session.complete_research(&data.research[0]).unwrap();

    assert!(!tool_is_locked(&tool, &session.completed_research));
    assert_eq!(session.research_points, 0);
}

#[test]
fn research_requires_enough_points_and_cannot_repeat() {
    let data = test_data();
    let mut session = GameSession::new(&data);
    session.game_started = true;

    assert!(session.complete_research(&data.research[0]).is_err());
    session.research_points = 20;
    session.complete_research(&data.research[0]).unwrap();
    assert!(session.complete_research(&data.research[0]).is_err());
}
