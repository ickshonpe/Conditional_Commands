use bevy::prelude::*;
use conditional_commands::ConditionalEntityCommandsExt;

#[derive(Component)]
struct A;

#[derive(Component)]
struct B;

fn spawn_entities(
    mut commands: Commands,
) {
    for i in 0..10 { 
        commands
        .spawn()
        .insert(A)
        .insert_if(i < 3, B);
    }
}

fn report(
    a_query: Query<&A, Without<B>>,
    ab_query: Query<&A, With<B>>,
) {
    println!("{} entities with component A and without B.", a_query.iter().count());
    println!("{} entities with both component A and B.", ab_query.iter().count());
}

fn main() {
    App::new()
    .add_startup_system(spawn_entities)
    .add_system(report)
    .run();
}