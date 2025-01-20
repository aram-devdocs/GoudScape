pub mod networking {
    use renet::RenetClient;
    use serde::{Deserialize, Serialize};
    use std::net::SocketAddr;

    #[derive(Serialize, Deserialize)]
    pub struct CubeMovement {
        pub x: f32,
        pub y: f32,
    }

    pub struct Networking {
        client: RenetClient,
    }

    impl Networking {
        pub fn new(server_addr: SocketAddr) -> Self {
            let client = RenetClient::new(server_addr);
            Networking { client }
        }

        pub fn send_cube_movement(&mut self, movement: CubeMovement) {
            let data = serde_json::to_vec(&movement).unwrap();
            self.client.send(data);
        }

        pub fn receive_cube_movement(&mut self) -> Option<CubeMovement> {
            if let Some(data) = self.client.receive() {
                let movement: CubeMovement = serde_json::from_slice(&data).unwrap();
                Some(movement)
            } else {
                None
            }
        }
    }
}
