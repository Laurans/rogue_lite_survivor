use crate::bullet::Bullet;
use godot::classes::{Area2D, IArea2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
struct Gun {
    base: Base<Area2D>,
    bullet_scn: Gd<PackedScene>,
}

#[godot_api]
impl IArea2D for Gun {
    fn init(base: Base<Area2D>) -> Self {
        let scn_path = "res://bullet.tscn".to_godot();
        let scene = load::<PackedScene>(&scn_path);

        Self {
            base,
            bullet_scn: scene,
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        let enemies_in_range = self.base().get_overlapping_bodies();
        if let Some(target_enemy) = enemies_in_range.get(0) {
            self.base_mut().look_at(target_enemy.get_global_position());
        }
    }
}

#[godot_api]
impl Gun {
    fn shoot(&mut self) {
        let mut shooting_point = self.base().get_node_as::<Node2D>("%ShootingPoint");

        let mut new_bullet = self.bullet_scn.instantiate_as::<Bullet>();
        new_bullet.set_global_position(shooting_point.get_global_position());
        new_bullet.set_global_rotation(shooting_point.get_global_rotation());

        shooting_point.add_child(&new_bullet);
    }

    #[func]
    fn on_timer_timeout(&mut self) {
        self.shoot();
    }
}
