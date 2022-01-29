mod mover;
mod panda_factory;
mod tilemap;

use macroquad::prelude::*;
use macroquad_platformer::*;
use macroquad_tiled as tiled;
use macroquad::audio;

use mover::*;
use panda_factory::*;

use std::vec::Vec as Vector;

#[derive(PartialEq)]
enum PlayerState {
   Normal,
   Grabbing,
   Throwing,
}

struct Player {
   collider: Actor,
   speed: f32,
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

   let panda_normal_texture = load_texture("assets/panda.png").await.unwrap();
   let panda_thrown_texture = load_texture("assets/thrown_panda.png").await.unwrap();

   let player_normal_texture = load_texture("assets/cupid_panda.png").await.unwrap();
   let player_grabbing_texture = load_texture("assets/cupid_panda_black.png").await.unwrap();

   let heart_texture = load_texture("assets/heart.png").await.unwrap();

   /*

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
   */

  let mut world = World::new();
  let mut tilemap = tilemap::load_tilemap("assets/map.txt", &mut world).await;

  const THROW_COOLDOWN: f32 = 2.0;
   let mut player = Player {
      collider: world.add_actor(vec2(32.0, 32.0), 8, 8),
      speed: 100.0,
      dir: vec2(0.0, 0.0),
      state: PlayerState::Normal,
      throw_cooldown: THROW_COOLDOWN
   };

   let mut total_bamboo = 100.0;
   let mut pandas = Vector::<Panda>::new();

   println!("w:{}, h:{}", screen_width(), screen_height());

   pandas.push(PandaFactory::CreatePanda(&mut world, vec2(170.0, 130.0), vec2(0., 50.)));
   pandas.push(PandaFactory::CreatePanda(&mut world, vec2(200.0, 10.0), vec2(0., 0.)));
   
