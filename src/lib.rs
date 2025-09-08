/*
TO DO: Add Write Support
*/

mod serialize;
mod from_stream;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use from_stream::{i16_fs, i64_fs, str_fs, u16_fs, u32_fs};
use serialize::{Deserializer,DeserializeError};

use std::fs::{read, File, OpenOptions};
use std::io::{Read, Write};

use thiserror::Error;

#[derive(Default,Clone)]
pub struct Level {
    pub blocks: Option<Vec<u8>>, //pc-132211
    pub version: Option<u8>, //0.0.13a-dev
    pub name: Option<String>, //0.0.13a-dev
    pub creator: Option<String>, //0.0.13a-dev
    pub createTime: Option<i64>, //0.0.13a-dev
    pub width: Option<i32>, //0.0.13a-dev
    pub height: Option<i32>, //0.0.13a-dev
    pub depth: Option<i32>, //0.0.13a-dev
    pub xSpawn: Option<i32>, //0.0.14a_08
    pub ySpawn: Option<i32>, //0.0.14a_08
    pub zSpawn: Option<i32>, //0.0.14a_08
    pub rotSpawn: Option<f32>, //0.0.14a_08
    pub tickCount: Option<i32>, //0.0.14a_08
    pub unprocessed: Option<i32>, //0.0.14a_08
    pub entities: Option<Vec<Entity>>, //0.0.14a_08 - Removed 0.25_05_st
    pub networkMode: Option<bool>, //0.0.19a_04
    pub cloudColor: Option<i32>, //0.0.25_05_st
    pub fogColor: Option<i32>, //0.0.25_05_st
    pub skyColor: Option<i32>, //0.0.25_05_st
    pub waterLevel: Option<i32>, //0.0.25_05_st
    pub player: Option<Player>, //0.0.25_05_st
    //blockMap was added and the only thing of use it holds is the entity list
    //blockMap therefore just gets parsed into entities
    //Note the player is not included in this list when parsed
    //blockMap: Option<BlockMap> //0.0.25_05_st
    pub creativeMode: Option<bool>, //0.0.28_01
    pub growTrees: Option<bool> //0.0.29
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

#[derive(Clone)]
pub enum Entity {
    Sheep(Sheep),
    Pig(Pig),
    Creeper(Creeper),
    Zombie(Zombie),
    Skeleton(Skeleton),
    Spider(Spider),
    Item(Item),
    Arrow(Arrow),
    PrimedTnt(PrimedTnt),
    Sign(Sign),
    Smolder(Smolder),
    Player(Player)
}

impl Entity {
    pub fn get_sheep (&self) -> Result<Sheep,ClassicError> {
        match self {
            Entity::Sheep(value) => Ok(value.clone()),
            _ => Err(ClassicError::InvalidEntity())
        }
    }

    pub fn get_pig (&self) -> Result<Pig,ClassicError> {
        match self {
            Entity::Pig(value) => Ok(value.clone()),
            _ => Err(ClassicError::InvalidEntity())
        }
    }

    pub fn get_creeper (&self) -> Result<Creeper,ClassicError> {
        match self {
            Entity::Creeper(value) => Ok(value.clone()),
            _ => Err(ClassicError::InvalidEntity())
        }
    }

    pub fn get_zombie (&self) -> Result<Zombie,ClassicError> {
        match self {
            Entity::Zombie(value) => Ok(value.clone()),
            _ => Err(ClassicError::InvalidEntity())
        }
    }

    pub fn get_skeleton (&self) -> Result<Skeleton,ClassicError> {
        match self {
            Entity::Skeleton(value) => Ok(value.clone()),
            _ => Err(ClassicError::InvalidEntity())
        }
    }

    pub fn get_spider (&self) -> Result<Spider,ClassicError> {
        match self {
            Entity::Spider(value) => Ok(value.clone()),
            _ => Err(ClassicError::InvalidEntity())
        }
    }

    pub fn get_item (&self) -> Result<Item,ClassicError> {
        match self {
            Entity::Item(value) => Ok(value.clone()),
            _ => Err(ClassicError::InvalidEntity())
        }
    }

    pub fn get_arrow (&self) -> Result<Arrow,ClassicError> {
        match self {
            Entity::Arrow(value) => Ok(value.clone()),
            _ => Err(ClassicError::InvalidEntity())
        }
    }

    pub fn get_primed_tnt (&self) -> Result<PrimedTnt,ClassicError> {
        match self {
            Entity::PrimedTnt(value) => Ok(value.clone()),
            _ => Err(ClassicError::InvalidEntity())
        }
    }

    pub fn get_sign (&self) -> Result<Sign,ClassicError> {
        match self {
            Entity::Sign(value) => Ok(value.clone()),
            _ => Err(ClassicError::InvalidEntity())
        }
    }

    pub fn get_smolder (&self) -> Result<Smolder,ClassicError> {
        match self {
            Entity::Smolder(value) => Ok(value.clone()),
            _ => Err(ClassicError::InvalidEntity())
        }
    }

