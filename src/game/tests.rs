use super::*;

#[test]
fn home_capture_is_a_clean_front_door() {
    let seed = CaptureSceneSeed::for_scene("home");
    assert!(seed.reset_session);
    assert_eq!(seed.screen, AppScreen::Home);
    assert!(!seed.game_started);
    assert!(!seed.settings_open);
    assert_eq!(seed.tutorial_step, TutorialStep::Complete);
    assert_eq!(CaptureSceneSeed::for_scene("title"), seed);
}

#[test]
fn tutorial_capture_opens_the_guided_orientation() {
    let seed = CaptureSceneSeed::for_scene("tutorial");
    assert_eq!(seed.screen, AppScreen::Gameplay);
    assert!(seed.game_started);
    assert_eq!(seed.tutorial_step, TutorialStep::Welcome);
    assert_eq!(
        CaptureSceneSeed::for_scene("tutorial_buy").tutorial_step,
        TutorialStep::BuyPlanet
    );
    let market = CaptureSceneSeed::for_scene("tutorial_market");
    assert_eq!(market.tutorial_step, TutorialStep::InspectBuyer);
    assert!(market.planet_demo);
}

#[test]
fn settings_capture_opens_over_the_home_screen() {
    let seed = CaptureSceneSeed::for_scene("settings");
    assert_eq!(seed.screen, AppScreen::Home);
    assert!(seed.settings_open);
    assert!(!seed.game_started);
}

#[test]
fn research_seed_shows_mixed_progression_states() {
    let seed = CaptureSceneSeed::for_scene("research");
    assert_eq!(seed.screen, AppScreen::Gameplay);
    assert!(seed.game_started);
    assert_eq!(seed.research_points, 115);
    assert!(seed.first_research_complete);
    assert!(seed.research_open);
    assert_eq!(seed.tutorial_step, TutorialStep::Complete);
}

#[test]
fn biosphere_capture_scene_seeds_an_active_planet() {
    let seed = CaptureSceneSeed::for_scene("biosphere");
    assert!(seed.planet_demo);
    assert_eq!(seed.screen, AppScreen::Gameplay);
    assert_eq!(seed.tutorial_step, TutorialStep::Complete);
}

#[test]
fn reset_capture_scene_opens_confirmation() {
    let seed = CaptureSceneSeed::for_scene("reset");
    assert!(seed.reset_open);
    assert_eq!(seed.screen, AppScreen::Gameplay);
    assert_eq!(seed.tutorial_step, TutorialStep::Complete);
}

#[test]
fn ordinary_capture_scene_is_seeded_gameplay() {
    let seed = CaptureSceneSeed::for_scene("gameplay");
    assert!(seed.reset_session);
    assert!(seed.game_started);
    assert_eq!(seed.screen, AppScreen::Gameplay);
    assert_eq!(seed.tutorial_step, TutorialStep::Complete);
    assert_eq!(CaptureSceneSeed::for_scene("unknown"), seed);
}

#[test]
fn market_refresh_waits_for_started_session_and_valid_interval() {
    assert!(!market_refresh_due(false, 60.0, 30.0));
    assert!(!market_refresh_due(true, 29.99, 30.0));
    assert!(market_refresh_due(true, 30.0, 30.0));
    assert!(!market_refresh_due(true, 30.0, 0.0));
}
