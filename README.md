# Conditional Commands

This crate implements three extension traits that allow for conditional component, bundle and child insertion without the need for an intermediate ```EntityCommands``` or ```EntityMut``` binding.
* ```ConditionalInsertBundleExt``` for ```EntityCommands``` and ```EntityMut```\
    Methods:
    - ```insert_if```
    - ```insert_if_else```
    - ```insert_some```
    - ```insert_some_or```
* ```ConditionalChildBuilderExt``` for ```EntityCommands```\
    Methods:
    - ```with_children_if```
* ```ConditionalWorldChildBuilderExt``` for ```EntityMut```\
    Methods:
    - ```with_children_if```

Supports Bevy 0.9

#

## Usage

To add to a project either use:
```
cargo add conditional_commands
```

or manually add to your Cargo.toml:
```toml
[dependencies]
conditional_commands = "0.9.0"
```

## A Motivating But Contrived ECS Fizz-Buzz Example

```rust
use bevy::prelude::*;
use conditional_commands::*;

#[derive(Component)]
struct FizzBuzzer;

#[derive(Component)]
struct Number(usize);

#[derive(Component)]
struct Fizz;

#[derive(Component)]
struct Buzz;

fn fizz_buzz<const N: usize>(
    mut commands: Commands
) {
    for n in 1 ..= N {
        let mut entity_commands = commands.spawn(FizzBuzzer);
        match (n % 3, n % 5) {
            (0, 0) => { entity_commands.insert((Fizz, Buzz)); },
            (0, _) => { entity_commands.insert(Fizz); },
            (_, 0) => { entity_commands.insert(Buzz); },
            _ => { entity_commands.insert(Number(n)); }
        }
    }
}
```

With Conditional Commands the intermediate EntityCommands binding in no longer required.

```rust
fn fizz_buzz<const N: usize>(
    mut commands: Commands
) {
    for n in 1 ..= N {
        commands
        .spawn(FizzBuzzer)
        .insert_if(0 < n % 3 && 0 < n % 5, || Number(n))
        .insert_if(n % 3 == 0, || Fizz)
        .insert_if(n % 5 == 0, || Buzz);
    }
}
```

`ConditionalInsertBundleExt` is also implemented for `EntityMut`:

```rust
#[derive(Component)]
struct Even;

#[derive(Component)]
struct Odd;

fn exclusive_system(world: &mut World) {
    for n in 0..10 {
        world.spawn_empty().insert_if_else(n % 2 == 0, || Even, || Odd);
    }
}    
```
Bundles passed to the `_else`/`_or` methods don't need to be the same type,
as seen in the above example with the `Even` and `Odd` components.

Use `insert_some` to insert the inner value of an optional bundle, if present.

```rust
commands.spawn(MyBundle)
    .insert_some(Some(OtherBundle::default()))
    .insert_some_or(None::<MyThing>, AlternativeThing::default);
```

## Examples

```
cargo run --example exclusive
cargo run --example fizz_buzz
cargo run --example insert_if
cargo run --example insert_if_2
cargo run --example with_children_if

```
#
## Notes

* I haven't done any benchmarking. For performance critical systems it should be better to spawn entire bundles at once. 

* Earlier versions of this crate have both eager and lazy versions of each insertion method. I'm not sure the eager versions had any advantages (beyond no `||`), so they are gone.

* I wasn't able to quite finesse the lifetimes to get a single generic child builder trait for both ```EntityCommands``` and ```EntityMut```.
