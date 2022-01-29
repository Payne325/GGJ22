use macroquad::prelude::*;
use macroquad_platformer::*;
use macroquad_tiled as tiled;
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
   pub tileTextures: Vec<Texture2D>,
   pub map: Vec<TileData>,
   pub collisionMap: Vec<bool>
}

impl TileMap {
   pub fn draw(&mut self) {
      for tile in &mut self.map {
         tile.draw(&self.tileTextures[tile.texture_index]);
      }
   }

}


pub async fn load_tilemap(path: &str, world: &mut World) -> TileMap {
   let mut tilemap = TileMap { tileTextures: Vec::new(), map: Vec::new(), collisionMap: Vec::new() };

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

         let tileStrings: Vec<&str> = line.split(",").collect();

         for string in tileStrings {
            let index = string.trim().parse().unwrap();

            let t = TileData { 
               collider: tilemap.collisionMap[index],
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
         let tileStrings: Vec<&str> = line.split(",").collect();
         tilemap.tileTextures.push(load_texture(tileStrings[0]).await.unwrap());

         if tileStrings[2].contains("true") {
            tilemap.collisionMap.push(true);
         } else {
            tilemap.collisionMap.push(false);
         }
      }

   }

   let mut static_colliders = vec![];

   for tile_data in &tilemap.map {
      if tile_data.collider {
         static_colliders.push(Tile::Solid);
      } else {
         static_colliders.push(Tile::Empty);
      }
   }

   world.add_static_tiled_layer(static_colliders, size.x, size.y, width, 1);
   println!("{}", width);

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

