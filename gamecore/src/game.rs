use godot::prelude::*;
use godot::classes::{CanvasLayer, INode2D, Node2D, PathFollow2D};
use godot::global::randf;
use crate::mob::Mob;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Game {
    base: Base<Node2D>,
    mob_scn: Gd<PackedScene>,
    gameover_node: Option<Gd<CanvasLayer>>,
    pathfollow_node: Option<Gd<PathFollow2D>>
}

#[godot_api]
impl INode2D for Game {
    fn init(base: Base<Node2D>) -> Self {
        let scn_path = "res://mob.tscn".to_godot();
        let scene = load::<PackedScene>(&scn_path);
        Self {base, mob_scn: scene, gameover_node: None, pathfollow_node: None}
    }

    fn ready(&mut self) {
        let mut game_over = self.base().get_node_as::<CanvasLayer>("%GameOver");
        game_over.set_visible(false);
        self.gameover_node = Some(game_over);

        let pathfollow_node = self.base().get_node_as::<PathFollow2D>("%PathFollow2D");
        self.pathfollow_node = Some(pathfollow_node);
    }
}

#[godot_api]
impl Game {
    pub fn spawn_mob(&mut self) {
        let mut new_mob = self.mob_scn.instantiate_as::<Mob>();
        let pathfollow_node = self.pathfollow_node.as_mut().unwrap();
        pathfollow_node.set_progress_ratio(randf() as f32);
        new_mob.set_global_position(pathfollow_node.get_global_position());
        self.base_mut().add_child(&new_mob);
    }

    #[func]
    fn on_timer_timeout(&mut self) {
        self.spawn_mob();
    }

    #[func]
    fn on_player_health_depleted(&mut self){
        self.gameover_node.as_mut().unwrap().set_visible(true);
        self.base_mut().get_tree().unwrap().set_pause(true);
    }
}