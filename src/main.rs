mod serialize;
mod from_stream;

use flate2::read::GzDecoder;
use from_stream::{i16_fs, i32_fs, f32_fs, i64_fs, str_fs, u16_fs, u32_fs, u64_fs};
use serialize::{Deserializer,DeserializeError};

use std::fs::read;
use std::io::Read;

//use thiserror::Error;

use mc_classic_js as js;


fn main() {

    let mut input: String = String::from("test/Classic Levels/level_30_s.mine");
    //let output: String = String::from("test/data.sqlite");
    let mut level: Level = read_level(input);
    input = String::from("test/Classic Levels/level_14a_08.dat");
    level = read_level(input);
    input = String::from("test/Classic Levels/level_25_05_st.dat");
    level = read_level(input);
    println!("File is read");

    return;

    println!("Iterating through {} iterations", level.blocks.clone().unwrap().len());
    let tile_map: Vec<u8> = classic_id_to_js_id(level.blocks.clone().unwrap());
    println!("Tiles are converted");
    println!("Level Author: {}", level.creator.clone().unwrap());

    let json_string: String = js::serialize_saved_game_from_seed(1, tile_map);
    println!("Json is prepped");
    //js::write_saved_game(output, json_string).expect("Uh oh!");
    println!("File has been written");


    //read_level(file);
}

#[derive(Default)]
pub struct Level {
    blocks: Option<Vec<u8>>, //pc-132211
    version: Option<u8>, //0.0.13a-dev
    name: Option<String>, //0.0.13a-dev
    creator: Option<String>, //0.0.13a-dev
    createTime: Option<i64>, //0.0.13a-dev
    width: Option<i32>, //0.0.13a-dev
    height: Option<i32>, //0.0.13a-dev
    depth: Option<i32>, //0.0.13a-dev
    xSpawn: Option<i32>, //0.0.14a_08
    ySpawn: Option<i32>, //0.0.14a_08
    zSpawn: Option<i32>, //0.0.14a_08
    rotSpawn: Option<f32>, //0.0.14a_08
    tickCount: Option<i32>, //0.0.14a_08
    unprocessed: Option<i32>, //0.0.14a_08
    entities: Option<Vec<Entity>>, //0.0.14a_08 - Removed 0.25_05_st
    networkMode: Option<bool>, //0.0.19a_04
    cloudColor: Option<i32>, //0.0.25_05_st
    fogColor: Option<i32>, //0.0.25_05_st
    skyColor: Option<i32>, //0.0.25_05_st
    waterLevel: Option<i32>, //0.0.25_05_st
    player: Option<Entity>, //0.0.25_05_st
    //blockMap was added and the only thing of use it holds is the entity list
    //blockMap therefore just gets parsed into entities
    //Note the player is not included in this list when parsed
    //blockMap: Option<BlockMap> //0.0.25_05_st
    creativeMode: Option<bool>, //0.0.28_01
    growTrees: Option<bool> //0.0.29
}

impl Level {
    pub fn new () -> Self {
        Level {
            blocks: None,
            version: None,
            name: None,
            creator: None,
            createTime: None,
            width: None,
            height: None,
            depth: None,
            xSpawn: None,
            ySpawn: None,
            zSpawn: None,
            rotSpawn: None,
            tickCount: None,
            unprocessed: None,
            entities: None,
            networkMode: None,
            cloudColor: None,
            fogColor: None,
            skyColor: None,
            waterLevel: None,
            player: None,
            creativeMode: None,
            growTrees: None
        }
    }
}

pub struct Entity {

}

pub struct Player {

}

fn read_level (file: String) -> Level {
    //Reading in a classic level and converting it to a decompressed stream of bytes
    let stream: Vec<u8> = read(file).unwrap();
    let mut d_stream = GzDecoder::new(&stream[..]);
    let mut bytes: Vec<u8> = Vec::new();
    d_stream.read_to_end(&mut bytes).unwrap();

    //Checking for a magic number at the start of the file
    let magic_number: u32 = u32_fs(0, &bytes[..]);

    println!("{magic_number}");

    if magic_number != 0x271BB788 {return pre_classic_to_level(bytes)}
    if bytes[4] == 1 {return classic_13_to_level(bytes)}
    if bytes[4] == 2 {return classic_to_level(bytes).expect("Classic level broke :( ")}

    return Level::new();

}

