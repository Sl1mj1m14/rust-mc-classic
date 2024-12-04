/**
 * To Do:
 * Fix all instance of getting strings
 * Make Handle Map (somehow...)
 * Understand Arrays
 * Handle Exceptions TC_EXCEPTION
 * Handle Resets
 */

use crate::from_stream::{u16_fs,i16_fs,i32_fs,i64_fs,f32_fs,f64_fs,str_fs};
use thiserror::Error;

pub const STREAM_MAGIC: u16 = 0xAC_ED;
pub const STREAM_VERSION: u16 = 0x00_05;
pub const TC_NULL: u8 = 0x70;
pub const TC_REFERENCE: u8 = 0x71;
pub const TC_CLASSDESC: u8 = 0x72;
pub const TC_OBJECT: u8 = 0x73;
pub const TC_STRING: u8 = 0x74;
pub const TC_ARRAY: u8 = 0x75;
pub const TC_CLASS: u8 = 0x76;
pub const TC_BLOCKDATA: u8 = 0x77;
pub const TC_ENDBLOCKDATA: u8 = 0x78;
pub const TC_RESET: u8 = 0x79;
pub const TC_BLOCKDATALONG: u8 = 0x7A;
pub const TC_EXCEPTION: u8 = 0x7B;
pub const TC_LONGSTRING: u8 =  0x7C;
pub const TC_PROXYCLASSDESC: u8 =  0x7D;
pub const TC_ENUM: u8 =  0x7E;
pub const BASE_WIRE_HANDLE: i32 = 0x7E0000;

pub const SC_WRITE_METHOD: u8 = 0x01; //if SC_SERIALIZABLE
pub const SC_BLOCK_DATA: u8 = 0x08;    //if SC_EXTERNALIZABLE
pub const SC_SERIALIZABLE: u8 = 0x02;
pub const SC_EXTERNALIZABLE: u8 = 0x04;
pub const SC_ENUM: u8 = 0x10;

pub const CC_ARRAYLIST: &str = "java.util.ArrayList";

#[derive(Clone,Debug)]
pub enum Content {
    Object(Object),
    BlockData(BlockData)
}

