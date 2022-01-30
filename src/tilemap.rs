use macroquad::prelude::*;
use macroquad_platformer::*;
// use macroquad_tiled as tiled;
use std::fs;

pub struct TileData {
   pub collider: bool,
   pub pos: Vec2,
   pub size: Vec2,
   pub texture_index: usize
}

impl TileData {
   pub fn draw(&mut self, texture: &Texture2D) {
      draw_texture_ex(*texture,
         self.pos.x,
         self.pos.y,
         WHITE,
         DrawTextureParams {
            dest_size: Some(self.size),
            source: None,
            ..Default::default()
        });
   }
}


pub struct TileMap {
   pub tile_textures: Vec<Texture2D>,
   pub map: Vec<TileData>,
   pub collision_map: Vec<bool>,

   map_size: Vec2
}

impl TileMap {
   pub fn draw(&mut self) {
      for tile in &mut self.map {
         tile.draw(&self.tile_textures[tile.texture_index]);
      }
   }

   pub fn is_collidable(&mut self, loc: Vec2) -> bool {
      let index = (loc.y * self.map_size.x + loc.x) as usize;

      return self.collision_map[self.map[index].texture_index];
   }

}


pub async fn load_tilemap(path: &str, world: &mut World) -> TileMap {
   let mut tilemap = TileMap { 
      tile_textures: Vec::new(),
      map: Vec::new(),
      collision_map: Vec::new(),
      map_size: Vec2::new(0.0, 0.0) };

   let file = fs::read_to_string(path).expect("Something went wrong reading the file");
   let lines = file.lines();

   let mut read_tile = false;
   let mut read_map = false;

   let mut loc = Vec2::new(0.0, 0.0);
   let size = Vec2::new(32.0, 32.0);

   let mut width = 0;
   let mut height = 0;

   for line in lines {
      if line.contains("#Tiles#") {
         read_tile = true;
         read_map = false;
         continue;
      } else if line.contains("#Map#") {
         read_tile = false;
         read_map = true;
         continue;
      }

      if read_map {
         width = 0;
         loc.x = 0.0;

         let tile_strings: Vec<&str> = line.split(",").collect();

         for string in tile_strings {
            let index = string.trim().parse().unwrap();

            let t = TileData { 
               collider: tilemap.collision_map[index],
               pos: loc,
               size: size,
               texture_index: index
            };

            tilemap.map.push(t);
            loc.x += size.x;
            width += 1;
         }
         
         loc.y += size.y;
         height += 1;

      } else if read_tile {
         let tile_strings: Vec<&str> = line.split(",").collect();
         tilemap.tile_textures.push(load_texture(tile_strings[0]).await.unwrap());

         if tile_strings[2].contains("true") {
            tilemap.collision_map.push(true);
         } else {
            tilemap.collision_map.push(false);
         }
      }

   }

   tilemap.map_size.x = width as f32;
   tilemap.map_size.y = height as f32;

   let mut static_colliders = vec![];

   for tile_data in &tilemap.map {
      if tile_data.collider {
         static_colliders.push(Tile::Solid);
      } else {
         static_colliders.push(Tile::Empty);
      }
   }

   world.add_static_tiled_layer(static_colliders, size.x, size.y, width, 1);
   println!("{} and {}", width, height);

   return tilemap;
}


/*
#[derive(PartialEq)]
pub enum PandaState {
   Normal,
   Grabbed,
   Thrown,
   FoundLove,
}

pub struct Panda {
   pub collider: Actor,
   pub speed: Vec2,
   pub mover: Box<dyn Mover>,
   pub state: PandaState
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
*/

