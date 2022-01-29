mod mover;
mod panda_factory;

use macroquad::prelude::*;
use macroquad_platformer::*;
use macroquad_tiled as tiled;
use macroquad::audio;

use mover::*;
use panda_factory::*;

use std::collections::VecDeque;

#[derive(PartialEq)]
enum PlayerState {
   Normal,
   Grabbing,
   Throwing,
}

struct Player {
   collider: Actor,
   speed: Vec2,
   dir: Vec2,
   state: PlayerState,
   throw_cooldown: f32
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DrumFillEvent {
    Start,
    Finish,
}

#[macroquad::main("Platformer")]
async fn main() {
   let track1 = audio::load_sound("assets/GGJ22_a2_loop.wav").await.unwrap();
   audio::play_sound(track1, audio::PlaySoundParams{ looped: true, volume: 0.015});

   let tileset = load_texture("assets/tileset.png").await.unwrap();
   tileset.set_filter(FilterMode::Nearest);

   let tiled_map_json = load_string("assets/map.json").await.unwrap();
   let tiled_map = tiled::load_map(&tiled_map_json, &[("tileset.png", tileset)], &[]).unwrap();

   let mut static_colliders = vec![];
   for (_x, _y, tile) in tiled_map.tiles("main layer", None) {
      static_colliders.push(if tile.is_some() {
         Tile::Solid
      } else {
         Tile::Empty
      });
   }

   let mut world = World::new();
   world.add_static_tiled_layer(static_colliders, 8., 8., 40, 1);

   const THROW_COOLDOWN: f32 = 2.0;
   let mut player = Player {
      collider: world.add_actor(vec2(50.0, 80.0), 8, 8),
      speed: vec2(0., 0.),
      dir: vec2(1.0, 0.0),
      state: PlayerState::Normal,
      throw_cooldown: THROW_COOLDOWN
   };

   
   let mut pandas = VecDeque::<Panda>::new();

   pandas.push_back(PandaFactory::CreatePanda(&mut world, vec2(170.0, 130.0), vec2(0., 50.)));
   pandas.push_back(PandaFactory::CreatePanda(&mut world, vec2(10.0, 10.0), vec2(0., 0.)));
   
   let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 320.0, 152.0));

   loop {
      clear_background(BLACK);

      set_camera(&camera);

      tiled_map.draw_tiles("main layer", Rect::new(0.0, 0.0, 320.0, 152.0), None);

      const CHARACTER_SPRITE: u32 = 120;

      // draw pandas
      {
         for panda in &pandas {
            let pos = world.actor_pos(panda.collider);
            tiled_map.spr(
               "tileset",
               CHARACTER_SPRITE,
               Rect::new(pos.x + 8.0, pos.y, -8.0, 8.0),
            )
         }
      }

      // draw player
      {
         // sprite id from tiled
         let pos = world.actor_pos(player.collider);
         if player.speed.x >= 0.0 {
            tiled_map.spr(
               "tileset",
               CHARACTER_SPRITE,
               Rect::new(pos.x, pos.y, 8.0, 8.0),
            );
         } else {
            tiled_map.spr(
               "tileset",
               CHARACTER_SPRITE,
               Rect::new(pos.x + 8.0, pos.y, -8.0, 8.0),
            );
         }
      }

      // player control
      {
         let any_movement_key_down = is_key_down(KeyCode::Right) || 
                                     is_key_down(KeyCode::Left) ||
                                     is_key_down(KeyCode::Up) ||
                                     is_key_down(KeyCode::Down);

         if  !any_movement_key_down {
            player.speed.x = 0.0;
            player.speed.y = 0.0;
         }

         const PLAYER_X_SPEED: f32 = 100.0;
         const PLAYER_Y_SPEED: f32 = 75.0;

         let mut changed_dir = false;
         let mut new_dir = vec2(0.0, 0.0);

         if is_key_down(KeyCode::Right) {
            player.speed.x = PLAYER_X_SPEED;
            new_dir.x = 1.0;
            changed_dir = true;
         } else if is_key_down(KeyCode::Left) {
            player.speed.x = -PLAYER_X_SPEED;
            new_dir.x = -1.0;
            changed_dir = true;
         } 
         
         if is_key_down(KeyCode::Up) {
            player.speed.y = -PLAYER_Y_SPEED;
            new_dir.y = -1.0;
            changed_dir = true;
         } else if is_key_down(KeyCode::Down) {
            player.speed.y = PLAYER_Y_SPEED;
            new_dir.y = 1.0;
            changed_dir = true;
         } 

         if (changed_dir) {
            player.dir = new_dir;
         }
         
         world.move_h(player.collider, player.speed.x * get_frame_time());
         world.move_v(player.collider, player.speed.y * get_frame_time());

         for panda in &mut pandas {
            if player.state == PlayerState::Grabbing &&
               is_key_pressed(KeyCode::Space) {

                  player.state = PlayerState::Throwing;
                  panda.state = PandaState::Thrown;
                  panda.mover = Box::new(ThrownMover::new(player.dir));
            }
            else if player.state == PlayerState::Throwing {
               player.throw_cooldown -= get_frame_time();

               if player.throw_cooldown < 0.0 {
                  player.throw_cooldown = THROW_COOLDOWN;
                  player.state = PlayerState::Normal;
               }
            }
         }
      }

      // panda movement
      {
         for panda in &mut pandas {
            if panda.state == PandaState::Grabbed {
               let player_pos = world.actor_pos(player.collider);
               world.set_actor_position(panda.collider, player_pos + vec2(0., -5.));
            } else {
               panda.apply_movement(&mut world);

               if panda.mover.movement_complete() {
                  panda.mover = Box::new(NormalMover::new());
                  panda.speed = vec2(50., 0.);
               }
            }
         }
      }

      // collision detection
      {
         for panda in &mut pandas {
            let player_pos = world.actor_pos(player.collider);
            let panda_pos = world.actor_pos(panda.collider);

            const GRAB_RANGE: f32 = 20.0;
            if (player_pos.x - panda_pos.x).abs() < GRAB_RANGE
               && (player_pos.y - panda_pos.y).abs() < GRAB_RANGE
               && is_key_pressed(KeyCode::Space)
               && panda.state != PandaState::Grabbed
               && player.state == PlayerState::Normal
            {
               panda.state = PandaState::Grabbed;
               player.state = PlayerState::Grabbing;
            }
         }
      }  

      next_frame().await
   }
}
