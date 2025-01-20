use bevy::prelude::*;
use networking::networking::{Networking, CubeMovement};
use std::net::SocketAddr;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(move_cube.system())
        .add_system(send_cube_movement.system())
        .run();
}

struct Cube;
struct NetworkResource {
    networking: Networking,
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
        sprite: Sprite::new(Vec2::new(30.0, 30.0)),
        ..Default::default()
    })
    .insert(Cube);

    let server_addr = "127.0.0.1:3000".parse::<SocketAddr>().unwrap();
    let networking = Networking::new(server_addr);
    commands.insert_resource(NetworkResource { networking });
}

fn move_cube(mut query: Query<(&Cube, &mut Transform)>, keyboard_input: Res<Input<KeyCode>>) {
    for (_cube, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }
        transform.translation += direction * 2.0;
    }
}

fn send_cube_movement(
    query: Query<(&Cube, &Transform)>,
    mut network_resource: ResMut<NetworkResource>,
) {
    for (_cube, transform) in query.iter() {
        let movement = CubeMovement {
            x: transform.translation.x,
            y: transform.translation.y,
        };
        network_resource.networking.send_cube_movement(movement);
    }
}
