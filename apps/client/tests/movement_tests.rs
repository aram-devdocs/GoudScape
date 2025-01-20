use bevy::prelude::*;
use bevy::ecs::system::SystemState;
use super::super::main::{Cube, move_cube};

#[test]
fn test_move_cube() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut system_state: SystemState<(
        Query<(&Cube, &mut Transform)>,
        Res<Input<KeyCode>>,
    )> = SystemState::new(&mut world);

    // Setup initial state
    let mut transform = Transform::default();
    transform.translation = Vec3::new(0.0, 0.0, 0.0);
    world.spawn().insert(Cube).insert(transform);

    // Simulate key press
    let mut input = Input::<KeyCode>::default();
    input.press(KeyCode::Right);
    resources.insert(input);

    // Run the system
    move_cube.system().run(&mut world, &mut resources);

    // Check the result
    let (cube, transform) = system_state.get(&world).0.single().unwrap();
    assert_eq!(transform.translation, Vec3::new(2.0, 0.0, 0.0));
}
