use macroquad::prelude::*;
use macroquad_platformer::*;
use crate::mover::*;

#[derive(PartialEq)]
pub enum StorkState {
   Loaded,
   Unloaded
}

pub struct Stork {
   pub state: StorkState,
   pub pos: Vec2,
   pub dest: Vec2,
   pub speed: Vec2,

   pub frame_countdown: f32,
   pub frame_time: f32,
   pub anim_index: f32

}

impl Stork {
   pub fn apply_movement(&mut self, dt: f32) {
      self.pos += self.speed * dt;

      let min_distance_sq = 16.0 * 16.0;
      let x = self.pos.x - self.dest.x;
      let y = self.pos.y - self.dest.y;

      if (x*x + y*y < min_distance_sq && self.state == StorkState::Loaded) {
         self.state = StorkState::Unloaded;
         self.anim_index = 0.0;
         self.frame_countdown = 0.0;
      }
   }

   pub fn update_animation(&mut self, dt: f32) {
      self.frame_countdown += dt;

      if self.frame_countdown > self.frame_time {
         self.anim_index += 1.0;
         self.frame_countdown = 0.0;

         if self.anim_index == 3.0 {
            self.anim_index = 0.0;
         }
      }


   }
}


pub struct StorkFactory {
   
}

impl StorkFactory {
   pub fn create_stork(dest: Vec2) -> Stork {
      let mut pos = Vec2::new(0.0, dest.y);
      let mut speed = Vec2::new(0.0, 0.0);
      
      if dest.x < screen_width() * 0.5 {
         pos.x = -64.0;
         speed.x = 64.0;
      } else {
         pos.x = screen_width() + 64.0;
         speed.x = -64.0;
      }

      Stork {
         state: StorkState::Loaded,
         pos: pos,
         dest: dest,
         speed: speed,

         frame_countdown: 0.0,
         frame_time: 0.2,
         anim_index: 0.0
      }
   }
}

