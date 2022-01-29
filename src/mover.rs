use macroquad::prelude::*;
use macroquad_platformer::*;

pub trait Mover {
   fn apply_movement_routine(&mut self, world: &mut World, collider: &Actor, speed: &mut Vec2);
   fn movement_complete(&self) -> bool;
}

pub struct NormalMover {
   time_since_last_move: f64
}

impl NormalMover {
   pub fn new() -> Self {
      NormalMover {
         time_since_last_move: get_time()
      }
   }
}


impl Mover for NormalMover {
   fn apply_movement_routine(&mut self, world: &mut World, collider: &Actor, speed: &mut Vec2) {
      
      world.move_h(*collider, speed.x * get_frame_time());
      world.move_v(*collider, speed.y * get_frame_time());
      
      const TIME_TO_MOVE_SECONDS: f64 = 1.5;

      let elapsed_time = get_time() - self.time_since_last_move;
      if elapsed_time > TIME_TO_MOVE_SECONDS {

         let temp = speed.x;
         speed.x = -speed.y;
         speed.y = temp;

         self.time_since_last_move = get_time();
      } 
   }

   fn movement_complete(&self) -> bool {
      false
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

      let mut numerator = (self.thrown_direction.x * self.thrown_direction.x + 
         self.thrown_direction.y * self.thrown_direction.y).sqrt();

      if numerator == 0.0 {
         numerator = 1.0;
      }

      self.thrown_direction.x = self.thrown_direction.x / numerator;
      self.thrown_direction.y = self.thrown_direction.y / numerator;

      world.move_h(*collider, (self.thrown_direction.x * self.throwing_speed) * get_frame_time());
      world.move_v(*collider, (self.thrown_direction.y * self.throwing_speed) * get_frame_time());

      let time_delta = (get_time() - self.init_time) * 0.869;
      let decay_rate = 1.0 - (time_delta).powf(2.0);

      self.throwing_speed *= decay_rate as f32;
   }

   fn movement_complete(&self) -> bool {
      self.throwing_speed == 0.0
   }
}

pub struct LoveMover {
   init_time: f64,
   complete: bool
}

impl LoveMover {
   pub fn new() -> Self {
      LoveMover {
         init_time: get_time(),
         complete: false
      }
   }
}

impl Mover for LoveMover {
   fn apply_movement_routine(&mut self, _: &mut World, _: &Actor, _: &mut Vec2) {

      const TIMEOUT_SECONDS: f64 = 3.0;

      if (get_time() - self.init_time) > TIMEOUT_SECONDS {
         self.complete = true;
      }
   }
   fn movement_complete(&self) -> bool {
      self.complete
   }
} 