use super::*;

#[test]
fn title_and_tutorial_use_the_same_clean_first_run_seed() {
    let expected = CaptureSceneSeed {
        reset_session: true,
        game_started: false,
        research_points: 0,
        first_research_complete: false,
        research_open: false,
        planet_demo: false,
        reset_open: false,
    };

    assert_eq!(CaptureSceneSeed::for_scene("title"), expected);
    assert_eq!(CaptureSceneSeed::for_scene("tutorial"), expected);
}

#[test]
fn research_seed_shows_mixed_progression_states() {
    assert_eq!(
        CaptureSceneSeed::for_scene("research"),
        CaptureSceneSeed {
            reset_session: true,
            game_started: true,
            research_points: 115,
            first_research_complete: true,
            research_open: true,
            planet_demo: false,
            reset_open: false,
        }
    );
}

#[test]
fn biosphere_capture_scene_seeds_an_active_planet() {
    assert_eq!(
        CaptureSceneSeed::for_scene("biosphere"),
        CaptureSceneSeed {
            reset_session: true,
            game_started: true,
            research_points: 0,
            first_research_complete: false,
            research_open: false,
            planet_demo: true,
            reset_open: false,
        }
    );
}

#[test]
fn reset_capture_scene_opens_confirmation() {
    assert_eq!(
        CaptureSceneSeed::for_scene("reset"),
        CaptureSceneSeed {
            reset_session: true,
            game_started: true,
            research_points: 0,
            first_research_complete: false,
            research_open: false,
            planet_demo: false,
            reset_open: true,
        }
    );
}

#[test]
fn ordinary_capture_scene_keeps_loaded_gameplay_state() {
    assert_eq!(
        CaptureSceneSeed::for_scene("gameplay"),
        CaptureSceneSeed {
            reset_session: false,
            game_started: false,
            research_points: 0,
            first_research_complete: false,
            research_open: false,
            planet_demo: false,
            reset_open: false,
        }
    );
    assert_eq!(
        CaptureSceneSeed::for_scene("unknown"),
        CaptureSceneSeed::for_scene("gameplay")
    );
}

#[test]
fn market_refresh_waits_for_started_session_and_valid_interval() {
    assert!(!market_refresh_due(false, 60.0, 30.0));
    assert!(!market_refresh_due(true, 29.99, 30.0));
    assert!(market_refresh_due(true, 30.0, 30.0));
    assert!(!market_refresh_due(true, 30.0, 0.0));
}
