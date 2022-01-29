use macroquad::prelude::*;
use macroquad_platformer::*;
use crate::mover::*;

#[derive(PartialEq)]
pub enum PandaState {
   Normal,
   Grabbed,
   Thrown,
}

pub struct Panda {
   pub collider: Actor,
   pub speed: Vec2,
   pub mover: Box<dyn Mover>,
   pub state: PandaState,
}

impl Panda {
   pub fn apply_movement(&mut self, world: &mut World) {
      self
         .mover
         .apply_movement_routine(world, &self.collider, &mut self.speed)
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
      }
   }
}
