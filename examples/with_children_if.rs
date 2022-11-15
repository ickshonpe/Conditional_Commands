use bevy::prelude::*;
use conditional_commands::*;

#[derive(Component)]
struct A;

#[derive(Component)]
struct B;

fn spawn_entities(mut commands: Commands) {
    for i in 0..10 {
        commands
            .spawn_empty()
            .insert(A)
            .with_children_if(i % 2 == 0, |child_builder| {
                child_builder.spawn(B);
            });
    }
}

fn report(a_query: Query<&A>, b_query: Query<&B>) {
    println!("{} entities with component A.", a_query.iter().count());
    println!("{} entities with component B.", b_query.iter().count());
}

fn main() {
    App::new()
        .add_startup_system(spawn_entities)
        .add_system(report)
        .run();
}