   let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 400.0, 300.0));

   loop {
      clear_background(GREEN);

      set_camera(&camera);

      // draw map
      tilemap.draw();

      // tiled_map.draw_tiles("main layer", Rect::new(0.0, 0.0, 320.0, 152.0), None);

      let text = format!("Remaining Bamboo: {}", total_bamboo as i32);
      draw_text_ex(
         &text,
         20.0,
         20.0,
         TextParams {
             font_size: 20,
             color: RED,
             ..Default::default()
         });

      // draw pandas
      {
         for panda in &pandas {
            let pos = world.actor_pos(panda.collider);
            
            if panda.state == PandaState::Thrown { 
               draw_texture_ex(panda_thrown_texture,
                  pos.x, 
                  pos.y, 
                  WHITE,
                  DrawTextureParams {
                     dest_size: Some(vec2(32.0, 32.0)),
                     source: Some(Rect::new(
                        0.0,
                        0.0,
                        32.0,
                        32.0,
                    )),
                     ..Default::default()
                 });
            } else { 
               draw_texture_ex(
                  panda_normal_texture,
                  pos.x, 
                  pos.y, 
                  WHITE,
                  DrawTextureParams {
                     dest_size: Some(vec2(32.0, 32.0)),
                     ..Default::default()
                 });
             };

             if panda.state == PandaState::FoundLove {
               draw_texture_ex(
                  heart_texture,
                  pos.x, 
                  pos.y - 8.0, 
                  WHITE,
                  DrawTextureParams {
                     dest_size: Some(vec2(16.0, 16.0)),
                     source: Some(Rect::new(
                        16.0 * panda.anim_index,
                        0.0,
                        16.0,
                        16.0,
                    )),
                     ..Default::default()
                 });
             }

         }
      }

      // draw player
      {
         // sprite id from tiled
         let pos = world.actor_pos(player.collider);
         let texture = if player.state == PlayerState::Grabbing { player_grabbing_texture } else { player_normal_texture };
         draw_texture_ex(texture,
            pos.x, 
            pos.y, 
            WHITE,
            DrawTextureParams {
               dest_size: Some(vec2(32.0, 32.0)),
               ..Default::default()
           });
      }

      // player control
      {
         player.dir = vec2(0.0, 0.0);

         if is_key_down(KeyCode::Right) {
            player.dir.x = 1.0;
         } else if is_key_down(KeyCode::Left) {
            player.dir.x = -1.0;
         } 
         
         if is_key_down(KeyCode::Up) {
            player.dir.y = -1.0;
         } else if is_key_down(KeyCode::Down) {
            player.dir.y = 1.0;
         } 
         
         let diag_move = player.dir.x != 0.0 && player.dir.y != 0.0;
         let x_speed = if diag_move {
             (1.0/(2.0 as f32).sqrt() * player.dir.x) * player.speed
            } else {
               player.dir.x * player.speed
            };

         let y_speed = if diag_move {
            (1.0/(2.0 as f32).sqrt() * player.dir.y) * player.speed
           } else {
              player.dir.y * player.speed
           };

         world.move_h(player.collider, x_speed * get_frame_time());
         world.move_v(player.collider, y_speed * get_frame_time());

         if player.state == PlayerState::Throwing {
            player.throw_cooldown -= get_frame_time();

            if player.throw_cooldown < 0.0 {
               player.throw_cooldown = THROW_COOLDOWN;
               player.state = PlayerState::Normal;
            }
         }
      }

      // panda movement
      {
         for panda in &mut pandas {
            if panda.state == PandaState::ReadyForDeletion{
               continue;
            }

            if panda.state == PandaState::Grabbed {
               if is_key_pressed(KeyCode::Space) {

                  player.state = PlayerState::Throwing;
                  panda.state = PandaState::Thrown;
                  panda.mover = Box::new(ThrownMover::new(player.dir));
               } 
               else {
                  let player_pos = world.actor_pos(player.collider);
                  world.set_actor_position(panda.collider, player_pos + vec2(0., -5.));
               }
            } else if panda.state == PandaState::FoundLove {
            
               panda.apply_movement(&mut world);

               if panda.mover.movement_complete() {
                  panda.state = PandaState::ReadyForDeletion;
               }
            } else {
               panda.apply_movement(&mut world);

               if panda.mover.movement_complete() {
                  panda.state = PandaState::Normal;
                  panda.mover = Box::new(NormalMover::new());
                  panda.speed = vec2(50., 0.);
               }
            }
         }
      }

      // collision detection
      {
         // detect player grabbing pandas 
         for panda in &mut pandas {

            let player_pos = world.actor_pos(player.collider);
            let panda_pos = world.actor_pos(panda.collider);
            //println!("{}", panda_pos);

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


         // detect pandas finding other pandas
         let mut in_love_indices = Vector::<usize>::new();
         let num_of_pandas = pandas.len();
         for first_panda_index in 0..num_of_pandas {

            let first_panda = &pandas[first_panda_index];
            if first_panda.state != PandaState::Normal {
               continue;
            }
            
            let first_panda_pos = world.actor_pos(first_panda.collider);

            for second_panda_index in first_panda_index+1..num_of_pandas {

               let second_panda = &pandas[second_panda_index];
               if second_panda.state != PandaState::Normal {
                  continue;
               }

               let second_panda_pos = world.actor_pos(second_panda.collider);

               const HUBBA_HUBBA_RANGE: f32 = 32.0;

               let val = (first_panda_pos.x - second_panda_pos.x).abs();
               let val2 = (first_panda_pos.y - second_panda_pos.y).abs();

               if val < HUBBA_HUBBA_RANGE && val2 < HUBBA_HUBBA_RANGE {
      
                  in_love_indices.push(first_panda_index);
                  in_love_indices.push(second_panda_index);
               }  
            }
         }

         for index in in_love_indices {
            pandas[index].state = PandaState::FoundLove;
            pandas[index].mover = Box::new(LoveMover::new());
         }
      }  

      // update bamboo count
      {
         if total_bamboo <= 0.0 {
            total_bamboo = 0.0;
         }
         else {
            let hungry_pandas = 
               pandas.iter().by_ref().filter(|p| p.state == PandaState::Normal).count();

            const HUNGER_RATE: f32 = 0.25;
            total_bamboo -= hungry_pandas as f32 * (HUNGER_RATE * get_frame_time());
         }
      }

      // update animation count
      {
         let lover_pandas = 
            pandas.iter_mut().filter(|p| p.state == PandaState::FoundLove);

         for p in lover_pandas {
            p.anim_index += 1.0;
            if p.anim_index == 16.0 {
               p.anim_index = 0.0;
            }
         }
      }

      next_frame().await
   }
}
