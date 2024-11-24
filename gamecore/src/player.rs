use godot::classes::{Area2D, CharacterBody2D, ICharacterBody2D, ProgressBar};
use godot::prelude::*;

const DAMAGE_RATE: f64 = 5.0;

// Change the type in godot editor to apply the impl
#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,
    health: f64,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base,
            health: 100.0,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();
        let direction = input.get_vector("move_left", "move_right", "move_up", "move_down");
        let velocity = direction * 600.0;

        // To get unique name, right-click on the node to have a % path
        let mut node = self.base().get_node_as::<Node2D>("%HappyBoo");

        if velocity.length() > 0.0 {
            self.base_mut().set_velocity(velocity);
            self.base_mut().move_and_slide();

            node.call("play_walk_animation", &[]);
        } else {
            node.call("play_idle_animation", &[]);
        }

        let overlapping_mobs = self
            .base()
            .get_node_as::<Area2D>("%HurtBox")
            .get_overlapping_bodies();
        let mut health_bar = self.base().get_node_as::<ProgressBar>("%HealthBar");
        self.health -= (overlapping_mobs.len() as f64) * DAMAGE_RATE * delta;
        health_bar.set_value(self.health);

        if self.health <= 0.0 {
            self.base_mut().emit_signal("health_depleted", &[]);
        }
    }
}

#[godot_api]
impl Player {
    #[signal]
    fn health_depleted();
}
