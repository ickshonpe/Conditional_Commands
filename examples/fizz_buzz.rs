use bevy::prelude::*;
use conditional_commands::ConditionalInsertBundleExt;

#[derive(Component)]
struct FizzBuzzer;

#[derive(Component)]
struct Number(usize);

#[derive(Component)]
struct Fizz;

#[derive(Component)]
struct Buzz;

fn fizz_buzz<const N: usize>(mut commands: Commands) {
    for n in 1..=N {
        commands
            .spawn(FizzBuzzer)
            .insert_if(0 < n % 3 && 0 < n % 5, || Number(n))
            .insert_if(n % 3 == 0, || Fizz)
            .insert_if(n % 5 == 0, || Buzz);
    }
}

fn report(
    entities: Query<Entity>,
    query: Query<&Number>,
    fizz_query: Query<&Fizz, Without<Buzz>>,
    buzz_query: Query<&Buzz, Without<Fizz>>,
    fizz_buzz_query: Query<(&Fizz, &Buzz)>,
) {
    println!("{} Fizz-Buzz entites spawned", entities.iter().count());
    println!("{} Numbered", query.iter().count());
    println!("{} Only Fizz", fizz_query.iter().count());
    println!("{} Only Buzz", buzz_query.iter().count());
    println!("{} Fizz & Buzz", fizz_buzz_query.iter().count());
}

fn main() {
    App::new()
        .add_startup_system(fizz_buzz::<30>)
        .add_system(report)
        .run();
}
