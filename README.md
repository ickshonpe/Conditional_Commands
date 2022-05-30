# Conditional Commands

Provides an extension trait ```ConditionalEntityCommandsExt``` on Bevy ```EntityCommands```.

 It adds three new methods:

* ```insert_if```
* ```insert_bundle_if```
* ```with_children_if```

that allow for conditional component, bundle, and child insertion without the need for an intermediate ```EntityCommands``` binding.

* Supports Bevy 0.7
#
## Usage

Add to your Cargo.toml ```[Dependencies]``` section

```
conditional_commands = "0.1"
```

Then access with the ```use``` declaration

```rust
use conditional_commands::*;
```
#

## Contrived ECS Fizz-Buzz Example

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

