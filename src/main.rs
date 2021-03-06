mod mover;
mod panda_factory;
mod stork_factory;
mod tilemap;

use macroquad::audio::Sound;
use macroquad::audio::{self};
use macroquad::prelude::*;
use macroquad_platformer::*;
use macroquad::ui::{hash, root_ui, widgets::Window};
// use macroquad_tiled as tiled;

use mover::*;
use panda_factory::*;
use stork_factory::*;

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

fn conf() -> Conf {
    Conf {
        window_title: String::from("Panda Date Arcade"),
        window_width: 1920,
        window_height: 1080,
        fullscreen: true,
        ..Default::default()
    }
}

fn add_bamboo(bamboo_collection: &mut f32, bamboo_to_add: f32, bamboo_points: &mut Vec<Vec2>) {
    *bamboo_collection += bamboo_to_add;

    for _ in 0..bamboo_to_add as usize {
        bamboo_points.push(get_random_game_point());
    }
}

fn remove_bamboo(
    bamboo_collection: &mut f32,
    bamboo_to_remove: f32,
    bamboo_points: &mut Vec<Vec2>,
) {
    *bamboo_collection -= bamboo_to_remove;

    let diff = (*bamboo_collection - bamboo_points.len() as f32).round();

    if diff < 0f32 {
        bamboo_points.drain(0..diff.abs() as usize);
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut player_score = 0;
    let mut elapsed_time = 0.0;

    let track1 =
        audio::load_sound("assets/Panda Dating Simulator - Turbo Arcade Edition Loop (127bpm).wav")
            .await
            .unwrap();

    play(&track1, true, 0.4);

    let sfx_heart = audio::load_sound("assets/sfx_heart.wav").await.unwrap();
    let sfx_impact = audio::load_sound("assets/sfx_impact.wav").await.unwrap();
    let sfx_pickup = audio::load_sound("assets/sfx_pickup.wav").await.unwrap();
    let sfx_throw = audio::load_sound("assets/sfx_throw.wav").await.unwrap();
    let mut sfx_loop_threshold = 0.0;
    let mut is_love_making = false;

    let font = load_ttf_font("./assets/Gameplay.ttf").await.unwrap();

    let panda_walking_texture = load_texture("assets/walking_panda-export2.png")
        .await
        .unwrap();
    let panda_thrown_texture = load_texture("assets/thrown_panda-export2.png")
        .await
        .unwrap();
    let panda_love_texture = load_texture("assets/dancing_panda-export2.png")
        .await
        .unwrap();
    let panda_dead_texture = load_texture("assets/dead_panda.png").await.unwrap();

    let stork_loaded_texture = load_texture("assets/stork_loaded.png").await.unwrap();
    let stork_unloaded_texture = load_texture("assets/stork_unloaded.png").await.unwrap();

    let player_walking_texture = load_texture("assets/walking_cupid_panda-export2.png")
        .await
        .unwrap();
    let player_grabbing_texture = load_texture("assets/walking_cupid_panda_black-export2.png")
        .await
        .unwrap();

    let heart_texture = load_texture("assets/heart.png").await.unwrap();

    let mut total_bamboo = 100.0;
    let bamboo_texture = load_texture("assets/bamboo.png").await.unwrap();
    let mut bamboo_points = Vec::new();
    for _ in 0..total_bamboo as usize {
        bamboo_points.push(get_random_game_point());
    }

    let material =
        load_material(CRT_VERTEX_SHADER, CRT_FRAGMENT_SHADER, Default::default()).unwrap();
    let mut world = World::new();
    let mut tilemap = tilemap::load_tilemap("assets/map.txt", &mut world).await;

    const THROW_COOLDOWN: f32 = 2.0;
    let mut player = Player {
        collider: world.add_actor(vec2(32.0, 150.0), 10, 10),
        speed: 100.0,
        dir: vec2(0.0, 0.0),
        state: PlayerState::Normal,
        throw_cooldown: THROW_COOLDOWN,
        walk_anim_index: 0.0,
        frame_countdown: 0.05,
    };

    let mut pandas = Vector::<Panda>::new();
    let mut storks = Vector::<Stork>::new();

    println!("w:{}, h:{}", screen_width(), screen_height());

    pandas.push(PandaFactory::create_panda(&mut world));
    pandas.push(PandaFactory::create_panda(&mut world));

    let map_screen_width = 1920.0 / 4.0;
    let mut camera =
        Camera2D::from_display_rect(Rect::new(0.0, 15.0, map_screen_width, 1080.0 / 4.0));
    let render_target = render_target(map_screen_width as u32, 1080 / 4);

    const PANDA_LOVING_COOLDOWN_SECONDS: f32 = 3.0;
    const PANDA_INDEPENDANT_DEATH_RATE_SECONDS: f64 = 20.0;

    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }

        let delta_time = get_frame_time();
        elapsed_time += delta_time;

        camera.render_target = Some(render_target);
        set_camera(&camera);

        // draw map
        tilemap.draw();

        for bam in &bamboo_points {
            draw_texture_ex(
                bamboo_texture,
                bam.x,
                bam.y,
                WHITE,
                DrawTextureParams {
                    ..Default::default()
                },
            );
        }

        //   panda_spawn_countdown -= delta_time;

        //   if panda_spawn_countdown <= 0.0 {
        //      panda_spawn_countdown = PANDA_LOVING_COOLDOWN_SECONDS;
        //      pandas.push(PandaFactory::create_panda(&mut world));
        //   }

        is_love_making = false;

        // draw pandas
        {
            for panda in &pandas {
                let pos = world.actor_pos(panda.collider);
                if panda.state == PandaState::Dead {
                    draw_texture_ex(
                        panda_dead_texture,
                        pos.x - 12.0,
                        pos.y - 15.0,
                        WHITE,
                        DrawTextureParams {
                            ..Default::default()
                        },
                    );
                    continue;
                }

                if panda.state == PandaState::Thrown {
                    draw_texture_ex(
                        panda_thrown_texture,
                        pos.x - 5.0,
                        pos.y - 15.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(32.0, 32.0)),
                            source: Some(Rect::new(
                                32.0 * panda.thrown_anim_index,
                                0.0,
                                32.0,
                                32.0,
                            )),
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
                    is_love_making = true;
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

        // handle not enough pandas for early exit
        {
         let alive_pandas = pandas
            .iter()
            .by_ref()
            .filter(|p| p.state != PandaState::Dead)
            .count();

            if alive_pandas <= 1 {
              if game_over(false) {
                return;
              }
            }
        }

        // draw storks
        {
            for stork in &mut storks {
                if stork.state == StorkState::Loaded {
                    draw_texture_ex(
                        stork_loaded_texture,
                        stork.pos.x,
                        stork.pos.y,
                        WHITE,
                        DrawTextureParams {
                            flip_x: stork.speed.x < 0.0,
                            dest_size: Some(vec2(32.0, 32.0)),
                            source: Some(Rect::new(32.0 * stork.anim_index, 0.0, 32.0, 32.0)),
                            ..Default::default()
                        },
                    );
                } else {
                    draw_texture_ex(
                        stork_unloaded_texture,
                        stork.pos.x,
                        stork.pos.y,
                        WHITE,
                        DrawTextureParams {
                            flip_x: stork.speed.x < 0.0,
                            dest_size: Some(vec2(32.0, 32.0)),
                            source: Some(Rect::new(32.0 * stork.anim_index, 0.0, 32.0, 32.0)),
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
                world.move_h(player.collider, x_speed * delta_time);
                world.move_v(player.collider, y_speed * delta_time);
            }

            if player.state == PlayerState::Throwing {
                player.throw_cooldown -= delta_time;

                if player.throw_cooldown < 0.0 {
                    player.throw_cooldown = THROW_COOLDOWN;
                    player.state = PlayerState::Normal;
                }
            }
        }

        // panda movement
        {
            for panda in &mut pandas {
                if panda.state == PandaState::Dead {
                    continue;
                }

                if panda.state == PandaState::Grabbed {
                    if is_key_pressed(KeyCode::Space) {
                        player.state = PlayerState::Throwing;
                        panda.state = PandaState::Thrown;
                        panda.mover = Box::new(ThrownMover::new(player.dir));
                        play(&sfx_throw, false, 0.8);
                    } else {
                        let player_pos = world.actor_pos(player.collider);
                        world.set_actor_position(panda.collider, player_pos + vec2(0., -5.));
                    }
                } else if panda.state == PandaState::FoundLove {
                    panda.apply_movement(&mut world);

                    if panda.mover.movement_complete() {
                        panda.state = PandaState::Normal;
                        panda.mover = Box::new(NormalMover::new());

                        let speed_x = rand::gen_range(0.0, 50.0);
                        let speed_y = rand::gen_range(0.0, 50.0);
                        panda.speed = vec2(speed_x, speed_y);
                        panda.sweet_panda_loving_cooldown = PANDA_LOVING_COOLDOWN_SECONDS;
                    }
                } else {
                    panda.apply_movement(&mut world);

                    if panda.mover.movement_complete() {
                        panda.state = PandaState::Normal;
                        panda.mover = Box::new(NormalMover::new());

                        let speed_x = rand::gen_range(0.0, 50.0);
                        let speed_y = rand::gen_range(0.0, 50.0);
                        panda.speed = vec2(speed_x, speed_y);
                    }
                }
            }
        }

        // move storks
        {
            for stork in &mut storks {
                if stork.apply_movement(get_frame_time()) {
                    pandas.push(PandaFactory::create_panda_at(&mut world, stork.pos));
                }

                stork.update_animation(get_frame_time());
            }
        }

        // collision detection
        {
            // detect player grabbing pandas
            for panda in &mut pandas {
                if panda.state != PandaState::Normal {
                    continue;
                }

                let player_pos = world.actor_pos(player.collider);
                let panda_pos = world.actor_pos(panda.collider);

                const GRAB_RANGE: f32 = 20.0;
                if (player_pos.x - panda_pos.x).abs() < GRAB_RANGE
                    && (player_pos.y - panda_pos.y).abs() < GRAB_RANGE
                    && is_key_pressed(KeyCode::Space)
                    && panda.state != PandaState::Grabbed
                    && player.state == PlayerState::Normal
                {
                    play(&sfx_pickup, false, 0.8);
                    panda.state = PandaState::Grabbed;
                    player.state = PlayerState::Grabbing;
                }
            }

            // detect pandas finding other pandas
            let mut in_love_indices = Vector::<usize>::new();

            let num_of_pandas = pandas.len();
            for first_panda_index in 0..(num_of_pandas - 1) {
                let first_panda = &pandas[first_panda_index];

                if first_panda.state != PandaState::Normal
                    || in_love_indices.contains(&first_panda_index)
                    || first_panda.sweet_panda_loving_cooldown > 0.0
                {
                    continue;
                }

                let first_panda_pos = world.actor_pos(first_panda.collider);

                for second_panda_index in (first_panda_index + 1)..num_of_pandas {
                    let second_panda = &pandas[second_panda_index];

                    if second_panda.state != PandaState::Normal
                        || in_love_indices.contains(&second_panda_index)
                        || second_panda.sweet_panda_loving_cooldown > 0.0
                    {
                        continue;
                    }

                    let second_panda_pos = world.actor_pos(second_panda.collider);

                    const HUBBA_HUBBA_RANGE: f32 = 32.0;

                    let val = (first_panda_pos.x - second_panda_pos.x).abs();
                    let val2 = (first_panda_pos.y - second_panda_pos.y).abs();

                    if val < HUBBA_HUBBA_RANGE && val2 < HUBBA_HUBBA_RANGE {
                        in_love_indices.push(first_panda_index);
                        in_love_indices.push(second_panda_index);
                        storks.push(StorkFactory::create_stork(
                            first_panda_pos,
                            map_screen_width,
                        ));
                        player_score += 50;
                        break;
                    }
                }
            }

            for index in in_love_indices {
                pandas[index].state = PandaState::FoundLove;
                pandas[index].mover = Box::new(LoveMover::new());
            }
        }

        // detect pandas dead of old age
        {
            for p in pandas.iter_mut() {
                if p.state == PandaState::Grabbed {
                    continue;
                }

                if get_time() - p.spawn_time > PANDA_INDEPENDANT_DEATH_RATE_SECONDS {
                    p.state = PandaState::Dead;
                }
            }

            // then remove them
            //pandas = pandas.into_iter().filter(|p| p.state != PandaState::ReadyForDeletion).collect();
        }

        // update bamboo count
        {
            if total_bamboo <= 0.0 {
                total_bamboo = 0.0; 
                
                if game_over(true) {
                   return;
                }
            } else {
                let hungry_pandas = pandas
                    .iter()
                    .by_ref()
                    .filter(|p| p.state == PandaState::Normal)
                    .count();

                const HUNGER_RATE: f32 = 0.25;
                let eaten_bamboo = hungry_pandas as f32 * (HUNGER_RATE * delta_time);
                remove_bamboo(&mut total_bamboo, eaten_bamboo, &mut bamboo_points);

                total_bamboo -= eaten_bamboo;
            }

            const BAMBOO_REFRESH_TIME_SECONDS: f32 = 10.0;
            const BAMBOO_TO_ADD: f32 = 10.0;
            if elapsed_time > BAMBOO_REFRESH_TIME_SECONDS {
                elapsed_time = 0.0;
                add_bamboo(&mut total_bamboo, BAMBOO_TO_ADD, &mut bamboo_points)
            }
        }

        // update animation count and lovin-cooldown
        {
            for p in pandas.iter_mut() {
                if p.state != PandaState::Dead {
                    p.sweet_panda_loving_cooldown -= delta_time;

                    if p.sweet_panda_loving_cooldown < 0.0 {
                        p.sweet_panda_loving_cooldown = 0.0;
                    }

                    p.frame_countdown -= delta_time;

                    if p.frame_countdown <= 0.0 {
                        p.update_animation_indices();
                    }
                }
            }

            player.frame_countdown -= delta_time;

            if player.frame_countdown <= 0.0 {
                player.frame_countdown = 0.25;
                player.walk_anim_index += 1.0;

                if player.walk_anim_index == 4.0 {
                    player.walk_anim_index = 0.0;
                }
            }
        }

        // SFX looping
        {
            if is_love_making {
                sfx_loop_threshold += macroquad::time::get_frame_time();
                if sfx_loop_threshold > 0.20 {
                    play(&sfx_heart, false, 0.8);
                    sfx_loop_threshold = 0.0;
                }
            }
        }

        set_default_camera();
        clear_background(GREEN);
        render_target.texture.set_filter(FilterMode::Nearest);
        gl_use_material(material);
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
        gl_use_default_material();

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

        let score_text = format!("Score: {}", player_score as i32);

        draw_text_ex(
            &score_text,
            20.0,
            60.0,
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

fn game_over(ran_out_of_bamboo: bool) -> bool {
   let msg = if ran_out_of_bamboo {
      "You ran out of Bamboo and the Pandas shall perish!"
   }
   else {
      "You ran out of Pandas and the Bamboo shall conquer the Earth!"
   };

   let dialog_size = vec2(440., 100.);
   let screen_size = vec2(screen_width(), screen_height());
   let dialog_position = screen_size / 2. - dialog_size / 2.;
   let mut user_response = false;
   Window::new(hash!(), dialog_position, dialog_size).ui(&mut *root_ui(), |ui| {
       ui.label(None, &msg);
       ui.separator();
       ui.same_line(480.);
       if ui.button(None, "Goodbye") {
         user_response = true;
       }
    });

   user_response
}


fn get_random_game_point() -> Vec2 {
    let x: f32 = macroquad::rand::gen_range(16.0, 480.0 - 32.0 - 16.0);
    let y: f32 = macroquad::rand::gen_range(16.0, 270.0 - 16.0 - 16.0);
    vec2(x.floor(), y.floor())
}

fn play(sound: &Sound, looped: bool, volume: f32) {
    // println!("Playing: {:?}", sound);
    audio::play_sound(
        *sound,
        audio::PlaySoundParams {
            looped,
            volume: volume,
        },
    );
}

const CRT_FRAGMENT_SHADER: &'static str = r#"#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;
    
uniform sampler2D Texture;

// https://www.shadertoy.com/view/XtlSD7

vec2 CRTCurveUV(vec2 uv)
{
    uv = uv * 2.0 - 1.0;
    vec2 offset = abs( uv.yx ) / vec2( 12.0, 4.0 );
    uv = uv + uv * offset * offset;
    uv = uv * 0.5 + 0.5;
    return uv;
}

void DrawVignette( inout vec3 color, vec2 uv )
{    
    float vignette = uv.x * uv.y * ( 1.0 - uv.x ) * ( 1.0 - uv.y );
    vignette = clamp( pow( 50.0 * vignette, 0.2 ), 0.0, 1.0 );
    color *= vignette * 1.0;
}


void DrawScanline( inout vec3 color, vec2 uv )
{
    float iTime = 0.9;
    float scanline 	= clamp( 0.95 + 0.05 * cos( 3.14 * ( uv.y + 0.008 * iTime ) * 1080.0 * 0.4 ), 0.0, 1.0 );
    float grille 	= 1.0 + 0.15 * clamp( 1.0 * cos( 3.14159265359 * uv.x * 1920.0 * 0.4 ), 0.2, 1.0 );    
    color *= scanline * grille * 1.1;
}

void main() {
    
    vec2 crtUV = CRTCurveUV(uv);
    
    vec3 res = texture2D(Texture, uv).rgb * color.rgb;
 	
    if (crtUV.x < 0.0 || crtUV.x > 1.0 || crtUV.y < 0.0 || crtUV.y > 1.0)
    {
        res = vec3(0.0, 0.0, 0.0);
    } 
    DrawVignette(res, crtUV);
    DrawScanline(res, uv);
    gl_FragColor = vec4(res, 1.0);

}
"#;

const CRT_VERTEX_SHADER: &'static str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying lowp vec2 uv;
varying lowp vec4 color;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    color = color0 / 255.0;
    uv = texcoord;
}
";