//Pre-classic saves only store an array of blocks, no other information
pub fn pre_classic_to_level (bytes: Vec<u8>) -> Level {

    let mut level: Level = Level::new();
    level.blocks = Some(bytes);
    return level;
    
}

pub fn classic_13_to_level (bytes: Vec<u8>) -> Level {
    let mut buf: usize = 4;
    let mut level: Level = Level::new();

    //Setting level version - Byte format
    level.version = Some(bytes[buf]);
    buf += 1;

    //Parsing and setting level name - String format
    let mut sh: u16 = u16_fs(buf, &bytes[..]);
    level.name = Some(str_fs(buf, &bytes[..], sh as i32));
    buf += 2 + sh as usize;

    //Parsing and setting author name - String format
    sh = u16_fs(buf, &bytes[..]);
    level.creator = Some(str_fs(buf, &bytes[..], sh as i32));
    buf += 2 + sh as usize;

    //Setting timestamp - Long format
    level.createTime = Some(i64_fs(buf, &bytes[..]));
    buf += 2;

    //Setting width, depth, and height - Short Format
    level.width = Some(i16_fs(buf, &bytes[..]) as i32);
    buf += 2;
    level.height = Some(i16_fs(buf, &bytes[..]) as i32);
    buf += 2;
    level.depth = Some(i16_fs(buf, &bytes[..]) as i32);
    buf += 2;

    println!("x: {} y: {} z: {}", level.width.unwrap(),level.height.unwrap(),level.depth.unwrap());

    //Setting tile map - Array in the format of x -> z -> y
    let mut tile_map: Vec<u8> = Vec::new();
    for i in buf..bytes.len() as usize {
        tile_map.push(bytes[i as usize]);
    }

    level.blocks = Some(tile_map);

    return level;

}

pub fn classic_to_level (bytes: Vec<u8>) -> Result<Level, DeserializeError> {

    let mut buf: usize = 4;
    let mut level: Level = Level::new();

    level.version = Some(bytes[buf as usize]);
    buf += 1;

   println!("{}",&bytes[buf..].len());

    let mut deserializer: Deserializer = Deserializer::new();
    let contents: Vec<serialize::Content> = deserializer.deserialize(&bytes[buf..])?;
    println!("Contents: {}", contents.len());

    if contents.len() != 1 { return Err(DeserializeError::InvalidContentLength(1, contents.len())) }

    let object: serialize::NewObject = contents[0].get_object()?.get_new_object()?;

    let class_info: serialize::NewClassDesc = object.class_desc.get_new_class_desc()?;
    let class_data: serialize::ClassData = object.class_data.unwrap();

    if class_info.get_class_name()? != "com.mojang.minecraft.level.Level" {
        return Err(DeserializeError::InvalidClass(class_info.get_class_name()?.clone()))
    }

    let fields: serialize::Fields = class_info.get_class_desc_info()?.unwrap().fields;
    let values: Vec<serialize::Value> = class_data.get_values()?;

    for i in 0..fields.count as usize {
        match fields.field_descs[i].get_field_name()?.as_str() {
            "createTime" => { level.createTime = Some(values[i].get_long()?) },
            "depth" => { level.depth = Some(values[i].get_integer()?) },
            "height" => { level.height = Some(values[i].get_integer()?) },
            "rotSpawn" => { level.rotSpawn = Some(values[i].get_float()?) },
            "tickCount" => { level.tickCount = Some(values[i].get_integer()?) },
            "unprocessed" => { level.unprocessed = Some(values[i].get_integer()?) },
            "width" => { level.width = Some(values[i].get_integer()?) },
            "xSpawn" => { level.xSpawn = Some(values[i].get_integer()?) },
            "ySpawn" => { level.zSpawn = Some(values[i].get_integer()?) },
            "zSpawn" => { level.ySpawn = Some(values[i].get_integer()?) },
            "networkMode" => { level.networkMode = Some(values[i].get_boolean()?) },
            "cloudColor" => { level.cloudColor = Some(values[i].get_integer()?) },
            "fogColor" => { level.fogColor = Some(values[i].get_integer()?) },
            "skyColor" => { level.skyColor = Some(values[i].get_integer()?) },
            "waterLevel" => { level.waterLevel = Some(values[i].get_integer()?) },
            "creativeMode" => { level.creativeMode = Some(values[i].get_boolean()?) },
            "growTrees" => { level.growTrees = Some(values[i].get_boolean()?) },
            "blocks" => { 
                let wrapped: Vec<serialize::Value> = values[i].get_array()?; 
                let mut blocks: Vec<u8> = Vec::new();
                for value in wrapped {
                    blocks.push(value.get_byte()?)
                }
                level.blocks = Some(blocks);
            },
            "creator" => { 
                level.creator = values[i].get_object()?.get_new_string()?.string; 
            },
            "name" => { 
                level.creator = values[i].get_object()?.get_new_string()?.string; 
            },
            "entities" => { //ADD SUPPORT FOR ARRAY LIST TYPES IN THE FUTURE!!
                () 
            }, 
            "blockMap" => { //ADD SUPPORT FOR PARSING BLOCKMAPS INTO ENTITY LISTS & PLAYER
                //println!("{:?}", values[i].get_object())  
            }, 
            "player" => { //ADD SUPPORT FOR PARSING THE PLAYER
                println!("{:?}", values[i].get_object()) 
            }, 
            _ => println!("Unexpected Field: {}", fields.field_descs[i].get_field_name()?.as_str())

        }
    }

    println!("HOLY CRAP SWEET LORD ABOVE IT ACTUALLY WORKED YOU INSANE IDIOT!!!!!!!");

    Ok(level)
}

