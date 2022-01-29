use macroquad::prelude::*;
use macroquad_platformer::*;


pub trait Mover {
   fn apply_movement_routine(&mut self, world: &mut World, collider: &Actor, speed: &mut Vec2);
}

pub struct BasicMover {

}

impl Mover for BasicMover {
   fn apply_movement_routine(&mut self, world: &mut World, collider: &Actor, speed: &mut Vec2) {

      world.move_h(*collider, speed.x * get_frame_time());
      world.move_v(*collider, speed.y * get_frame_time());

      let pos = world.actor_pos(*collider);

      if speed.y > 1. && pos.x >= 220. {
         speed.y *= -1.;
      }
      if speed.y < -1. && pos.x <= 150. {
         speed.y *= -1.;
      }
   }
}


pub struct AltMover {

}

impl Mover for AltMover {
   fn apply_movement_routine(&mut self, world: &mut World, collider: &Actor, speed: &mut Vec2) {
      
      world.move_h(*collider, speed.x * get_frame_time());
      world.move_v(*collider, speed.y * get_frame_time());
      
      let pos = world.actor_pos(*collider);
      if speed.y > 1. && pos.y >= 130. {
         speed.y *= -1.;
      }
      if speed.y < -1. && pos.y <= 40. {
         speed.y *= -1.;
      }
   }
}

pub struct ThrownMover {
   thrown_direction: Vec2,
   throwing_speed: f32,
   init_time: f64
}

impl ThrownMover {
   pub fn new(dir: Vec2) -> Self {
      const THROWING_SPEED: f32 = 500.0;

      ThrownMover{
         thrown_direction: dir,
         throwing_speed: THROWING_SPEED,
         init_time: get_time(),
      }
   }
}

impl Mover for ThrownMover {
   fn apply_movement_routine(&mut self, world: &mut World, collider: &Actor, _: &mut Vec2) {

      if self.throwing_speed < 1.0 {
         self.throwing_speed = 0.0;
         ()
      }

      world.move_h(*collider, (self.thrown_direction.x * self.throwing_speed) * get_frame_time());
      world.move_v(*collider, (self.thrown_direction.y * self.throwing_speed) * get_frame_time());

      let time_delta = (get_time() - self.init_time) * 0.869;
      let decay_rate = 1.0 - (time_delta).powf(2.0);

      self.throwing_speed *= decay_rate as f32;
   }
}