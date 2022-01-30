use macroquad::prelude::*;
use macroquad_platformer::*;
use crate::mover::*;

#[derive(PartialEq)]
pub enum PandaState {
   Normal,
   Grabbed,
   Thrown,
   FoundLove,
   ReadyForDeletion
}

pub struct Panda {
   pub collider: Actor,
   pub speed: Vec2,
   pub mover: Box<dyn Mover>,
   pub state: PandaState,
   pub heart_anim_index: f32,
   pub walk_anim_index: f32,
   pub love_anim_index: f32,
   pub frame_countdown: f32
}

impl Panda {
   pub fn apply_movement(&mut self, world: &mut World) {
      self
         .mover
         .apply_movement_routine(world, &self.collider, &mut self.speed)
   }

   pub fn update_animation_indices(&mut self) {
      self.frame_countdown = 0.05;
      self.heart_anim_index += 1.0;
      self.walk_anim_index += 1.0;
      self.love_anim_index += 1.0;

      if self.heart_anim_index == 4.0 {
         self.heart_anim_index = 0.0;
      }

      if self.walk_anim_index == 4.0 {
         self.walk_anim_index = 0.0;
      }

      if self.love_anim_index == 9.0 {
         self.love_anim_index = 0.0;
      }
   }
}


pub struct PandaFactory {
   
}

impl PandaFactory {
   pub fn create_panda(world: &mut World, pos: Vec2, init_speed: Vec2) -> Panda {
      Panda {
         collider: world.add_actor(pos, 32, 32),
         speed: init_speed,
         mover: Box::new(NormalMover::new()),
         state: PandaState::Normal,
         heart_anim_index: 0.0,
         walk_anim_index: 0.0,
         love_anim_index: 0.0,
         frame_countdown: 0.05,
      }
   }
}

