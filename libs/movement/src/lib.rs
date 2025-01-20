pub mod movement {
    use std::sync::Mutex;

    pub struct CubePosition {
        pub x: f32,
        pub y: f32,
    }

    pub struct MovementTracker {
        position: Mutex<CubePosition>,
    }

    impl MovementTracker {
        pub fn new() -> Self {
            MovementTracker {
                position: Mutex::new(CubePosition { x: 0.0, y: 0.0 }),
            }
        }

        pub fn track_movement(&self, x: f32, y: f32) {
            let mut position = self.position.lock().unwrap();
            position.x = x;
            position.y = y;
        }

        pub fn get_position(&self) -> CubePosition {
            let position = self.position.lock().unwrap();
            CubePosition {
                x: position.x,
                y: position.y,
            }
        }

        pub fn update_position(&self, x: f32, y: f32) {
            let mut position = self.position.lock().unwrap();
            position.x = x;
            position.y = y;
        }
    }
}
