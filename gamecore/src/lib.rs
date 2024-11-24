use godot::prelude::*;

mod player;
mod mob;
mod gun;
mod bullet;
mod game;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}