pub fn classic_id_to_js_id (tile_map: Vec<u8>) -> Vec<u8> {
    let mut i: usize = 0;
    let mut tile_map1 = tile_map.clone();
    for tile in tile_map1.clone() {
        
        match tile {
            1 => tile_map1[i] = 2, //Stone
            2 => tile_map1[i] = 1, //Grass Block
            4 => tile_map1[i] = 9, //Cobblestone
            5 => tile_map1[i] = 4, //Planks
            6 => tile_map1[i] = 8, //Sapling
            7 => tile_map1[i] = 10, //Bedrock
            8 => tile_map1[i] = 7, //Flowing Water
            9 => tile_map1[i] = 7, //Stationary Water
            10 => tile_map1[i] = 17, //Flowing Lava
            11 => tile_map1[i] = 17, //Stationary Lava
            12 => tile_map1[i] = 11, //Sand
            13 => tile_map1[i] = 12, //Gravel
            14 => tile_map1[i] = 18, //Gold Ore
            15 => tile_map1[i] = 19, //Iron Ore
            16 => tile_map1[i] = 20, //Coal Ore
            17 => tile_map1[i] = 13, //Logs
            18 => tile_map1[i] = 14, //Leaves
            19 => tile_map1[i] = 22, //Sponge
            20 => tile_map1[i] = 23, //Glass
            21 => tile_map1[i] = 24, //Red Cloth
            22 => tile_map1[i] = 25, //Orange Cloth
            23 => tile_map1[i] = 26, //Yellow Cloth
            24 => tile_map1[i] = 27, //Chartreuse Cloth
            25 => tile_map1[i] = 28, //Green Cloth
            26 => tile_map1[i] = 29, //Spring Green Cloth
            27 => tile_map1[i] = 30, //Cyan Cloth
            28 => tile_map1[i] = 31, //Capri Cloth
            29 => tile_map1[i] = 32, //Ultramarine Cloth
            30 => tile_map1[i] = 34, //Purple Cloth
            31 => tile_map1[i] = 33, //Violet Cloth
            32 => tile_map1[i] = 35, //Magenta Cloth
            33 => tile_map1[i] = 36, //Rose Cloth
            34 => tile_map1[i] = 37, //Dark Gray Cloth
            35 => tile_map1[i] = 38, //Light Gray Cloth
            36 => tile_map1[i] = 39, //White Cloth
            37 => tile_map1[i] = 6, //Dandelion
            38 => tile_map1[i] = 5, //Rose
            39 => tile_map1[i] = 16, //Brown Mushroom
            40 => tile_map1[i] = 15, //Red Mushroom
            41 => tile_map1[i] = 21, //Block of Gold
            42 => tile_map1[i] = 0, //Block of Iron
            43 => tile_map1[i] = 0, //Double Slab
            44 => tile_map1[i] = 0, //Slab
            45 => tile_map1[i] = 0, //Bricks
            46 => tile_map1[i] = 0, //TNT
            47 => tile_map1[i] = 0, //Bookshelf
            48 => tile_map1[i] = 0, //Mossy Cobblestone
            49 => tile_map1[i] = 0, //Obsidian
            _ => ()
        }

        i += 1;
        //println!("{i}");
    }

    return tile_map1;
}
