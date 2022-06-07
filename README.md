# Conditional Commands

Implements three extension traits 
* ```ConditionalInsertComponentsExt``` for ```EntityCommands``` and ```EntityMut```\
    with methods:
    - ```insert_if```
    - ```insert_if_else```
    - ```insert_bundle_if```
    - ```insert_bundle_if_else```
* ```ConditionalChildBuilderExt``` for ```EntityCommands```\
    with method:
    - ```with_children_if```
* ```ConditionalWorldChildBuilderExt``` for ```EntityMut```\
    with method:
    - ```with_children_if```

that allow for conditional component, bundle, and child insertion without the need for an intermediate ```EntityCommands``` or ```EntityMut``` binding.
* Supports Bevy 0.7

#
## New for Version 0.2

* Implementated traits for ```EntityMut```.

* ```*_if_else``` methods.

## Example
```rust
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
```

#

## Usage

Add to your Cargo.toml ```[Dependencies]``` section

```
conditional_commands = "0.2"
```

Then access with the ```use``` declaration

```rust
use conditional_commands::*;
```
#

## Motivating Contrived ECS Fizz-Buzz Example

```rust
use bevy::prelude::*;

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
        let mut entity_commands = commands.spawn();
        if n % 3 {
            (0, 0) => { entity_commands.insert_bundle((Fizz, Buzz)); },
            (0, _) => { entity_commands.insert(Fizz); },
            (_, 0) => { entity_commands.insert(Buzz); },
            _ => { entity_commands.insert(Number(n)); }
        }
    }
}
```
With Conditional Commands we no longer need the intermediate EntityCommands binding.

```rust
use conditional_commands::*;

fn fizz_buzz<const N: usize>(
    mut commands: Commands
) {
    for n in 1 ..= N {
        commands
        .spawn()
        .insert_if(0 < n % 3 && 0 < n % 5, Number(n))
        .insert_if(n % 3 == 0, Fizz)
        .insert_if(n % 5 == 0, Buzz);
    }
}
```

