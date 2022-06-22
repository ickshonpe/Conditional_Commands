# Conditional Commands

Implements three extension traits 
* ```ConditionalInsertComponentsExt``` for ```EntityCommands``` and ```EntityMut```\
    with methods:
    - ```insert_if```
    - ```insert_if_lazy```
    - ```insert_if_else```
    - ```insert_if_else_lazy```
    - ```insert_bundle_if```
    - ```insert_bundle_if_lazy```
    - ```insert_bundle_if_else```
    - ```insert_bundle_if_else_lazy```
    - ```insert_some```
    - ```insert_some_or_else```
    - ```insert_bundle_some```
    - ```insert_bundle_some_or_else```
* ```ConditionalChildBuilderExt``` for ```EntityCommands```\
    with method:
    - ```with_children_if```
* ```ConditionalWorldChildBuilderExt``` for ```EntityMut```\
    with method:
    - ```with_children_if```

that allow for conditional component, bundle, and child insertion without the need for an intermediate ```EntityCommands``` or ```EntityMut``` binding.
* Supports Bevy 0.7

#
## Version 0.5
* Lazy equivalents for insertion methods. 

## Version 0.4
* ```insert_some_or_else``` methods. If present, insert the inner value of an ```Option```, otherwise insert the alternative value

## New for Version 0.3
* ```insert_some``` methods. If present, insert the inner value of an ```Option```.

## Version 0.2

* Implementated traits for ```EntityMut```.

* ```*_if_else``` methods.

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
conditional_commands = "0.5"
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
#
## Notes

Some ergonomic compromises I had to make:

* Wasn't able to quite finesse the lifetimes to get a single generic child builder trait for both ```EntityCommands``` and ```EntityMut```.
* I wanted a generic API with methods that can accept both a ```Component``` or an ```Fn() -> Component```. That doesn't seem possible because Rust doesn't have negative trait bound. So I have an explosion of if_else / if_else lazy methods that isn't very satisfying. 

Any solutions for either issue would be very welcome.

Considering seperating the lazy methods into a seperate trait or feature gating them.

Also not happy with the naming of the methods. 