impl Content {
    pub fn get_object (&self) -> Result<Object,DeserializeError> {
        match self {
            Content::Object(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_block_data (&self) -> Result<BlockData,DeserializeError> {
        match self {
            Content::BlockData(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }
}

#[derive(Clone,Debug)]
pub enum Object {
    NewObject(NewObject),
    NewClass(NewClass),
    NewArray(NewArray),
    NewString(NewString), 
    NewEnum(NewEnum),
    NewClassDesc(NewClassDesc),
    //TC_REFERENCE
    Null, //TC_NULL
    Exception, //TC_EXCEPTION how does this work????
    Reset, //TC_RESET how does this work????
}

impl Object {
    pub fn get_new_object (&self) -> Result<NewObject,DeserializeError> {
        match self {
            Object::NewObject(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_new_class (&self) -> Result<NewClass,DeserializeError> {
        match self {
            Object::NewClass(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_new_array (&self) -> Result<NewArray,DeserializeError> {
        match self {
            Object::NewArray(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_new_string (&self) -> Result<NewString,DeserializeError> {
        match self {
            Object::NewString(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_new_enum (&self) -> Result<NewEnum,DeserializeError> {
        match self {
            Object::NewEnum(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_new_class_desc (&self) -> Result<NewClassDesc,DeserializeError> {
        println!("Okay, I'm getting a new class...");
        match self {
            Object::NewClassDesc(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_null (&self) -> Result<(),DeserializeError> {
        match self {
            Object::Null => Ok(()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_exception (&self) -> Result<(),DeserializeError> {
        match self {
            Object::Exception => return Err(DeserializeError::Unimplemented(String::from("Exceptions"))),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_reset (&self) -> Result<(),DeserializeError> {
        match self {
            Object::Reset => return Err(DeserializeError::Unimplemented(String::from("Resets"))),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }
}

#[derive(Clone,Debug)]
pub enum BlockData {
    BlockDataShort(BlockDataShort), 
    BlockDataLong(BlockDataLong)
}

impl BlockData {
    pub fn get_block_data_short (&self) -> Result<BlockDataShort,DeserializeError> {
        match self {
            BlockData::BlockDataShort(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_block_data_long (&self) -> Result<BlockDataLong,DeserializeError> {
        match self {
            BlockData::BlockDataLong(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }
}

#[derive(Clone,Debug)]
pub struct NewObject {
    //TC_OBJECT
    pub class_desc: ClassDesc,
    //newHandle
    pub class_data: Option<ClassData>
}

#[derive(Clone,Debug)]
pub struct NewClass {
    //TC_CLASS
    pub class_desc: ClassDesc
    //newHandle
}

#[derive(Clone,Debug)]
pub struct NewArray {
    //TC_ARRAY
    pub class_desc: ClassDesc,
    //newHandle
    pub size: Option<i32>,
    pub values: Option<Vec<Value>>
}

#[derive(Clone,Debug)]
pub struct NewString {
    //TC_LONGSTRING || TC_STRING
    //newHandle
    //When decoding, the first [8 bytes if Long | 2 bytes if short] are used to determine length of string
    pub string: Option<String> 
}

#[derive(Clone,Debug)]
pub struct NewEnum {
    //TC_ENUM
    pub class_desc: ClassDesc,
    //newHandle
    pub enum_constant_name: Option<String>
}

#[derive(Clone,Debug)]
pub enum NewClassDesc {
    ClassDesc(
        //TC_CLASSDESC 
        String, //className 
        i64, //serialVersionUID
        //newHandle
        Option<ClassDescInfo>
    ),
    ProxyClassDesc (
        //TC_PROXYCLASSDESC
        //newHandle
        Option<ProxyClassDescInfo>
    )
}

impl NewClassDesc {
    pub fn get_class_name (&self) -> Result<String,DeserializeError> {
        //println!("Okay this is so weird I don't understand what is wrong here...");
        match self {
            NewClassDesc::ClassDesc(
                string, _, _ 
            ) => Ok(string.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_serial_version_uuid (&self) -> Result<i64,DeserializeError> {
        match self {
            NewClassDesc::ClassDesc(
                _, uuid, _ 
            ) => Ok(uuid.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_class_desc_info (&self) -> Result<Option<ClassDescInfo>,DeserializeError> {
        match self {
            NewClassDesc::ClassDesc(
                _, _, class_desc_info 
            ) => Ok(class_desc_info.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_proxy_class_desc_info (&self) -> Result<Option<ProxyClassDescInfo>,DeserializeError> {
        match self {
            NewClassDesc::ProxyClassDesc(class_desc_info) => Ok(class_desc_info.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }
}

#[derive(Clone,Debug)]
pub struct BlockDataShort {
    //TC_BLOCKDATA
    pub size: u8,
    pub block_data: Vec<u8>
}

#[derive(Clone,Debug)]
pub struct BlockDataLong {
    //TC_BLOCKDATALONG
    pub size: i32,
    pub block_data: Vec<u8> //Array length is size
}

#[derive(Clone,Debug)]
pub enum ClassDesc {
    NewClassDesc(NewClassDesc),
    Null
}

impl ClassDesc {
    pub fn get_new_class_desc (&self) -> Result<NewClassDesc,DeserializeError> {
        //println!("Hi, I am a class description");
        match self {
            ClassDesc::NewClassDesc(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_null (&self) -> Result<(),DeserializeError> {
        match self {
            ClassDesc::Null => Ok(()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }
}

#[derive(Clone,Debug)]
pub enum ClassData {
    // SC_SERIALIZABLE & classDescFlag && !(SC_WRITE_METHOD & classDescFlags)
    NoWrClass(Vec<Value>), 
    // SC_SERIALIZABLE & classDescFlag && SC_WRITE_METHOD & classDescFlags
    WrClass(Vec<Value>, ObjectAnnotation),
    // SC_EXTERNALIZABLE & classDescFlag && !(SC_BLOCKDATA  & classDescFlags
    ExternalContents(Vec<ExternalContent>),
    // SC_EXTERNALIZABLE & classDescFlag && SC_BLOCKDATA & classDescFlags
    ObjectAnnotation(ObjectAnnotation)
}

impl ClassData {
    pub fn get_values (&self) -> Result<Vec<Value>,DeserializeError> {
        match self {
            ClassData::NoWrClass(value) => Ok(value.clone()),
            ClassData::WrClass(value,_) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_object_annotation (&self) -> Result<ObjectAnnotation,DeserializeError> {
        match self {
            ClassData::ObjectAnnotation(value) => Ok(value.clone()),
            ClassData::WrClass(_,value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_external_contents (&self) -> Result<Vec<ExternalContent>,DeserializeError> {
        match self {
            ClassData::ExternalContents(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }
}

#[derive(Clone,Debug)]
pub struct ClassDescInfo {
    pub class_desc_flags: u8,
    pub fields: Fields,
    pub class_annotation: ClassAnnotation,
    pub super_class_desc: Box<ClassDesc>
}

#[derive(Clone,Debug)]
pub struct ProxyClassDescInfo {
    pub count: i32,
    pub proxy_interface_names: Vec<String>, //Array length is count
    pub class_annotation: ClassAnnotation,
    pub super_class_desc: Box<ClassDesc>
}

#[derive(Clone,Debug)]
pub enum ObjectAnnotation {
    EndBlockData, //TC_ENDBLOCKDATA
    Contents(
        Vec<Content>
        //TC_ENDBLOCKDATA
    )
}

impl ObjectAnnotation {
    pub fn get_contents (&self) -> Result<Vec<Content>,DeserializeError> {
        match self {
            ObjectAnnotation::Contents(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_end_block_data (&self) -> Result<(),DeserializeError> {
        match self {
            ObjectAnnotation::EndBlockData => Ok(()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }
}

#[derive(Clone,Debug)]
pub enum ExternalContent {
    Bytes(Vec<u8>),
    Object(Object)
}

impl ExternalContent {
    pub fn get_bytes (&self) -> Result<Vec<u8>,DeserializeError> {
        match self {
            ExternalContent::Bytes(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_object (&self) -> Result<Object,DeserializeError> {
        match self {
            ExternalContent::Object(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }
}

#[derive(Clone,Debug)]
pub struct Fields {
    pub count: i16,
    pub field_descs: Vec<FieldDesc> //Array length is count
}

#[derive(Clone,Debug)]
pub enum ClassAnnotation {
    EndBlockData, //TC_ENDBLOCKDATA
    Contents(
        Vec<Content>
        //TC_ENDBLOCKDATA
    )
}

impl ClassAnnotation {
    pub fn get_contents (&self) -> Result<Vec<Content>,DeserializeError> {
        match self {
            ClassAnnotation::Contents(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_end_block_data (&self) -> Result<(),DeserializeError> {
        match self {
            ClassAnnotation::EndBlockData => Ok(()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }
}

#[derive(Clone,Debug)]
pub enum FieldDesc {
    PrimitiveDesc(
        char,   //prim_typecode
        String, //fieldName
    ),
    ObjectDesc(
        char,   //obj_typecode
        String, //fieldName
        String, // String containing the field's type in field descriptor format
    )
}

impl FieldDesc {
    pub fn get_typecode (&self) -> Result<char,DeserializeError> {
        match self {
            FieldDesc::PrimitiveDesc(char,_) => Ok(char.clone()),
            FieldDesc::ObjectDesc(char,_,_) => Ok(char.clone())
        }
    }

    pub fn get_field_name (&self) -> Result<String,DeserializeError> {
        match self {
            FieldDesc::PrimitiveDesc(_,name) => Ok(name.clone()),
            FieldDesc::ObjectDesc(_,name,_) => Ok(name.clone())
        }
    }

    pub fn get_field_type (&self) -> Result<String,DeserializeError> {
        match self {
            FieldDesc::ObjectDesc(_,_,ftype) => Ok(ftype.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }
}

#[derive(Clone,Debug)]
pub enum Value {
    Byte(u8),
    Char(char),
    Double(f64),
    Float(f32),
    Integer(i32),
    Short(i16),
    Long(i64),
    Boolean(bool),
    Array(Vec<Value>), //How do arrays work???
    Object(Object),
    BlockData(BlockData)
}

impl Value {
    pub fn get_byte (&self) -> Result<u8,DeserializeError> {
        match self {
            Value::Byte(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_char (&self) -> Result<char,DeserializeError> {
        match self {
            Value::Char(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_double (&self) -> Result<f64,DeserializeError> {
        match self {
            Value::Double(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_float (&self) -> Result<f32,DeserializeError> {
        match self {
            Value::Float(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_integer (&self) -> Result<i32,DeserializeError> {
        match self {
            Value::Integer(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_short (&self) -> Result<i16,DeserializeError> {
        match self {
            Value::Short(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_long (&self) -> Result<i64,DeserializeError> {
        match self {
            Value::Long(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_boolean (&self) -> Result<bool,DeserializeError> {
        match self {
            Value::Boolean(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_array (&self) -> Result<Vec<Value>,DeserializeError> {
        match self {
            Value::Array(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_object (&self) -> Result<Object,DeserializeError> {
        match self {
            Value::Object(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }

    pub fn get_block_data (&self) -> Result<BlockData,DeserializeError> {
        match self {
            Value::BlockData(value) => Ok(value.clone()),
            _ => Err(DeserializeError::InvalidEnumValue())
        }
    }
}

#[derive(Error, Debug)]
pub enum DeserializeError {
    #[error("Invalid Enum Value Returned")]
    InvalidEnumValue(),

    #[error("Invalid Magic Number: Expected {STREAM_MAGIC} & Found {0}")]
    InvalidMagic(u16),

    #[error("Invalid Version Number: Expected {STREAM_VERSION} & Found {0}")]
    InvalidVersion(u16),

    #[error("Invalid Content Length: Expected {0} & Found {1}")]
    InvalidContentLength(usize, usize),

    #[error("Invalid Object Typecode Found: {0} at buffer {1}")]
    InvalidObjectTypecode(u8,usize),

    #[error("Index Out of Bounds: Index: {0}; Array Length: {1}")]
    IndexOutOfBounds(usize, usize),

    #[error("Um, buddy this is not minecraft, this is: {0}")]
    InvalidClass(String),

    #[error("Genuine Apologies, but {0} has not be implemented yet because I am confused...")]
    Unimplemented(String)
}

#[derive(Clone,Debug)]
pub struct Deserializer {
    buf: usize,
    handles: Vec<Object>,
    contents: Vec<Content>,
    super_flag: u8
}

impl Deserializer {

    pub fn new () -> Self {
        let buf: usize = 0;
        let super_flag: u8 = 0;
        let handles: Vec<Object> = Vec::new();
        let contents: Vec<Content> = Vec::new();
        
        Deserializer {
            buf,
            handles,
            contents,
            super_flag
        }
    }

    pub fn deserialize (&mut self, bytes: &[u8]) -> Result<Vec<Content>,DeserializeError> {
        //Checking for valid stream magic number
        let magic: u16 = u16_fs(self.buf, bytes);
        self.buf += 2;
        if magic != STREAM_MAGIC {return Err(DeserializeError::InvalidMagic(magic))}

        //Checking for valid stream version number
        let version: u16 = u16_fs(self.buf, bytes);
        self.buf += 2;
        if version != STREAM_VERSION {return Err(DeserializeError::InvalidVersion(version))}

        self.read_contents(bytes)?;

        Ok(self.contents.clone())
    }

    pub fn read_contents (&mut self, bytes: &[u8]) -> Result<(),DeserializeError> {
        while self.buf < bytes.len() {
            println!("Oh boy, here we go again");
            let content: Content = self.read_content(bytes)?;
            println!("I guess we are trying for more content");
            self.contents.push(content);
        }
        Ok(())
    }

    pub fn read_content (&mut self, bytes: &[u8]) -> Result<Content,DeserializeError> {
        println!("Uh oh - this is content, here is the byte we entered this bad boy: {} and it's {}",self.buf,bytes[self.buf]);
        let content: Content = match bytes[self.buf] {
            TC_OBJECT => Content::Object(self.read_object(bytes)?),
            TC_CLASS => Content::Object(self.read_object(bytes)?),
            TC_ARRAY => Content::Object(self.read_object(bytes)?),
            TC_STRING => Content::Object(self.read_object(bytes)?),
            TC_LONGSTRING => Content::Object(self.read_object(bytes)?),
            TC_ENUM => Content::Object(self.read_object(bytes)?),
            TC_CLASSDESC => Content::Object(self.read_object(bytes)?),
            TC_PROXYCLASSDESC => Content::Object(self.read_object(bytes)?),
            TC_REFERENCE => Content::Object(self.read_object(bytes)?),
            TC_NULL => Content::Object(self.read_object(bytes)?),
            TC_EXCEPTION => Content::Object(self.read_object(bytes)?),
            TC_RESET => Content::Object(self.read_object(bytes)?),
            TC_BLOCKDATA => Content::BlockData(self.read_blockdata(bytes)?),
            TC_BLOCKDATALONG => Content::BlockData(self.read_blockdata(bytes)?),
            _ => return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf))
        };
        
        println!("returning content");
        Ok(content)
    }

    pub fn read_object (&mut self, bytes: &[u8]) -> Result<Object,DeserializeError> {
        println!("Uh oh - this is an object, here is the byte we entered this bad boy: {} and it's {}",self.buf,bytes[self.buf]);
        let object: Object = match bytes[self.buf] {
            TC_OBJECT => Object::NewObject(self.read_new_object(bytes,false)?),
            TC_CLASS => Object::NewClass(self.read_new_class(bytes)?),
            TC_ARRAY => Object::NewArray(self.read_new_array(bytes)?),
            TC_STRING => Object::NewString(self.read_new_string(bytes)?),
            TC_LONGSTRING => Object::NewString(self.read_new_string(bytes)?),
            TC_ENUM => Object::NewEnum(self.read_new_enum(bytes)?),
            TC_CLASSDESC => Object::NewClassDesc(self.read_new_class_desc(bytes)?),
            TC_PROXYCLASSDESC => Object::NewClassDesc(self.read_new_class_desc(bytes)?),
            TC_REFERENCE => self.read_reference(bytes)?,
            TC_NULL => { self.buf += 1; Object::Null },
            TC_EXCEPTION => return Err(DeserializeError::Unimplemented(String::from("Exceptions"))),
            TC_RESET => return Err(DeserializeError::Unimplemented(String::from("Resets"))),
            _ => return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf))
            
        };
        println!("returning object");
        Ok(object)
    }

    pub fn read_blockdata (&mut self, bytes: &[u8]) -> Result<BlockData,DeserializeError> {
        println!("Uh oh - this is blockdata, here is the byte we entered this bad boy: {} and it's {}",self.buf,bytes[self.buf]);
        let block_data: BlockData = match bytes[self.buf] {
            TC_BLOCKDATA => {
                self.buf += 1;
                let byte: u8 = bytes[self.buf];
                self.buf += 1;
                let arr: Vec<u8> = self.get_arr(bytes, byte as usize)?;
                println!("Okay, array is back at {} and it's {}",self.buf,bytes[self.buf]);
                if bytes[self.buf] == TC_ENDBLOCKDATA {self.buf += 1;}
                BlockData::BlockDataShort(BlockDataShort { size: byte, block_data: arr })

            }
            TC_BLOCKDATALONG => {
                self.buf += 1;
                let int: i32 = i32_fs(self.buf, bytes);
                self.buf += 4;
                let arr: Vec<u8> = self.get_arr(bytes, int as usize)?;
                if bytes[self.buf] == TC_ENDBLOCKDATA {self.buf += 1;}
                BlockData::BlockDataLong(BlockDataLong { size: int, block_data: arr })
            }
            _ => return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf))
        };

        Ok(block_data)

    }

    pub fn read_new_object (&mut self, bytes: &[u8], flag: bool) -> Result<NewObject,DeserializeError> {
        println!("Uh oh - this is an object, here is the byte we entered this bad boy: {} and it's {}",self.buf,bytes[self.buf]);
        println!("I'm looking for the length of the index baby... {}", self.handles.len());

        if bytes[self.buf] == TC_OBJECT {
            self.buf += 1; 
        } else if !flag {
            return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf))
        }

        let class_desc: ClassDesc = self.read_class_desc(bytes)?;
        let mut new_object: NewObject = NewObject {class_desc: class_desc.clone(), class_data: None};

        println!("Hello!");
        //println!("Here is the class_desc: {:?}", class_desc.clone());

        let index: usize = self.handles.len();
        if !flag { println!("AHHHHHHHHHH - is this sheep? You better hope so...: {} at {index}", class_desc.get_new_class_desc()?.get_class_name()? ); self.handles.push(Object::NewObject(new_object.clone())); }


        //let class_desc_info: ClassDescInfo = class_desc.get_new_class_desc()?.get_class_desc_info()?.unwrap();

        //Change this so the whole class_desc is being passed to class_data - this will allow for arrayList handling
        new_object.class_data = Some(self.read_class_data(bytes,class_desc)?);
        println!("class data is done");

        if !flag { self.handles[index] = Object::NewObject(new_object.clone()); }

        Ok(new_object)
    }

    pub fn read_new_class (&mut self, bytes: &[u8]) -> Result<NewClass,DeserializeError> {
        println!("Uh oh - this is a class, here is the byte we entered this bad boy: {} and it's {}",self.buf,bytes[self.buf]);

        if bytes[self.buf] != TC_CLASS { return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf)) }
        self.buf += 1;

        let class_desc: ClassDesc = self.read_class_desc(bytes)?;
        let new_class: NewClass = NewClass { class_desc: class_desc };

        self.handles.push(Object::NewClass(new_class.clone()));

        Ok(new_class)
    }

    pub fn read_new_array (&mut self, bytes: &[u8]) -> Result<NewArray,DeserializeError> {
        println!("Uh oh - this is an array, here is the byte we entered this bad boy: {} and it's {}",self.buf,bytes[self.buf]);
        if bytes[self.buf] != TC_ARRAY { return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf)) }
        self.buf += 1;

        let class_desc: ClassDesc = self.read_class_desc(bytes)?;
        println!("{:?}", class_desc.clone());
        let mut new_array: NewArray = NewArray { class_desc: class_desc.clone(), size: None, values: None };
        
        let index: usize = self.handles.len();
        self.handles.push(Object::NewArray(new_array.clone()));

        new_array.size = Some(i32_fs(self.buf, bytes));
        self.buf += 4;
        println!("The size for this array is: {}", new_array.size.clone().unwrap());

        //println!("{:?}",class_desc.get_new_class_desc()?);

        //FIX TYPECODE FOR ARRAYS

        let mut values: Vec<Value> = Vec::new();
        let code: char = class_desc.get_new_class_desc()?.get_class_name()?.chars().nth(1).unwrap();
        let mut ftype: String = String::from("");

        if code == '[' || code == 'L' {
            let str: String = class_desc.get_new_class_desc()?.get_class_name()?;
            let mut chars = str.chars();
            chars.next();
            ftype = String::from(chars.as_str());
            println!("ftype");
        }

        println!("The type for this array is: {}", code);
        for _ in 0..new_array.size.unwrap() { 
            let value: Value = self.read_value(bytes, code)?;
            values.push(value);
        }
        new_array.values = Some(values);

        self.handles[index] = Object::NewArray(new_array.clone());

        Ok(new_array)
    }

    pub fn read_new_string (&mut self, bytes: &[u8]) -> Result<NewString,DeserializeError> {
        println!("Uh oh - this is a string, here is the byte we entered this bad boy: {} and it's {}",self.buf,bytes[self.buf]);
        match bytes[self.buf] {
            TC_STRING => {
                self.buf += 1;
                let mut new_string: NewString = NewString {string: None};

                let index: usize = self.handles.len();
                self.handles.push(Object::NewString(new_string.clone()));

                let sh: i16 = i16_fs(self.buf, bytes);
                self.buf += 2;
                new_string = NewString{string: Some(str_fs(self.buf, bytes, sh as i32))};
                self.buf += sh as usize;

                self.handles[index] = Object::NewString(new_string.clone());

                Ok(new_string)
            },
            TC_LONGSTRING => {
                self.buf += 1;
                let mut new_string: NewString = NewString {string: None};

                let index: usize = self.handles.len();
                self.handles.push(Object::NewString(new_string.clone()));

                let int: i32 = i32_fs(self.buf, bytes);
                self.buf += 4;
                new_string = NewString {string: Some(str_fs(self.buf, bytes, int))};
                self.buf += int as usize;

                self.handles[index] = Object::NewString(new_string.clone());

                Ok(new_string)
            },
            TC_REFERENCE => {
                Ok(self.read_reference(bytes)?.get_new_string()?)
            }
            _ => return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf))
        }

    }

    pub fn read_new_enum (&mut self, bytes: &[u8]) -> Result<NewEnum,DeserializeError> {
        println!("Uh oh - this is an enum, here is the byte we entered this bad boy: {} and it's {}",self.buf,bytes[self.buf]);
        if bytes[self.buf] != TC_ENUM { return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf)) }
        self.buf += 1;

        let class_desc: ClassDesc = self.read_class_desc(bytes)?;
        let mut new_enum: NewEnum = NewEnum { class_desc: class_desc, enum_constant_name: None };

        let index = self.handles.len();
        self.handles.push(Object::NewEnum(new_enum.clone()));

        new_enum.enum_constant_name = Some(self.read_new_string(bytes)?.string.unwrap());

        self.handles[index] = Object::NewEnum(new_enum.clone());

        Ok(new_enum)
    }

    pub fn read_new_class_desc (&mut self, bytes: &[u8]) -> Result<NewClassDesc,DeserializeError> {
        println!("Uh oh - this is a class desc, here is the byte we entered this bad boy: {} and it's {}",self.buf,bytes[self.buf]);
        match bytes[self.buf] {
            TC_CLASSDESC => {
                self.buf += 1;
                let len: i16 = i16_fs(self.buf, bytes);
                self.buf += 2;
                let class_name: String = str_fs(self.buf, bytes, len as i32);
                println!("Hey, so um, if you don't mind pre fields my class name is: {}", class_name);
                self.buf += len as usize;

                println!("{:?}",&bytes[self.buf..self.buf+25]);

                let serial_version_uuid: i64 = i64_fs(self.buf, bytes);
                self.buf += 8;
                println!("Hey - here is the uuid: {}", serial_version_uuid);

                let mut new_class_desc: NewClassDesc = NewClassDesc::ClassDesc(class_name.clone(), serial_version_uuid, None);

                let index = self.handles.len();
                self.handles.push(Object::NewClassDesc(new_class_desc.clone()));

                let class_desc_info: ClassDescInfo = self.read_class_desc_info(bytes,class_name.clone().as_str())?;
                new_class_desc = NewClassDesc::ClassDesc(class_name.clone(), serial_version_uuid, Some(class_desc_info));

                self.handles[index] = Object::NewClassDesc(new_class_desc.clone());

                Ok(new_class_desc)
            },
            TC_PROXYCLASSDESC => {
                self.buf += 1;
                let mut new_proxy_class_desc: NewClassDesc = NewClassDesc::ProxyClassDesc(None);

                let index = self.handles.len();
                self.handles.push(Object::NewClassDesc(new_proxy_class_desc.clone()));

                let proxy_class_desc_info: ProxyClassDescInfo = self.read_proxy_class_desc_info(bytes)?;
                new_proxy_class_desc = NewClassDesc::ProxyClassDesc(Some(proxy_class_desc_info));

                self.handles[index] = Object::NewClassDesc(new_proxy_class_desc.clone());

                Ok(new_proxy_class_desc)
            },
            _ => return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf], self.buf))

        }

    }

    pub fn read_reference (&mut self, bytes: &[u8]) -> Result<Object,DeserializeError> {
        println!("Uh oh - this is a reference, here is the byte we entered this bad boy: {} and it's {}",self.buf,bytes[self.buf]);
        if bytes[self.buf] != TC_REFERENCE { return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf)) }

        self.buf += 1;
        let handle: i32 = i32_fs(self.buf, bytes);
        self.buf += 4;
        println!("HANDLE NUMBER IS THIS - WHYYYYY: {}",handle - BASE_WIRE_HANDLE);

        let mut index: usize = (handle - BASE_WIRE_HANDLE) as usize;
        //--DEBUG--
        //if index == 20 { index = 25 } //DEBUG
        //if index == 21 { index = 20 } //DEBUG
        //if index == 23 { index = 24; } //DEBUG
        
        if index >= self.handles.len() { return Err(DeserializeError::IndexOutOfBounds(index, self.handles.len()))}
        //if self.buf == 3004 + 1 + 4 {
            //println!("Here is #27: {:?}",self.handles[27].clone());
            //println!("Here is REFERENCE: {:?}",self.handles[index].clone());
            //println!("{:?}",&bytes[self.buf..self.buf+100]);
        //}
        Ok(self.handles[index].clone())
    }

    pub fn read_class_desc (&mut self, bytes: &[u8]) -> Result<ClassDesc,DeserializeError> {
        println!("Uh oh - this is an inner class desc, here is the byte we entered this bad boy: {} and it's {}",self.buf,bytes[self.buf]);
        match bytes[self.buf] {
            TC_CLASSDESC => Ok(ClassDesc::NewClassDesc(self.read_new_class_desc(bytes)?)),
            TC_NULL => { 
                self.buf += 1; 
                Ok(ClassDesc::Null) 
            },
            TC_REFERENCE => {
                let ref0: Object = self.read_reference(bytes)?;
                if matches!(ref0.clone(), Object::NewClassDesc(NewClassDesc)) {
                    return Ok(ClassDesc::NewClassDesc(ref0.get_new_class_desc()?))
                } else {
                    return Ok(ClassDesc::NewClassDesc(ref0.get_new_object()?.class_desc.get_new_class_desc()?))
                }
                //Ok(ClassDesc::NewClassDesc(self.read_reference(bytes)?.get_new_class_desc()?))
            },
            _ => return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf], self.buf))
        }

    }

    pub fn read_class_data (&mut self, bytes: &[u8], class_desc: ClassDesc) -> Result<ClassData,DeserializeError> {
        println!("Uh oh - this is class data, here is the byte we entered this bad boy: {} and it's {}",self.buf,bytes[self.buf]);
        println!("{:?}",&bytes[self.buf..self.buf+100]);
        println!("Here are the fields we are getting: {:?}",class_desc.get_new_class_desc()?.get_class_desc_info()?.unwrap().fields.field_descs);
        //println!("{:?}",class_desc.get_new_class_desc()?.get_class_desc_info()?.unwrap().super_class_desc);
        let flag: u8 = self.super_flag;
        println!("{}", flag);
        match flag {
            SC_WRITE_METHOD =>  return Err(DeserializeError::Unimplemented(String::from("Externalized Data"))),
            SC_BLOCK_DATA => Ok(ClassData::ObjectAnnotation(self.read_object_annotation(bytes)?)),
            SC_SERIALIZABLE => {
                //println!("I'm serialized!");
                let mut values: Vec<Value> = Vec::new();
                let class_name: String = class_desc.get_new_class_desc()?.get_class_name()?;
                println! ("Class Name Is This:{}", class_name);
                match class_name.as_str() {
                    CC_ARRAYLIST => {
                        let size = i32_fs(self.buf, bytes);
                        println!("Size is this: {}", size);
                        self.buf += 4;
                        values.push(Value::Integer(size));
                        if bytes[self.buf] != TC_BLOCKDATA {return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf], self.buf))}
                        self.buf += 1;
                        self.buf += 5; //It moves 4 as there is an int value that is equal exactly to size
                        let mut arr: Vec<Value> = Vec::new();
                        //Check for objects being repeated inside an arrayList
                        //Objects set all fields - including for super classes when called from references
                        //Check if object has a reference as class
                        //If it does, create object here
                        //Get class description, and then remove all class data up through superclass
                        //Reset class data - starting with superclass
                        for i in 0..size as usize {
                            let temp_float = f32_fs(0, &[63, 0, 0, 0]);
                            println!("I really hope this is a value that makes sense: {}", temp_float);
                            println!("RETREIVING OBJECT [{i}] - RIGHT NOW - RED ALERT");
                            println!("{:?}",&bytes[self.buf..self.buf+100]);

                            /*if self.buf < bytes.len()-1 && bytes[self.buf] == TC_OBJECT && bytes[self.buf + 1] == TC_REFERENCE {
                                self.buf += 1;
                                //Read the reference in as a class description
                                let new_class_desc: NewClassDesc = self.read_reference(bytes)?.get_new_class_desc()?;

                                let tmp_obj = NewObject { class_desc: ClassDesc::NewClassDesc(new_class_desc.clone()), class_data: None };
                                let index = self.handles.len();
                                self.handles.push(Object::NewObject(tmp_obj.clone()));

                                let mut super_class: ClassDesc = new_class_desc.get_class_desc_info()?.unwrap().super_class_desc.class_desc;

                                let mut class_descs: Vec<ClassDesc> = Vec::new();
                                class_descs.push(ClassDesc::NewClassDesc(new_class_desc.clone()));
                                
                                while !matches!(super_class, ClassDesc::Null) {
                                    class_descs.push(super_class.clone());
                                    super_class = super_class.clone().get_new_class_desc()?.get_class_desc_info()?.unwrap().super_class_desc.class_desc;
                                }

                                let mut class_datas: Vec<ClassData> = Vec::new();

                                for i in 0..class_descs.len() {
                                    class_datas.push(self.read_class_data(bytes, class_descs[class_descs.len()-i-1].clone())?);
                                } 

                                //Rebuild new class
                                //Start with super class first
                                //Get last ClassDesc
                                //Get last ClassDescInfo
                                //Set SuperClass to Null/None  
                                //Get last Class
                                // 

                                //New Object

                                let mut object: NewObject = NewObject {class_desc: ClassDesc::Null, class_data: None};

                                for i in 0..class_datas.len() {
                                    let mut class_desc: NewClassDesc  = class_descs[class_descs.len()-i-1].clone().get_new_class_desc()?;
                                    let name: String = class_desc.get_class_name()?;
                                    let uuid: i64 = class_desc.get_serial_version_uuid()?;
                                    let mut info: ClassDescInfo = class_desc.get_class_desc_info()?.unwrap();
                                    info.super_class_desc = Box::new(object.clone());
                                    class_desc = NewClassDesc::ClassDesc(name, uuid, Some(info));
                                    object = NewObject {class_desc: ClassDesc::NewClassDesc(class_desc), class_data: Some(class_datas[i].clone())};
                                }

                                self.handles[index] = Object::NewObject(object.clone());

                                arr.push(Value::Object(Object::NewObject(object)));
                            } else {
                                arr.push(Value::Object(self.read_object(bytes)?));
                            }*/
                            arr.push(Value::Object(self.read_object(bytes)?));
                            
                        }
                        if bytes[self.buf] != TC_ENDBLOCKDATA {return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf], self.buf))}
                        self.buf += 1;
                        //The array of values is never returned...
                        //Maybe try fixing this you dummy
                        values.push(Value::Array(arr));

                    },
                    _ => {
                        println!("{:?}",&bytes[self.buf..self.buf+100]);
                        
                        //let len: i16 = class_desc.get_new_class_desc()?.get_class_desc_info()?.unwrap().fields.count;
                        let mut fields: Vec<Vec<FieldDesc>> = Vec::new();
                        fields.push(class_desc.get_new_class_desc()?.get_class_desc_info()?.unwrap().fields.field_descs);

                        let mut super_class: ClassDesc = *class_desc.get_new_class_desc()?.get_class_desc_info()?.unwrap().super_class_desc;

                        while !matches!(super_class, ClassDesc::Null) {
                            fields.push(super_class.get_new_class_desc()?.get_class_desc_info()?.unwrap().fields.field_descs);
                            super_class = *super_class.get_new_class_desc()?.get_class_desc_info()?.unwrap().super_class_desc;
                        }

                        for i in 0..fields.len() {
                            let list: Vec<FieldDesc> = fields[fields.len()-i-1].clone();
                            for field in list {
                                if bytes[self.buf] == TC_ENDBLOCKDATA { self.buf += 1; }
                                //if i >= fields.len() { return Err(DeserializeError::IndexOutOfBounds(i, fields.len()))}
                                let code: char = field.get_typecode()?;
                                println!("Here is the expected field: {}", field.get_field_name()?);
                                values.push(self.read_value(bytes,code)?); 
                            }
                        }
                    }
                }
                Ok(ClassData::NoWrClass(values))
            },
            SC_EXTERNALIZABLE => return Err(DeserializeError::Unimplemented(String::from("Externalized Data"))),
            SC_ENUM => return Err(DeserializeError::Unimplemented(String::from("Serialized Enums"))),
            _ => return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf], self.buf))

        }
    }

    pub fn read_class_desc_info (&mut self, bytes: &[u8], class_name: &str) -> Result<ClassDescInfo,DeserializeError> {
        println!("Uh oh - this is class info, here is the byte we entered this bad boy: {} and it's {}",self.buf,bytes[self.buf]);

        let flag: u8 = bytes[self.buf];
        if self.super_flag == 0 { self.super_flag = flag }
        println!("{} |{}| {}", bytes[self.buf-1], bytes[self.buf], bytes[self.buf+1]);
        self.buf += 1;
        let fields: Fields = self.read_fields(bytes, class_name)?;
        let class_annotation: ClassAnnotation = self.read_class_annotation(bytes)?;
        let super_class_description: ClassDesc = self.read_class_desc(bytes)?;

        Ok(ClassDescInfo { 
            class_desc_flags: flag,
            fields: fields,
            class_annotation: class_annotation,
            super_class_desc: Box::new(super_class_description) 
        })

    }

    pub fn read_proxy_class_desc_info (&mut self, bytes: &[u8]) -> Result<ProxyClassDescInfo,DeserializeError> {

        let count: i32 = i32_fs(self.buf, bytes);
        self.buf += 4;

        let mut proxy_interface_names: Vec<String> = Vec::new();
        for _ in 0..count {
            let len: i16 = i16_fs(self.buf, bytes);
            self.buf += 2;
            let string: String  = str_fs(self.buf, bytes, len as i32);
            self.buf += len as usize;
            proxy_interface_names.push(string);
        }

        let class_annotation: ClassAnnotation = self.read_class_annotation(bytes)?;
        let super_class_description: ClassDesc = self.read_class_desc(bytes)?;

        Ok(ProxyClassDescInfo { 
            count: count,
            proxy_interface_names: proxy_interface_names,
            class_annotation: class_annotation,
            super_class_desc: Box::new(super_class_description) 
        })
    }

    pub fn read_value (&mut self, bytes: &[u8], typecode: char, /*ftype: String*/) -> Result<Value,DeserializeError> {
        let value: Value = match typecode {
            'B' => {
                let b: u8 = bytes[self.buf];
                self.buf += 1;
                Value::Byte(b)
            },
            'C' => {
                let c: char  = bytes[self.buf] as char;
                self.buf += 1;
                Value::Char(c)
            },
            'D' => {
                let d: f64 = f64_fs(self.buf, bytes);
                self.buf += 8;
                Value::Double(d)
            },
            'F' => {
                let f: f32 = f32_fs(self.buf, bytes);
                self.buf += 4;
                Value::Float(f)
            },
            'I' => {
                let i: i32 = i32_fs(self.buf, bytes);
                self.buf += 4;
                Value::Integer(i)
            },
            'J' => {
                let j: i64 = i64_fs(self.buf, bytes);
                self.buf += 8;
                Value::Long(j)
            },
            'S' => {
                let s: i16 = i16_fs(self.buf, bytes);
                self.buf += 2;
                Value::Short(s)
            },
            'Z' => {
                let z: bool = bytes[self.buf] != 0;
                self.buf += 1;
                Value::Boolean(z)
            },
            'O' => {
                Value::BlockData(self.read_blockdata(bytes)?)
            },
            '[' =>  {
                match bytes[self.buf] {
                    TC_BLOCKDATA => Value::BlockData(self.read_blockdata(bytes)?),
                    TC_ARRAY => Value::Array(self.read_new_array(bytes)?.values.unwrap()),
                    TC_OBJECT => Value::Object(self.read_object(bytes)?),
                    _ => return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf))
                }
                
            },
            'L' =>  {
                println!("Oh hey, I'm looking for an object right now at {}", bytes[self.buf]);
                let content: Content = self.read_content(bytes)?;
                println!("object matched");
                match content {
                    Content::Object(_) => Value::Object(content.get_object()?),
                    Content::BlockData(_) => Value::BlockData(content.get_block_data()?)
                }
            },
            _ => return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf))
        };

        //println!("Value is: {:?}", value);
        Ok(value)
    }

    pub fn read_object_annotation (&mut self, bytes: &[u8]) -> Result<ObjectAnnotation,DeserializeError> {
        let mut contents: Vec<Content> = Vec::new();
        while bytes[self.buf] != TC_ENDBLOCKDATA {
            contents.push(self.read_content(bytes)?);
        }
        if contents.len() > 0 {return Ok(ObjectAnnotation::Contents(contents))}
        self.buf += 1;
        Ok(ObjectAnnotation::EndBlockData)
    }

    pub fn read_fields (&mut self, bytes: &[u8], class_name: &str) -> Result<Fields,DeserializeError> {
        println!("Uh oh - these are fields, here is the byte we entered this bad boy: {} and it's {}",self.buf,bytes[self.buf]);
        let mut count: i16 = i16_fs(self.buf, bytes);
        self.buf += 2;
        let mut field_descs: Vec<FieldDesc> = Vec::new();
        for _ in 0..count {
            field_descs.push(self.read_field_desc(bytes)?);
        }

        println!("HEY YOU GUYS!!! - HERE IS THE CLASS NAME: {}", class_name);
        println!("{:?}",&bytes[self.buf..self.buf+25]);

        Ok(Fields { count: count, field_descs: field_descs })
    }

    pub fn read_field_desc (&mut self, bytes: &[u8]) -> Result<FieldDesc,DeserializeError> {
        println!("Uh oh - this is a field desc, here is the byte we entered this bad boy: {} and it's {}",self.buf,bytes[self.buf]);
        let typecode: char = bytes[self.buf] as char;
        self.buf += 1;

        let mut len: i16 = i16_fs(self.buf, bytes);
        self.buf += 2;
        let name: String = str_fs(self.buf, bytes, len as i32);
        println!("{}:{}",typecode,name);
        self.buf += len as usize;

        if typecode != '[' && typecode != 'L' { return Ok(FieldDesc::PrimitiveDesc(typecode, name))}

        println!("Hey, this byte is like actually this: {} {} {} {} {} and NOT a length", 
        bytes[self.buf],
        bytes[self.buf + 1],
        bytes[self.buf + 2],
        bytes[self.buf + 3] as char,
        bytes[self.buf + 4] as char,
        );

        let ftype: String = self.read_new_string(bytes)?.string.unwrap();

        println!("{}",ftype);

        Ok(FieldDesc::ObjectDesc(typecode, name, ftype))
    }

    pub fn read_class_annotation (&mut self, bytes: &[u8]) -> Result<ClassAnnotation,DeserializeError> {
        let mut contents: Vec<Content> = Vec::new();
        while bytes[self.buf] != TC_ENDBLOCKDATA {
            contents.push(self.read_content(bytes)?);
        }
        if contents.len() > 0 {return Ok(ClassAnnotation::Contents(contents))}
        self.buf += 1;
        Ok(ClassAnnotation::EndBlockData)
    }

    pub fn get_arr (&mut self, bytes: &[u8], len: usize) -> Result<Vec<u8>,DeserializeError> {
        let mut ret: Vec<u8> = Vec::new();
        for _ in self.buf..(len + self.buf) {
            if self.buf >= bytes.len() { return Err(DeserializeError::IndexOutOfBounds(self.buf, bytes.len()))}
            ret.push(bytes[self.buf]);
            self.buf += 1;
        }

        Ok(ret)
    }
}