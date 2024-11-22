/**
 * To Do:
 * Make Handle Map (somehow...)
 * Understand Arrays
 * Handle Exceptions TC_EXCEPTION
 */

use crate::from_stream::{i16_fs,i32_fs};
use thiserror::Error;

pub const STREAM_MAGIC: i16 = 0xAC_ED;
pub const STREAM_VERSION: i16 = 0x00_05;
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

//pub buf: usize = 0;
//pub contents: Vec<Content> = Vec::new();

#[derive(Clone)]
pub enum Content {
    Object(Object),
    BlockData(BlockData)
}

#[derive(Clone)]
pub enum Object {
    NewObject(NewObject),
    NewClass(NewClass),
    NewArray(NewArray),
    NewString(NewString), 
    NewEnum(NewEnum),
    NewClassDesc(NewClassDesc),
    PrevObject(i32), //TC_REFERENCE && Handle
    Null, //TC_NULL
    Exception, //TC_EXCEPTION how does this work????
    Reset, //TC_RESET how does this work????
}

#[derive(Clone)]
pub enum BlockData {
    BlockDataShort(BlockDataShort), 
    BlockDataLong(BlockDataLong)
}

#[derive(Clone)]
pub struct NewObject {
    //TC_OBJECT
    class_desc: ClassDesc,
    //newHandle
    class_data: Option<ClassData>
}

#[derive(Clone)]
pub struct NewClass {
    //TC_CLASS
    class_desc: ClassDesc
    //newHandle
}

#[derive(Clone)]
pub struct NewArray {
    //TC_ARRAY
    class_desc: ClassDesc,
    //newHandle
    size: Option<i32>,
    values: Option<Vec<Value>>
}

#[derive(Clone)]
pub enum NewString {
    String(
       //TC_STRING
       //newHandle
       Option<String> //When decoding, the first 2 bytes are used to determine length of string
    ),
    LongString(
        //TC_LONGSTRING
        //newHandle
        Option<String> //When decoding, the first 8 bytes are used to determine length of string
     )
}

#[derive(Clone)]
pub struct NewEnum {
    //TC_ENUM
    class_desc: ClassDesc,
    //newHandle
    enum_constant_name: Option<String>
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct BlockDataShort {
    //TC_BLOCKDATA
    size: u8,
    block_data: Vec<u8>
}

#[derive(Clone)]
pub struct BlockDataLong {
    //TC_BLOCKDATALONG
    size: i32,
    block_data: Vec<u8> //Array length is size
}

#[derive(Clone)]
pub enum ClassDesc {
    NewClassDesc(NewClassDesc),
    Null,
    PrevObject(i32), //TC_REFERENCE && Handle
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct ClassDescInfo {
    class_desc_flags: u8,
    fields: Fields,
    class_annotation: ClassAnnotation,
    super_class_desc: Box<ClassDesc>
}

#[derive(Clone)]
pub struct ProxyClassDescInfo {
    count: i32,
    proxy_interface_names: Vec<String>, //Array length is count
    class_annotation: ClassAnnotation,
    super_class_desc: Box<ClassDesc>
}

#[derive(Clone)]
pub enum ObjectAnnotation {
    EndBlockData, //TC_ENDBLOCKDATA
    Contents(
        Vec<Content>
        //TC_ENDBLOCKDATA
    )
}

#[derive(Clone)]
pub enum ExternalContent {
    Bytes(Vec<u8>),
    Object(Object)
}

#[derive(Clone)]
pub struct Fields {
    count: i16,
    field_descs: Vec<FieldDesc> //Array length is count
}

#[derive(Clone)]
pub enum ClassAnnotation {
    EndBlockData, //TC_ENDBLOCKDATA
    Contents(
        Vec<Content>
        //TC_ENDBLOCKDATA
    )
}

#[derive(Clone)]
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

#[derive(Clone)]
pub enum Value {
    Byte(u8),
    Char(char),
    Double(u64),
    Float(u32),
    Integer(i32),
    Short(i16),
    Long(i64),
    Boolean(bool),
    Array, //How do arrays work???
    Object(Object)
}

#[derive(Clone)]
pub enum Handle {
    NewClass(NewClass),
    NewClassDesc(NewClassDesc),
    NewArray(NewArray),
    NewObject(NewObject),
    NewString(NewString),
    NewEnum(NewEnum)
}

#[derive(Error, Debug)]
pub enum DeserializeError {
    #[error("Invalid Magic Number: Expected {STREAM_MAGIC} & Found {0}")]
    InvalidMagic(i16),

    #[error("Invalid Version Number: Expected {STREAM_VERSION} & Found {0}")]
    InvalidVersion(i16),

    #[error("Invalid Object Typecode Found: {0}")]
    InvalidObjectTypecode(u8),

    #[error("Genuine Apologies, but {0} has not be implemented yet because I am confused...")]
    Unimplemented(String)
}

pub struct Deserializer {
    buf: usize,
    handles: Vec<Handle>,
    contents: Vec<Content>
}

impl Deserializer {

