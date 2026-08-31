use super::*;

#[test]
fn market_refresh_label_reflects_session_and_countdown_state() {
    assert_eq!(market_refresh_label(false, 0.0, 30.0), "Market offline");
    assert_eq!(market_refresh_label(true, 0.0, 30.0), "Refresh in 30s");
    assert_eq!(market_refresh_label(true, 12.2, 30.0), "Refresh in 18s");
    assert_eq!(market_refresh_label(true, 30.0, 30.0), "Refresh in 0s");
    assert_eq!(market_refresh_label(true, 0.0, 0.0), "No refresh");
}
