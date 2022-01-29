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
   pub anim_index: f32,
   pub frame_countdown: f32
}

impl Panda {
   pub fn apply_movement(&mut self, world: &mut World) {
      self
         .mover
         .apply_movement_routine(world, &self.collider, &mut self.speed)
   }

   pub fn reset_frame_countdown(&mut self) {
      self.frame_countdown = 0.05;
   }
}


pub struct PandaFactory {
   
}

impl PandaFactory {
   pub fn CreatePanda(world: &mut World, pos: Vec2, init_speed: Vec2) -> Panda {
      Panda {
         collider: world.add_actor(pos, 8, 8),
         speed: init_speed,
         mover: Box::new(NormalMover::new()),
         state: PandaState::Normal,
         anim_index: 0.0,
         frame_countdown: 0.05,
      }
   }
}