    pub fn get_player (&self) -> Result<Player,ClassicError> {
        match self {
            Entity::Player(value) => Ok(value.clone()),
            _ => Err(ClassicError::InvalidEntity())
        }
    }

}

#[derive(Clone)]
pub struct EntityFields {
    pub bbHeight: Option<f32>,
    pub bbWidth: Option<f32>,
    pub collision: Option<bool>,
    pub fallDistance: Option<f32>,
    pub footSize: Option<f32>,
    pub heightOffset: Option<f32>,
    pub horizontalCollision: Option<bool>,
    pub hovered: Option<bool>,
    pub makeStepSound: Option<bool>,
    pub nextStep: Option<i32>,
    pub noPhysics: Option<bool>,
    pub onGround: Option<bool>,
    pub pushthrough: Option<f32>,
    pub removed: Option<bool>,
    pub slide: Option<bool>,
    pub textureId: Option<i32>,
    pub walkDist: Option<f32>,
    pub walkDistO: Option<f32>,
    pub x: Option<f32>,
    pub xOld: Option<f32>,
    pub xRot: Option<f32>,
    pub xRotO: Option<f32>,
    pub xd: Option<f32>,
    pub xo: Option<f32>,
    pub y: Option<f32>,
    pub yOld: Option<f32>,
    pub yRot: Option<f32>,
    pub yRotO: Option<f32>,
    pub ySlideOffset: Option<f32>,
    pub yd: Option<f32>,
    pub yo: Option<f32>,
    pub z: Option<f32>,
    pub zOld: Option<f32>,
    pub zd: Option<f32>,
    pub zo: Option<f32>,
    pub bb: Option<AABB>
}

impl EntityFields {
    pub fn new() -> Self {
        EntityFields { 
            bbHeight: None,
            bbWidth: None,
            collision: None,
            fallDistance: None,
            footSize: None,
            heightOffset: None,
            horizontalCollision: None,
            hovered: None,
            makeStepSound: None,
            nextStep: None,
            noPhysics: None,
            onGround: None,
            pushthrough: None,
            removed: None,
            slide: None,
            textureId: None,
            walkDist: None,
            walkDistO: None,
            x: None,
            xOld: None,
            xRot: None,
            xRotO: None,
            xd: None,
            xo: None,
            y: None,
            yOld: None,
            yRot: None,
            yRotO: None,
            ySlideOffset: None,
            yd: None,
            yo: None,
            z: None,
            zOld: None,
            zd: None,
            zo: None,
            bb: None
        }
    }
}

#[derive(Clone)]
pub struct Mob {
    pub airSupply: Option<i32>,
    pub allowAlpha: Option<bool>,
    pub animStep: Option<f32>,
    pub animStepO: Option<f32>,
    pub attackTime: Option<i32>,
    pub bobStrength: Option<f32>,
    pub dead: Option<bool>,
    pub deathScore: Option<i32>,
    pub deathTime: Option<i32>,
    pub hasHair: Option<bool>,
    pub health: Option<i32>,
    pub hurtDir: Option<f32>,
    pub hurtDuration: Option<i32>,
    pub hurtTime: Option<i32>,
    pub invulnerableDuration: Option<i32>,
    pub invulnerableTime: Option<i32>,
    pub lastHealth: Option<i32>,
    pub oRun: Option<f32>,
    pub oTilt: Option<f32>,
    pub renderOffset: Option<f32>,
    pub rot: Option<f32>,
    pub rotA: Option<f32>,
    pub rotOffs: Option<f32>,
    pub run: Option<f32>,
    pub speed: Option<f32>,
    pub tickCount: Option<i32>,
    pub tilt: Option<f32>,
    pub timeOffs: Option<f32>,
    pub yBodyRot: Option<f32>,
    pub yBodyRotO: Option<f32>,
    pub ai: Option<AI>,
    pub modelName: Option<String>,
    pub textureName: Option<String>,
    pub entity: Option<EntityFields>
}

impl Mob {
    pub fn new() -> Self {
        Mob { 
            airSupply: None, 
            allowAlpha: None, 
            animStep: None, 
            animStepO: None, 
            attackTime: None, 
            bobStrength: None, 
            dead: None, 
            deathScore: None, 
            deathTime: None, 
            hasHair: None, 
            health: None, 
            hurtDir: None, 
            hurtDuration: None, 
            hurtTime: None, 
            invulnerableDuration: None, 
            invulnerableTime: None, 
            lastHealth: None, 
            oRun: None, 
            oTilt: None, 
            renderOffset: None, 
            rot: None, 
            rotA: None, 
            rotOffs: None, 
            run: None, 
            speed: None, 
            tickCount: None, 
            tilt: None, 
            timeOffs: None, 
            yBodyRot: None, 
            yBodyRotO: None, 
            ai: None, 
            modelName: None, 
            textureName: None, 
            entity: None 
        }
    }
}

#[derive(Clone)]
pub struct QuadrapedMob {
    pub mob: Mob
}

#[derive(Clone)]
pub struct HumanoidMob {
    pub armor: bool,
    pub helmet: bool,
    pub mob: Mob
}

#[derive(Clone)]
pub struct Sheep {
    pub graze: f32,
    pub grazeO: f32,
    pub grazing: bool,
    pub grazingTime: i32,
    pub hasFur: bool,
    pub quadrapedMob: QuadrapedMob
}

#[derive(Clone)]
pub struct Pig {
    pub quadrapedMob: QuadrapedMob
}

#[derive(Clone)]
pub struct Creeper {
    pub mob: Mob
}

#[derive(Clone)]
pub struct Zombie {
    pub humanoidMob: HumanoidMob
}

#[derive(Clone)]
pub struct Skeleton {
    pub zombie: Zombie
}

#[derive(Clone)]
pub struct Spider {
    pub quadrapedMob: QuadrapedMob
}

#[derive(Clone)]
pub struct Player {
    pub arrows: i32,
    pub bob: f32,
    pub oBob: f32,
    pub score: i32,
    pub userType: u8,
    pub inventory: Inventory,
    pub mob: Mob
}

#[derive(Clone)]
pub struct Item {
    pub age: i32,
    pub resource: i32,
    pub rot: f32,
    pub tickCount: i32,
    pub xd: f32,
    pub yd: f32,
    pub zd: f32,
    pub entity: EntityFields
}

#[derive(Clone)]
pub struct Arrow {
    pub damage: i32,
    pub gravity: f32,
    pub hasHilt: bool,
    pub stickTime: i32,
    pub time: i32,
    pub atype: i32,
    pub xRot: f32,
    pub xRotO: f32,
    pub xd: f32,
    pub yRot: f32,
    pub yRotO: f32,
    pub yd: f32,
    pub zd: f32,
    pub owner: Box<Option<Entity>>,
    pub entity: EntityFields
}

#[derive(Clone)]
pub struct PrimedTnt {
    pub life: i32,
    pub xd: f32,
    pub yd: f32,
    pub zd: f32,
    pub entity: EntityFields
}

#[derive(Clone)]
pub struct Sign {
    pub rot: f32,
    pub xd: f32,
    pub yd: f32,
    pub zd: f32,
    pub messages: Vec<String>,
    pub entity: EntityFields
}

#[derive(Clone)]
pub struct Smolder {
    pub life: i32,
    pub lifeTime: i32,
    pub entity: EntityFields
}

#[derive(Clone)]
pub struct AABB {
    pub epsilon: f32,
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub z0: f32,
    pub z1: f32
}

#[derive(Clone)]
pub struct AI {
    pub damage: i32,
    pub attackDelay: i32,
    pub jumping: bool,
    pub noActionTime: i32,
    pub runSpeed: f32,
    pub xxa: f32,
    pub yRotA: f32,
    pub yya: f32,
    pub attackTarget: Box<Option<Entity>>,
    pub mob: String,
    pub random: Random1
}

#[derive(Clone)]
pub struct Random1 {
    pub haveNextNextGaussian: bool,
    pub nextNextGaussian: f64,
    pub seed: i64
}

#[derive(Clone)]
pub struct Inventory {
    pub selected: i32,
    pub count: Vec<i32>,
    pub popTime: Vec<i32>,
    pub slots: Vec<i32>
}

#[derive(Error, Debug)]

pub enum ClassicError {
    #[error("Error Deserializing")]
    DeserializeError(#[from] DeserializeError),

    #[error("Classic Level Not Recognized")]
    LevelNotRecognized(),

    #[error("Unrecognized main class, expected `com.mojang.minecraft.level.Level`, but found {0}")]
    InvalidClass(String),

    #[error("File format not supported, expected 0-2 but found {0}")]
    InvalidFileFormat(u8),

    #[error("Unexpected Entity: {0}")]
    UnexpectedEntity(String),

    #[error("Invalid Entity Request")]
    InvalidEntity(),

    #[error("Unsupported Feature (Coming soon!)")]
    UnsupportedFeature(),
}

/*
* The following function accepts a file path, then reads in the file
* and determines which version the classic file is from
*/
pub fn read_level (file: String) -> Result <Level, ClassicError> {
    //Reading in a classic level and converting it to a decompressed stream of bytes
    let stream: Vec<u8> = read(file).unwrap();
    let mut d_stream = GzDecoder::new(&stream[..]);
    let mut bytes: Vec<u8> = Vec::new();
    d_stream.read_to_end(&mut bytes).unwrap();

    //Checking for a magic number at the start of the file
    let magic_number: u32 = u32_fs(0, &bytes[..]);

    //Preclassic levels do not have a magic number
    if magic_number != 0x271BB788 {return Ok(pre_classic_to_level(bytes))}

    //All 13a levels use version 1
    if bytes[4] == 1 {return Ok(classic_13_to_level(bytes))}

    //All future levels use version 2
    if bytes[4] == 2 {return Ok(classic_to_level(bytes)?)}

    return Err(ClassicError::LevelNotRecognized())

}

/*
* This function accepts a level object and a path and writes it to a level.dat
* in a classic format
*/
pub fn write_level (level: Level, path: String, format: u8) -> Result <(), ClassicError> {
    match format {
        0 => level_to_pre_classic(level, path)?,
        1 => level_to_classic_13(level, path)?,
        2 => {
            //Serialization is not yet supported
            return Err(ClassicError::UnsupportedFeature())
        },
        _ => return Err(ClassicError::InvalidFileFormat(format))
    }

    return Ok(())
}

/*
* This function will be called to parse all Minecraft Levels from
* rd-132211 to Classic 12a_03 
* Pre-classic saves only store an array of blocks in the shape of 256 x 64 x 256
*/
pub fn pre_classic_to_level (bytes: Vec<u8>) -> Level {
    let mut level: Level = Level::new();
    level.blocks = Some(bytes);
    return level;
}

/*
* This function will write a minecraft level in pre-classic format
* Pre-classic saves only store an array of blocks in the shape of 256 x 64 x 256
*/
pub fn level_to_pre_classic (level: Level, path: String) -> Result<(), ClassicError> {
    let name: &str = if path == "" {"level.dat"} else {"/level.dat"};

    let mut tiles: Vec<u8> = level.blocks.unwrap();
    //Removing blocks if the array is greater than the size pre-classic can support
    while tiles.len() > 256 * 64 * 256 {tiles.pop();}

    let output= OpenOptions::new()
        .write(true)
        .create(true)
        .open(path + name)
        .unwrap();

    let mut encoder = GzEncoder::new(output, Compression::default());
    encoder.write_all(&tiles).unwrap();

    Ok(())
}

/*
* This function will be called to parse all 13a Classic Minecraft Levels
* These levels follow a specific format, where the bytes are not labeled
* or serialized, but rather just follow a standard structure
*/
pub fn classic_13_to_level (bytes: Vec<u8>) -> Level {
    let mut buf: usize = 4;
    let mut level: Level = Level::new();

    //Setting level version - Byte format
    level.version = Some(bytes[buf]);
    buf += 1;

    //Parsing and setting level name - String format
    let mut sh: u16 = u16_fs(buf, &bytes[..]);
    buf += 2;
    level.name = Some(str_fs(buf, &bytes[..], sh as i32));
    buf += sh as usize;

    //Parsing and setting author name - String format
    sh = u16_fs(buf, &bytes[..]);
    buf += 2;
    level.creator = Some(str_fs(buf, &bytes[..], sh as i32));
    buf += sh as usize;

    //Setting timestamp - Long format
    level.createTime = Some(i64_fs(buf, &bytes[..]));
    buf += 8;

    //Setting width, depth, and height - Short Format
    level.width = Some(i16_fs(buf, &bytes[..]) as i32);
    buf += 2;
    level.height = Some(i16_fs(buf, &bytes[..]) as i32);
    buf += 2;
    level.depth = Some(i16_fs(buf, &bytes[..]) as i32);
    buf += 2;

    //Setting tile map - Array in the format of x -> z -> y
    let mut tile_map: Vec<u8> = Vec::new();
    for i in buf..bytes.len() as usize {
        tile_map.push(bytes[i as usize]);
    }

    level.blocks = Some(tile_map);

    return level;

}

/**
 * Following function writes a level object to a classic file in the 13a file format
 */
pub fn level_to_classic_13(level: Level, path: String) -> Result<(),ClassicError> {
    let mut bytes: Vec<u8> = Vec::new();

    //Writing magic header and version number
    let magic_number: u32 = 0x271BB788;
    bytes.extend_from_slice(&magic_number.to_be_bytes());
    if level.version.is_some() {bytes.push(level.version.unwrap())} else {bytes.push(1)}

    //Setting Author
    let name = if level.name.is_some() {level.name.unwrap()} else {String::from("A Nice World")};
    let len = name.len() as u16;
    bytes.extend_from_slice(&len.to_be_bytes());
    bytes.extend_from_slice(name.as_bytes());
    //let chars: Vec<char> = name.chars().collect();
    //for ch in chars {bytes.push(ch as u8)};

    //Setting World Name
    let creator = if level.creator.is_some() {level.creator.unwrap()} else {String::from("noname")};
    let len = creator.len() as u16;
    bytes.extend_from_slice(&len.to_be_bytes());
    bytes.extend_from_slice(creator.as_bytes());
    //let chars: Vec<char> = creator.chars().collect();
    //for ch in chars {bytes.push(ch as u8)};

    //Setting create time
    if level.createTime.is_some() {
        bytes.extend_from_slice(&level.createTime.unwrap().to_be_bytes())
    } else {
        bytes.extend_from_slice(&(0 as i64).to_be_bytes())
    }

    //Setting width, height, and depth
    if level.width.is_some() {
        bytes.extend_from_slice(&(level.width.unwrap() as i16).to_be_bytes())
    } else {
        bytes.extend_from_slice(&(256 as i16).to_be_bytes())
    }
    if level.height.is_some() {
        bytes.extend_from_slice(&(level.height.unwrap() as i16).to_be_bytes())
    } else {
        bytes.extend_from_slice(&(256 as i16).to_be_bytes())
    }
    if level.depth.is_some() {
        bytes.extend_from_slice(&(level.depth.unwrap() as i16).to_be_bytes())
    } else {
        bytes.extend_from_slice(&(64 as i16).to_be_bytes())
    }

    //Adding all blocks
    if level.blocks.is_some() {
        for tile in level.blocks.unwrap() {bytes.push(tile)}
    }

    //Opening file
    let output= OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap();

    //Writing to file
    let mut encoder = GzEncoder::new(output, Compression::default());
    encoder.write_all(&bytes).unwrap();

    Ok(())
}

/*
* All classic Levels from 14a_08 onwards use Java 6's Object
* serialization to encode the levels. As such, the following function
* deserializes the level, and then parse it into a level object
*/
pub fn classic_to_level (bytes: Vec<u8>) -> Result<Level, ClassicError> {

    //Moving past the Minecraft specific bytes
    let mut buf: usize = 4;
    let mut level: Level = Level::new();

    level.version = Some(bytes[buf as usize]);
    buf += 1;

    //Deserializing the classic level
    let mut deserializer: Deserializer = Deserializer::new();
    let contents: Vec<serialize::Object> = deserializer.deserialize(&bytes[buf..])?;

    //Running checks to determine the deserializer recognized the format of a classic level
    if contents.len() != 1 { return Err(ClassicError::DeserializeError(DeserializeError::InvalidContentLength(1, contents.len()))) }

    let object: serialize::NewObject = contents[0].get_new_object()?;

    //Unwrapping class info and class data
    let class_info: serialize::NewClassDesc = object.class_desc.get_new_class_desc()?;
    let class_data: serialize::ClassData = object.class_data.unwrap();

    if class_info.class_name != "com.mojang.minecraft.level.Level" {
        return Err(ClassicError::InvalidClass(class_info.class_name.clone()))
    }

    let fields: serialize::Fields = class_info.class_desc_info.unwrap().fields;
    let values: Vec<serialize::Value> = class_data.values;

    //Parsing all fields into a Level object
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
                level.name = values[i].get_object()?.get_new_string()?.string; 
            },
            "entities" => (), //All instances of the entities list are empty in the classic file, and as such this does not need to be parsed 
            "blockMap" => {

                //Parsing the blockMap into a list of entities and the player
                let block_map: serialize::NewObject = values[i].get_object()?.get_new_object()?;
                let field_descs: Vec<serialize::FieldDesc> = block_map.class_desc.get_new_class_desc()?.class_desc_info.unwrap().fields.field_descs;
                let values1: Vec<serialize::Value> = block_map.class_data.unwrap().values;

                //Parsing out the "all" list, which is the entity list
                let mut entity_grid: Vec<serialize::Value> = Vec::new();
                for i in 0..field_descs.len() {
                    if field_descs[i].get_field_name()? == "all" {
                        entity_grid = values1[i].clone().get_object()?.get_new_object()?.class_data.unwrap().values[1].get_array()?;
                    }
                }

                //Iterating through and converting all deserialized entities into Entity objects
                let mut entities: Vec<Entity> = Vec::new();
                for entity in entity_grid {
                    match parse_entity(entity.get_object()?.get_new_object()?) {
                        Ok(val) => {
                            entities.push(val.clone());
                            if matches!(val.clone(), Entity::Player(_)) {
                                level.player = Some(val.get_player()?);
                            }
                        },
                        Err(e) => println!("Entity Parsing Failed: {e}")
                    }
                }
                level.entities = Some(entities); 
            }, 
            "player" => {
                //Checking first if the player has been previously set under entities
                //Realistically this match arm would only be entered if the entity list corrupted
                if level.player.is_none() {
                    match parse_entity(values[i].get_object()?.get_new_object()?) {
                        Ok (val) => level.player = Some(val.get_player()?),
                        Err(e) => println!("Player Parsing Failed: {e}")
                    }
                }
            }, 
            _ => println!("Unexpected Field: {}", fields.field_descs[i].get_field_name()?.as_str())

        }
    }

    Ok(level)
}

