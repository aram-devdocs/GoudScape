use super::movement::{MovementTracker, CubePosition};

#[test]
fn test_track_movement() {
    let tracker = MovementTracker::new();
    tracker.track_movement(5.0, 10.0);

    let position = tracker.get_position();
    assert_eq!(position.x, 5.0);
    assert_eq!(position.y, 10.0);
}

#[test]
fn test_update_position() {
    let tracker = MovementTracker::new();
    tracker.update_position(7.0, 14.0);

    let position = tracker.get_position();
    assert_eq!(position.x, 7.0);
    assert_eq!(position.y, 14.0);
}