    pub fn new (&self) -> Self {
        let buf: usize = 0;
        let handles: Vec<Handle> = Vec::new();
        let contents: Vec<Content> = Vec::new();
        
        Deserializer {
            buf,
            handles,
            contents
        }
    }

    pub fn deserialize (&mut self, bytes: &[u8]) -> Result<Vec<Content>,DeserializeError> {
        //Checking for valid stream magic number
        let magic: i16 = i16_fs(self.buf, bytes);
        self.buf += 2;
        if magic != STREAM_MAGIC {return Err(DeserializeError::InvalidMagic(magic))}

        //Checking for valid stream version number
        let version: i16 = i16_fs(self.buf, bytes);
        self.buf += 2;
        if version != STREAM_VERSION {return Err(DeserializeError::InvalidVersion(version))}

        self.read_contents(bytes)?;

        Ok(self.contents.clone());
    }

    pub fn read_contents (&mut self, bytes: &[u8]) -> Result<(),DeserializeError> {
        while self.buf < bytes.len() {read_content(bytes)?}
        Ok(());
    }

    pub fn read_content (&mut self, bytes: &[u8]) -> Result<(),DeserializeError> {
        let content: Content = match bytes[self.buf] {
            TC_OBJECT => read_object(bytes)?,
            TC_CLASS => read_object(bytes)?,
            TC_ARRAY => read_object(bytes)?,
            TC_STRING => read_object(bytes)?,
            TC_LONGSTRING => read_object(bytes)?,
            TC_ENUM => read_object(bytes)?,
            TC_CLASSDESC => read_object(bytes)?,
            TC_PROXYCLASSDESC => read_object(bytes)?,
            TC_REFERENCE => read_object(bytes)?,
            TC_NULL => read_object(bytes)?,
            TC_EXCEPTION => read_object(bytes)?,
            TC_RESET => read_object(bytes)?,
            TC_BLOCKDATA => read_blockdata(bytes)?,
            TC_BLOCKDATALONG => read_blockdata(bytes)?,
            _ => return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf]))
        };
        self.contents.push(content);
        Ok(());
    }

    pub fn read_object (&mut self, bytes: &[u8]) -> Result<Object,DeserializeError> {
        let object: Object = match bytes[self.buf] {
            TC_OBJECT => read_new_object(bytes)?,
            TC_CLASS => read_new_class(bytes)?,
            TC_ARRAY => read_new_array(bytes)?,
            TC_STRING => read_new_string(bytes)?,
            TC_LONGSTRING => read_new_string(bytes)?,
            TC_ENUM => read_new_enum(bytes)?,
            TC_CLASSDESC => read_new_class_desc(bytes)?,
            TC_PROXYCLASSDESC => read_new_proxy_class_desc(bytes)?,
            TC_REFERENCE => return Err(DeserializeError::Unimplemented(String::from("References"))),
            TC_NULL => Object::Null,
            TC_EXCEPTION => return Err(DeserializeError::Unimplemented(String::from("Exceptions"))),
            TC_RESET => return Err(DeserializeError::Unimplemented(String::from("Resets"))),
            _ => return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf]))
            
        };

        return Ok(object);
    }

    pub fn read_new_object (&mut self, bytes: &[u8]) -> Result<NewObject,DeserializeError> {
        if bytes[self.buf] != TC_OBJECT { return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf])) }
        self.buf += 1;

        let class_desc: ClassDesc = read_class_desc(bytes)?;
        let mut new_object: NewObject = NewObject {class_desc: class_desc, class_data: None};
        self.handles.push(Handle::NewObject(new_object));
        new_object.class_data = Some(read_class_data(bytes)?);

        return Ok(new_object);
    }

    pub fn read_new_class (&mut self, bytes: &[u8]) -> Result<NewClass,DeserializeError> {

        if bytes[self.buf] != TC_CLASS { return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf])) }
        self.buf += 1;

        let class_desc: ClassDesc = read_class_desc(bytes)?;
        let new_class: NewClass = NewClass { class_desc: class_desc };
        self.handles.push(Handle::NewClass(new_class));

        return Ok(new_class)
    }

    pub fn read_new_array (&mut self, bytes: &[u8]) -> Result<NewArray,DeserializeError> {
        if bytes[self.buf] != TC_ARRAY { return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf])) }
        self.buf += 1;

        let class_desc: ClassDesc = read_class_desc(bytes)?;
        let mut new_array: NewArray = NewArray { class_desc: class_desc, size: None, values: None };
        self.handles.push(Handle::NewArray(new_array));

        new_array.size = Some(i32_fs(self.buf, bytes));
        self.buf += 4;

        let values: Vec<Value> = Vec::new();
        for i in 0..new_array.size.unwrap() {
            //Implement when read_class_desc is implemented
            //Check to make sure the fields aren't out of bounds with our index
            //Determine what type the Value should be from fields
            //Extract the value (This should also move the buffer up so we shouldn't need to change the buffer here)
            //Push the value into vec
        }
        new_array.values = Some(values);

        return Ok (new_array);
    }

    pub fn read_new_string (&mut self, bytes: &[u8]) -> Result<NewString,DeserializeError> {
        
    }
}