use crate::mob::Mob;
use godot::classes::{Area2D, IArea2D, PhysicsBody2D};
use godot::prelude::*;

const SPEED: f32 = 1000.0;
const RANGE: f32 = 1200.0;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Bullet {
    travelled_distance: f32,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for Bullet {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            travelled_distance: 0.0,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let _delta = delta as f32;
        let direction = Vector2::RIGHT.rotated(self.base().get_rotation());
        let new_position = self.base_mut().get_position() + direction * SPEED * _delta;
        self.base_mut().set_position(new_position);
        self.travelled_distance += SPEED * _delta;

        if self.travelled_distance > RANGE {
            self.base_mut().queue_free();
        }
    }
}

#[godot_api]
impl Bullet {
    #[func]
    fn on_body_entered(&mut self, body: Gd<PhysicsBody2D>) {
        if body.is_class("Mob") {
            let mut mob = body.cast::<Mob>();
            mob.bind_mut().take_damage();
        }
    }
}