fn parse_entity (entity: serialize::NewObject) -> Result<Entity,ClassicError> {
    let name: String = entity.class_desc.get_new_class_desc()?.class_name;
    let classes: Vec<&str> = name.split(".").collect();
    let values: Vec<serialize::Value> = entity.class_data.unwrap().values;
    let mut field_descs: Vec<Vec<serialize::FieldDesc>> = Vec::new();
    field_descs.push(entity.class_desc.get_new_class_desc()?.class_desc_info.unwrap().fields.field_descs);
    let mut super_class: Box<serialize::ClassDesc> = entity.class_desc.get_new_class_desc()?.class_desc_info.unwrap().super_class_desc;
    while !matches!(*super_class, serialize::ClassDesc::Null) {
        field_descs.push(super_class.get_new_class_desc()?.class_desc_info.unwrap().fields.field_descs);
        super_class = super_class.get_new_class_desc()?.class_desc_info.unwrap().super_class_desc;
    }
    let mut entity_fields: EntityFields = EntityFields::new();
    let mut index: usize = 0;
    for field in field_descs[field_descs.len()-1].clone() {
        match field.get_field_name()?.as_str() {
            "bbHeight" => entity_fields.bbHeight = Some(values[index].get_float()?), 
            "bbWidth" => entity_fields.bbWidth = Some(values[index].get_float()?),
            "collision" => entity_fields.collision = Some(values[index].get_boolean()?),
            "fallDistance" => entity_fields.fallDistance = Some(values[index].get_float()?),
            "footSize" => entity_fields.footSize = Some(values[index].get_float()?),
            "heightOffset" => entity_fields.heightOffset = Some(values[index].get_float()?),
            "horizontalCollision" => entity_fields.horizontalCollision = Some(values[index].get_boolean()?),
            "hovered" => entity_fields.hovered = Some(values[index].get_boolean()?),
            "makeStepSound" => entity_fields.makeStepSound = Some(values[index].get_boolean()?),
            "nextStep" => entity_fields.nextStep = Some(values[index].get_integer()?),
            "noPhysics" => entity_fields.noPhysics = Some(values[index].get_boolean()?),
            "onGround" => entity_fields.onGround = Some(values[index].get_boolean()?),
            "pushthrough" => entity_fields.pushthrough = Some(values[index].get_float()?),
            "removed" => entity_fields.removed = Some(values[index].get_boolean()?),
            "slide" => entity_fields.slide = Some(values[index].get_boolean()?),
            "textureId" => entity_fields.textureId = Some(values[index].get_integer()?),
            "walkDist" => entity_fields.walkDist = Some(values[index].get_float()?),
            "walkDistO" => entity_fields.walkDistO = Some(values[index].get_float()?),
            "x" => entity_fields.x = Some(values[index].get_float()?),
            "xOld" => entity_fields.xOld = Some(values[index].get_float()?),
            "xRot" => entity_fields.xRot = Some(values[index].get_float()?),
            "xRotO" => entity_fields.xRotO = Some(values[index].get_float()?),
            "xd" => entity_fields.xd = Some(values[index].get_float()?),
            "xo" => entity_fields.xo = Some(values[index].get_float()?),
            "y" => entity_fields.y = Some(values[index].get_float()?),
            "yOld" => entity_fields.yOld = Some(values[index].get_float()?),
            "yRot" => entity_fields.yRot = Some(values[index].get_float()?),
            "yRotO" => entity_fields.yRotO = Some(values[index].get_float()?),
            "ySlideOffset" => entity_fields.ySlideOffset = Some(values[index].get_float()?),
            "yd" => entity_fields.yd = Some(values[index].get_float()?),
            "yo" => entity_fields.yo = Some(values[index].get_float()?),
            "z" => entity_fields.z = Some(values[index].get_float()?),
            "zOld" => entity_fields.zOld = Some(values[index].get_float()?),
            "zd" => entity_fields.zd = Some(values[index].get_float()?),
            "zo" => entity_fields.zo = Some(values[index].get_float()?),
            "bb" => {
                let aabb_vals: Vec<serialize::Value> = values[index].get_object()?.get_new_object()?.class_data.unwrap().values;
                let aabb: AABB = AABB {
                    epsilon: aabb_vals[0].get_float()?,
                    x0: aabb_vals[1].get_float()?,
                    x1: aabb_vals[2].get_float()?,
                    y0: aabb_vals[3].get_float()?,
                    y1: aabb_vals[4].get_float()?,
                    z0: aabb_vals[5].get_float()?,
                    z1: aabb_vals[6].get_float()?
                };
                entity_fields.bb = Some(aabb);
            },
            "blockMap" => (),
            "level" => (),
            _ => println!("Unexpected entity field found: {}", field.get_field_name()?.as_str())
        }
        index += 1;
    }
    if classes[3] == "mob" || classes[3] == "player" {
        let mut mob: Mob = Mob::new();
        mob.entity = Some(entity_fields);
        for field in field_descs[field_descs.len()-2].clone() {
            match field.get_field_name()?.as_str() {
                "airSupply" => mob.airSupply = Some(values[index].get_integer()?),
                "allowAlpha" => mob.allowAlpha = Some(values[index].get_boolean()?), 
                "animStep" => mob.animStep = Some(values[index].get_float()?), 
                "animStepO" => mob.animStepO = Some(values[index].get_float()?), 
                "attackTime" => mob.attackTime = Some(values[index].get_integer()?), 
                "bobStrength" => mob.bobStrength = Some(values[index].get_float()?), 
                "dead" => mob.dead = Some(values[index].get_boolean()?), 
                "deathScore" => mob.deathScore = Some(values[index].get_integer()?), 
                "deathTime" => mob.deathTime = Some(values[index].get_integer()?), 
                "hasHair" => mob.hasHair = Some(values[index].get_boolean()?), 
                "health" => mob.health = Some(values[index].get_integer()?), 
                "hurtDir" => mob.hurtDir = Some(values[index].get_float()?), 
                "hurtDuration" => mob.hurtDuration = Some(values[index].get_integer()?), 
                "hurtTime" => mob.hurtTime = Some(values[index].get_integer()?), 
                "invulnerableDuration" => mob.invulnerableDuration = Some(values[index].get_integer()?), 
                "invulnerableTime" => mob.invulnerableTime = Some(values[index].get_integer()?), 
                "lastHealth" => mob.lastHealth = Some(values[index].get_integer()?), 
                "oRun" => mob.oRun = Some(values[index].get_float()?), 
                "oTilt" => mob.oTilt = Some(values[index].get_float()?), 
                "renderOffset" => mob.renderOffset = Some(values[index].get_float()?), 
                "rot" => mob.rot = Some(values[index].get_float()?), 
                "rotA" => mob.rotA = Some(values[index].get_float()?), 
                "rotOffs" => mob.rotOffs = Some(values[index].get_float()?), 
                "run" => mob.run = Some(values[index].get_float()?), 
                "speed" => mob.speed = Some(values[index].get_float()?), 
                "tickCount" => mob.tickCount = Some(values[index].get_integer()?), 
                "tilt" => mob.tilt = Some(values[index].get_float()?), 
                "timeOffs" => mob.timeOffs = Some(values[index].get_float()?), 
                "yBodyRot" => mob.yBodyRot = Some(values[index].get_float()?), 
                "yBodyRotO" => mob.yBodyRotO = Some(values[index].get_float()?), 
                "ai" => {
                    let ai_vals: Vec<serialize::Value> = values[index].get_object()?.get_new_object()?.class_data.unwrap().values;
                    let random_vals: Vec<serialize::Value> = ai_vals[11].get_object()?.get_new_object()?.class_data.unwrap().values;
                    let random: Random1 = Random1 { 
                        haveNextNextGaussian: random_vals[0].get_boolean()?,
                        nextNextGaussian: random_vals[1].get_double()?,
                        seed: random_vals[2].get_long()? 
                    };
                    let att_tar: Option<Entity> = match ai_vals[8].get_object()? {
                        serialize::Object::NewObject(_) => match parse_entity(ai_vals[8].get_object()?.get_new_object()?) {
                            Ok (val) => Some(val),
                            Err(e) => {
                                println!("Error Handling Target of {name}: {e}");
                                None
                            }
                        },
                        _ => None
                    };
                    let ai: AI = AI { 
                        damage: ai_vals[0].get_integer()?,
                        attackDelay: ai_vals[1].get_integer()?,
                        jumping: ai_vals[2].get_boolean()?,
                        noActionTime: ai_vals[3].get_integer()?,
                        runSpeed: ai_vals[4].get_float()?,
                        xxa: ai_vals[5].get_float()?,
                        yRotA: ai_vals[6].get_float()?,
                        yya: ai_vals[7].get_float()?,
                        attackTarget: Box::new(att_tar),
                        mob: name.clone(),
                        random: random
                    };
                    mob.ai = Some(ai);
                },
                "modelName" => mob.modelName = Some(values[index].get_object()?.get_new_string()?.string.unwrap()), 
                "textureName" => mob.textureName = Some(values[index].get_object()?.get_new_string()?.string.unwrap()), 
                _ => println!("Unexpected mob field found: {}", field.get_field_name()?.as_str())
            }
            index += 1;
        }
        match classes[4] {
            "Sheep" => {
                let quadraped_mob: QuadrapedMob = QuadrapedMob { mob: mob };
                let sheep: Sheep = Sheep { 
                    graze: values[index].get_float()?, 
                    grazeO: values[index + 1].get_float()?, 
                    grazing: values[index + 2].get_boolean()?,
                    grazingTime: values[index + 3].get_integer()?,
                    hasFur: values[index + 4].get_boolean()?,
                    quadrapedMob: quadraped_mob 
                };
                return Ok(Entity::Sheep(sheep))
            },
            "Pig" => {
                let quadraped_mob: QuadrapedMob = QuadrapedMob { mob: mob };
                let pig: Pig = Pig { quadrapedMob: quadraped_mob };
                return Ok(Entity::Pig(pig))
            },
            "Creeper" => {
                let creeper: Creeper = Creeper { mob: mob };
                return Ok(Entity::Creeper(creeper))
            },
            "Zombie" => {
                let humanoid_mob: HumanoidMob = HumanoidMob { 
                    armor: values[index].get_boolean()?, 
                    helmet: values[index + 1].get_boolean()?,
                    mob: mob
                };
                let zombie: Zombie = Zombie { humanoidMob: humanoid_mob };
                return Ok(Entity::Zombie(zombie))
            },
            "Skeleton" => {
                let humanoid_mob: HumanoidMob = HumanoidMob { 
                    armor: values[index].get_boolean()?, 
                    helmet: values[index + 1].get_boolean()?,
                    mob: mob
                };
                let zombie: Zombie = Zombie { humanoidMob: humanoid_mob };
                let skeleton: Skeleton = Skeleton { zombie: zombie };
                return Ok(Entity::Skeleton(skeleton))
            },
            "Spider" => {
                let quadraped_mob: QuadrapedMob = QuadrapedMob { mob: mob };
                let spider: Spider = Spider { quadrapedMob: quadraped_mob };
                return Ok (Entity::Spider(spider))
            },
            "Player" => {
                let inv_vals: Vec<serialize::Value> = values[index + 5].get_object()?.get_new_object()?.class_data.unwrap().values;
                let mut arrs: Vec<Vec<i32>> = Vec::new();
                for i in 1..inv_vals.len() {
                    let mut arr: Vec<i32> = Vec::new();
                    for val in inv_vals[i].get_array()? {
                        arr.push(val.get_integer()?);
                    }
                    arrs.push(arr);
                }
                let inventory: Inventory = Inventory { 
                    selected: inv_vals[0].get_integer()?, 
                    count: arrs[0].clone(), 
                    popTime: arrs[1].clone(), 
                    slots: arrs[2].clone(), 
                };
                let player: Player = Player { 
                    arrows: values[index].get_integer()?,  
                    bob: values[index + 1].get_float()?,   
                    oBob: values[index + 2].get_float()?,   
                    score: values[index + 3].get_integer()?,   
                    userType: values[index + 4].get_byte()?,   
                    inventory: inventory,   
                    mob: mob
                };
                return Ok(Entity::Player(player.clone()))
            },
            _ => return Err(ClassicError::UnexpectedEntity(name))
        }
    } else {
        match classes[4] {
            "Arrow" => {
                let owner: Option<Entity> = match values[index + 13].get_object()? {
                    serialize::Object::NewObject(_) => match parse_entity(values[index + 13].get_object()?.get_new_object()?) {
                        Ok (val) => Some(val),
                        Err(e) => {
                            println!("Error Handling Target of {name}: {e}");
                            None
                        }
                    },
                    _ => None
                };
                let arrow: Arrow = Arrow { 
                    damage: values[index].get_integer()?,
                    gravity: values[index + 1].get_float()?,
                    hasHilt: values[index + 2].get_boolean()?,
                    stickTime: values[index + 3].get_integer()?,
                    time: values[index + 4].get_integer()?,
                    atype: values[index + 5].get_integer()?,
                    xRot: values[index + 6].get_float()?,
                    xRotO: values[index + 7].get_float()?,
                    xd: values[index + 8].get_float()?,
                    yRot: values[index + 9].get_float()?,
                    yRotO: values[index + 10].get_float()?,
                    yd: values[index + 11].get_float()?,
                    zd: values[index + 12].get_float()?,
                    owner: Box::new(owner),
                    entity: entity_fields
                };
                return Ok(Entity::Arrow(arrow))
            },
            "Item" => {
                let item: Item = Item { 
                    age: values[index].get_integer()?, 
                    resource: values[index + 1].get_integer()?, 
                    rot: values[index + 2].get_float()?, 
                    tickCount: values[index + 3].get_integer()?, 
                    xd: values[index + 4].get_float()?, 
                    yd: values[index + 5].get_float()?, 
                    zd: values[index + 6].get_float()?, 
                    entity: entity_fields 
                };
                return Ok(Entity::Item(item));
            },
            "PrimedTnt" => {
                let primed_tnt: PrimedTnt = PrimedTnt { 
                    life: values[index].get_integer()?,
                    xd: values[index + 1].get_float()?, 
                    yd: values[index + 2].get_float()?, 
                    zd: values[index + 3].get_float()?, 
                    entity: entity_fields 
                };
                return Ok(Entity::PrimedTnt(primed_tnt))
            },
            "Sign" => {
                let mut arr: Vec<String> = Vec::new();
                for str in values[index + 4].get_array()? {
                    arr.push(str.get_object()?.get_new_string()?.string.unwrap())
                }
                let sign: Sign = Sign { 
                    rot: values[index].get_float()?, 
                    xd: values[index + 1].get_float()?, 
                    yd: values[index + 2].get_float()?, 
                    zd: values[index + 3].get_float()?, 
                    messages: arr, 
                    entity: entity_fields
                };
                return Ok(Entity::Sign(sign))
            }
            "fx" => {
                let smolder: Smolder = Smolder {
                    life: values[index].get_integer()?,
                    lifeTime: values[index + 1].get_integer()?,
                    entity: entity_fields 
                };
                return Ok(Entity::Smolder(smolder))
            },
            _ => return Err(ClassicError::UnexpectedEntity(name))
        }
    }
}
