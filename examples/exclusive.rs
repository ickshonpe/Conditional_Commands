use bevy::prelude::*;
use conditional_commands::*;

#[derive(Component)]
struct Even;

#[derive(Component)]
struct Odd;

fn exclusive_system(world: &mut World) {
    for n in 0..10 {
        world.spawn()
        .insert_if_else(n % 2 == 0, Even, Odd);
    }
}

fn count_components(
    evens_query: Query<With<Even>>,
    odds_query: Query<With<Odd>>,
) {
    println!("Counted {} entities with an Even component.", evens_query.iter().count());
    println!("Counted {} entities with an Odd component.", odds_query.iter().count());
}

fn main() {
    App::new()
    .add_startup_system(exclusive_system.exclusive_system())
    .add_system(count_components)
    .run();
}