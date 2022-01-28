mod mover;

use macroquad::prelude::*;
use macroquad_tiled as tiled;
use macroquad_platformer::*;

use mover::{Mover, BasicMover, AltMover};

struct Player {
    collider: Actor,
    speed: Vec2,
}

#[derive(PartialEq)]
enum PandaState {
   normal,
   grabbed,
   thrown
}

struct Panda {
    collider: Actor,
    speed: Vec2,
    mover: Box<dyn Mover>,
    state: PandaState, 
}

impl Panda {
   pub fn apply_movement(&mut self, world: &mut World) {
      self.mover.apply_movement_routine(world, &self.collider, &mut self.speed)
   }
}

#[macroquad::main("Platformer")]
async fn main() {
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

    let mut player = Player {
        collider: world.add_actor(vec2(50.0, 80.0), 8, 8),
        speed: vec2(0., 0.),
    };

    let mut panda = Panda {
        collider: world.add_actor(vec2(170.0, 130.0), 8, 8),
        speed: vec2(0., 50.),
        mover: Box::new(AltMover {}),
        state: PandaState::normal,
    };

    let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 320.0, 152.0));

    loop {
        clear_background(BLACK);

        set_camera(&camera);

        tiled_map.draw_tiles("main layer", Rect::new(0.0, 0.0, 320.0, 152.0), None);

        const CHARACTER_SPRITE: u32 = 120;

        // draw panda
        {
            let pos = world.actor_pos(panda.collider);
            tiled_map.spr(
                "tileset",
                CHARACTER_SPRITE,
                Rect::new(pos.x + 8.0, pos.y, -8.0, 8.0),
            )
        }

        // draw player
        {
            // sprite id from tiled
            let pos = world.actor_pos(player.collider);
            if player.speed.x >= 0.0 {
                tiled_map.spr("tileset", CHARACTER_SPRITE, Rect::new(pos.x, pos.y, 8.0, 8.0));
            } else {
                tiled_map.spr(
                    "tileset",
                    CHARACTER_SPRITE,
                    Rect::new(pos.x + 8.0, pos.y, -8.0, 8.0),
                );
            }
        }

        // player movement control
        {
            if is_key_down(KeyCode::Right) {
                player.speed.x = 100.0;
                player.speed.y = 0.0;
            } else if is_key_down(KeyCode::Left) {
                player.speed.x = -100.0;
                player.speed.y = 0.0;
            } else if is_key_down(KeyCode::Up) {
                player.speed.x = 0.0;
                player.speed.y = -100.0;
            } else if is_key_down(KeyCode::Down) {
                player.speed.x = 0.0;
                player.speed.y = 100.0;
            } else {
                player.speed.x = 0.0;
                player.speed.y = 0.0;
            }

            world.move_h(player.collider, player.speed.x * get_frame_time());
            world.move_v(player.collider, player.speed.y * get_frame_time());
        }

        // panda movement
        {
            panda.apply_movement(&mut world);
        }

        // collision detection
        {
           let player_pos = world.actor_pos(player.collider);
           let panda_pos = world.actor_pos(panda.collider);
           
           if (player_pos.x - panda_pos.x).abs() < 5.0 && 
              (player_pos.y - panda_pos.y).abs() < 5.0 &&
              is_key_down(KeyCode::Space)
              {
                 panda.state = PandaState::grabbed;
                 println!("Yo put me down bro!");
              }

           // handle collision
           if panda.state == PandaState::grabbed {
               world.set_actor_position(panda.collider, player_pos + vec2(0., -5.));
           }
        }

        next_frame().await
    }
}