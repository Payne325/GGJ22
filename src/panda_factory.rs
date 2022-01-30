use macroquad::prelude::*;
use macroquad_platformer::*;
use crate::mover::*;

#[derive(PartialEq)]
pub enum PandaState {
   Normal,
   Grabbed,
   Thrown,
   FoundLove,
   Dead
}

pub struct Panda {
   pub collider: Actor,
   pub speed: Vec2,
   pub mover: Box<dyn Mover>,
   pub state: PandaState,
   pub heart_anim_index: f32,
   pub walk_anim_index: f32,
   pub love_anim_index: f32,
   pub thrown_anim_index: f32,
   pub frame_countdown: f32,
   pub spawn_time: f64
}

impl Panda {
   pub fn apply_movement(&mut self, world: &mut World) {
      self
         .mover
         .apply_movement_routine(world, &self.collider, &mut self.speed)
   }

   pub fn update_animation_indices(&mut self) {
      self.frame_countdown = 0.1;
      self.heart_anim_index += 1.0;
      self.walk_anim_index += 1.0;
      self.love_anim_index += 1.0;
      self.thrown_anim_index += 1.0;

      if self.heart_anim_index == 4.0 {
         self.heart_anim_index = 0.0;
      }

      if self.walk_anim_index == 4.0 {
         self.walk_anim_index = 0.0;
      }

      if self.love_anim_index == 9.0 {
         self.love_anim_index = 0.0;
      }

      if self.thrown_anim_index == 2.0 {
         self.thrown_anim_index = 0.0;
      }
   }
}


pub struct PandaFactory {
   
}

impl PandaFactory {
   pub fn create_panda(world: &mut World) -> Panda {
      
      let spawn_points = vec![vec2(170.0, 230.0), vec2(200.0, 100.0), vec2(350.0, 170.0), vec2(100.0, 350.0)];
      let spawn_index = rand::gen_range(0, spawn_points.len()) as usize;
   
      let speed_x = rand::gen_range(0.0, 50.0);
      let speed_y = rand::gen_range(0.0, 50.0);
      
      Panda {
         collider: world.add_actor(spawn_points[spawn_index], 16, 16),
         speed: vec2(speed_x, speed_y),
         mover: Box::new(NormalMover::new()),
         state: PandaState::Normal,
         heart_anim_index: 0.0,
         walk_anim_index: 0.0,
         love_anim_index: 0.0,
         thrown_anim_index: 0.0,
         frame_countdown: 0.05,
         spawn_time: get_time()
      }
   }
}

