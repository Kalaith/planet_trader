use super::*;

#[test]
fn wheel_scroll_direction_matches_panel_buttons() {
    assert_eq!(wheel_scroll_delta(1.0), Some(-1));
    assert_eq!(wheel_scroll_delta(-1.0), Some(1));
    assert_eq!(wheel_scroll_delta(0.0), None);
}
