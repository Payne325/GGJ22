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
   thrown_speed: Vec2
}

impl ThrownMover {
   pub fn new(speed: Vec2) -> Self {
      ThrownMover{
         thrown_speed: speed
      }
   }
}

impl Mover for ThrownMover {
   fn apply_movement_routine(&mut self, world: &mut World, collider: &Actor, _: &mut Vec2) {
      
      world.move_h(*collider, self.thrown_speed.x * get_frame_time());
      world.move_v(*collider, self.thrown_speed.y * get_frame_time());
      
      let pos = world.actor_pos(*collider);
      if self.thrown_speed.y > 1. && pos.y >= 130. {
         self.thrown_speed.y *= -1.;
      }
      if self.thrown_speed.y < -1. && pos.y <= 40. {
         self.thrown_speed.y *= -1.;
      }
   }
}