use godot::classes::CharacterBody2D;
use godot::classes::ICharacterBody2D;
use godot::prelude::*;

use crate::player::Player;

const SMOKE_PATH: &str = "res://smoke_explosion/smoke_explosion.tscn";

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Mob {
    base: Base<CharacterBody2D>,
    health: i8,
    player: Option<Gd<Player>>,
}

#[godot_api]
impl ICharacterBody2D for Mob {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            health: 3,
            player: None,
            base,
        }
    }

    fn ready(&mut self) {
        self.player = Some(self.base().get_node_as::<Player>("/root/Game/Player"));

        let mut node = self.base().get_node_as::<Node2D>("%Slime");
        node.call("play_walk", &[]);
    }

    fn physics_process(&mut self, _delta: f64) {
        if let Some(player) = &self.player {
            let direction = self
                .base()
                .get_global_position()
                .direction_to(player.get_global_position());
            self.base_mut().set_velocity(direction * 300.0);
            self.base_mut().move_and_slide();
        }
    }
}

#[godot_api]
impl Mob {
    #[func]
    pub fn take_damage(&mut self) {
        self.health -= 1;

        let mut node = self.base().get_node_as::<Node2D>("%Slime");
        node.call("play_hurt", &[]);

        if self.health == 0 {
            self.base_mut().queue_free();

            if let Some(mut parent) = self.base().get_parent() {
                let scene = load::<PackedScene>(&SMOKE_PATH.to_godot());
                let mut new_smoke = scene.instantiate_as::<Node2D>();
                new_smoke.set_global_position(self.base().get_global_position());
                parent.add_child(&new_smoke);
            }
        }
    }
}
