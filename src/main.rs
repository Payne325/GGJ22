mod mover;
mod panda_factory;
mod tilemap;

use macroquad::audio::Sound;
use macroquad::audio::{self};
use macroquad::prelude::*;
use macroquad_platformer::*;
// use macroquad_tiled as tiled;

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
    throw_cooldown: f32,
    walk_anim_index: f32,
    frame_countdown: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DrumFillEvent {
    Start,
    Finish,
}

#[macroquad::main("Platformer")]
async fn main() {
    let track1 = audio::load_sound("assets/GGJ22_b3_loop.wav").await.unwrap();
    // play(&track1, true); //TODO: re-enable music

    let sfx_heart = audio::load_sound("assets/sfx_heart.wav").await.unwrap();
    let sfx_impact = audio::load_sound("assets/sfx_impact.wav").await.unwrap();
    let sfx_pickup = audio::load_sound("assets/sfx_pickup.wav").await.unwrap();
    let sfx_throw = audio::load_sound("assets/sfx_throw.wav").await.unwrap();
    let mut sfx_heart_counter = None;

    let font = load_ttf_font("./assets/Gameplay.ttf").await.unwrap();

    let panda_walking_texture = load_texture("assets/walking_panda.png").await.unwrap();
    let panda_thrown_texture = load_texture("assets/thrown_panda.png").await.unwrap();
    let panda_love_texture = load_texture("assets/dancing_panda.png").await.unwrap();

    let player_walking_texture = load_texture("assets/walking_cupid_panda.png")
        .await
        .unwrap();
    let player_grabbing_texture = load_texture("assets/walking_cupid_panda_black.png")
        .await
        .unwrap();

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
        // collider: world.add_actor(vec2(32.0, 150.0), 32, 32),
        collider: world.add_actor(vec2(32.0, 150.0), 10, 10),
        speed: 100.0,
        dir: vec2(0.0, 0.0),
        state: PlayerState::Normal,
        throw_cooldown: THROW_COOLDOWN,
        walk_anim_index: 0.0,
        frame_countdown: 0.05,
    };

    let mut total_bamboo = 100.0;
    let mut pandas = Vector::<Panda>::new();

    println!("w:{}, h:{}", screen_width(), screen_height());

    pandas.push(PandaFactory::create_panda(
        &mut world,
        vec2(170.0, 230.0),
        vec2(0., 50.),
    ));
    pandas.push(PandaFactory::create_panda(
        &mut world,
        vec2(200.0, 100.0),
        vec2(0., 0.),
    ));

    let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
    let render_target = render_target(400, 300);

    loop {
        camera.render_target = Some(render_target);
        set_camera(&camera);

        // draw map
        tilemap.draw();

        // tiled_map.draw_tiles("main layer", Rect::new(0.0, 0.0, 320.0, 152.0), None);

        // draw pandas
        {
            for panda in &pandas {
                let pos = world.actor_pos(panda.collider);

                if panda.state == PandaState::Thrown {
                    draw_texture_ex(
                        panda_thrown_texture,
                        pos.x - 5.0,
                        pos.y - 15.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(32.0, 32.0)),
                            source: Some(Rect::new(0.0, 0.0, 32.0, 32.0)),
                            ..Default::default()
                        },
                    );
                } else if panda.state == PandaState::Grabbed {
                    let ms = macroquad::time::get_time() * 1000.0;
                    if ms as u64 % 2 == 0 {
                        draw_texture_ex(
                            panda_walking_texture,
                            pos.x - 14.0,
                            pos.y - 30.0,
                            WHITE,
                            DrawTextureParams {
                                dest_size: Some(vec2(32.0, 32.0)),
                                source: Some(Rect::new(
                                    32.0 * panda.walk_anim_index,
                                    0.0,
                                    32.0,
                                    32.0,
                                )),
                                rotation: 0.5 * 3.14,
                                ..Default::default()
                            },
                        );
                    }
                } else if panda.state == PandaState::FoundLove {
                    draw_texture_ex(
                        panda_love_texture,
                        pos.x - 12.0,
                        pos.y - 15.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(32.0, 32.0)),
                            source: Some(Rect::new(32.0 * panda.love_anim_index, 0.0, 32.0, 32.0)),
                            ..Default::default()
                        },
                    );

                    draw_texture_ex(
                        heart_texture,
                        pos.x - 8.0,
                        pos.y - 25.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(16.0, 16.0)),
                            source: Some(Rect::new(16.0 * panda.heart_anim_index, 0.0, 16.0, 16.0)),
                            ..Default::default()
                        },
                    );
                } else {
                    draw_texture_ex(
                        panda_walking_texture,
                        pos.x - 12.0,
                        pos.y - 15.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(32.0, 32.0)),
                            source: Some(Rect::new(32.0 * panda.walk_anim_index, 0.0, 32.0, 32.0)),
                            ..Default::default()
                        },
                    );
                }
            }
        }

        // draw player
        {
            // sprite id from tiled
            let pos = world.actor_pos(player.collider);
            let texture = if player.state == PlayerState::Grabbing {
                player_grabbing_texture
            } else {
                player_walking_texture
            };
            draw_texture_ex(
                texture,
                pos.x - 12.0,
                pos.y - 15.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(32.0, 32.0)),
                    source: Some(Rect::new(32.0 * player.walk_anim_index, 0.0, 32.0, 32.0)),
                    ..Default::default()
                },
            );
        }

        // player control
        {
            let movement_is_happening = is_key_down(KeyCode::Right)
                || is_key_down(KeyCode::Left)
                || is_key_down(KeyCode::Down)
                || is_key_down(KeyCode::Up);

            if movement_is_happening {
                player.dir = vec2(0.0, 0.0);
            }

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
                (1.0 / (2.0 as f32).sqrt() * player.dir.x) * player.speed
            } else {
                player.dir.x * player.speed
            };

            let y_speed = if diag_move {
                (1.0 / (2.0 as f32).sqrt() * player.dir.y) * player.speed
            } else {
                player.dir.y * player.speed
            };

            if movement_is_happening {
                world.move_h(player.collider, x_speed * get_frame_time());
                world.move_v(player.collider, y_speed * get_frame_time());
            }

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
                if panda.state == PandaState::ReadyForDeletion {
                    continue;
                }

                if panda.state == PandaState::Grabbed {
                    if is_key_pressed(KeyCode::Space) {
                        player.state = PlayerState::Throwing;
                        panda.state = PandaState::Thrown;
                        panda.mover = Box::new(ThrownMover::new(player.dir));
                        play(&sfx_throw, false);
                    } else {
                        let player_pos = world.actor_pos(player.collider);
                        world.set_actor_position(panda.collider, player_pos + vec2(0., -5.));
                    }
                } else if panda.state == PandaState::FoundLove {
                    panda.apply_movement(&mut world);

                    if panda.mover.movement_complete() {
                        sfx_heart_counter = None;
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

                const GRAB_RANGE: f32 = 20.0;
                if (player_pos.x - panda_pos.x).abs() < GRAB_RANGE
                    && (player_pos.y - panda_pos.y).abs() < GRAB_RANGE
                    && is_key_pressed(KeyCode::Space)
                    && panda.state != PandaState::Grabbed
                    && player.state == PlayerState::Normal
                {
                    play(&sfx_pickup, false);
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

                for second_panda_index in first_panda_index + 1..num_of_pandas {
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
                sfx_heart_counter = Some(0);
                pandas[index].state = PandaState::FoundLove;
                pandas[index].mover = Box::new(LoveMover::new());
            }

            if let Some(heart_counter) = sfx_heart_counter {
                if heart_counter % 10 == 0 {
                    play(&sfx_heart, false);
                }

                sfx_heart_counter = Some(heart_counter + 1);

                if heart_counter > 1000 {
                    sfx_heart_counter = None;
                }
            }
        }

        // update bamboo count
        {
            if total_bamboo <= 0.0 {
                total_bamboo = 0.0;
            } else {
                let hungry_pandas = pandas
                    .iter()
                    .by_ref()
                    .filter(|p| p.state == PandaState::Normal)
                    .count();

                const HUNGER_RATE: f32 = 0.25;
                total_bamboo -= hungry_pandas as f32 * (HUNGER_RATE * get_frame_time());
            }
        }

        // update animation count
        {
            let f_dt = get_frame_time();
            for p in pandas.iter_mut() {
                p.frame_countdown -= f_dt;

                if p.frame_countdown <= 0.0 {
                    p.update_animation_indices();
                }
            }

            player.frame_countdown -= f_dt;

            if player.frame_countdown <= 0.0 {
                player.frame_countdown = 0.25;
                player.walk_anim_index += 1.0;

                if player.walk_anim_index == 4.0 {
                    player.walk_anim_index = 0.0;
                }
            }
        }

        set_default_camera();
        clear_background(GREEN);
        render_target.texture.set_filter(FilterMode::Nearest);
        draw_texture_ex(
            render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: true,
                ..Default::default()
            },
        );

        let text = format!("Remaining Bamboo: {}", total_bamboo as i32);

        draw_text_ex(
            &text,
            20.0,
            30.0,
            TextParams {
                font_size: 20,
                color: RED,
                font,
                ..Default::default()
            },
        );

        next_frame().await
    }
}

fn play(sound: &Sound, looped: bool) {
    // println!("Playing: {:?}", sound);
    audio::play_sound(
        *sound,
        audio::PlaySoundParams {
            looped,
            volume: 0.5,
        },
    );
}
