/**
 * To Do:
 * Make Handle Map (somehow...)
 * Understand Arrays
 * Handle Exceptions TC_EXCEPTION
 * Handle Resets
 */

use crate::from_stream::{i16_fs,i32_fs,i64_fs,str_fs};
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
pub enum Handle<'a> {
    NewClass(&'a NewClass),
    NewClassDesc(&'a NewClassDesc),
    NewArray(&'a NewArray),
    NewObject(&'a NewObject),
    NewString(&'a NewString),
    NewEnum(&'a NewEnum)
}

#[derive(Error, Debug)]
pub enum DeserializeError {
    #[error("Invalid Magic Number: Expected {STREAM_MAGIC} & Found {0}")]
    InvalidMagic(i16),

    #[error("Invalid Version Number: Expected {STREAM_VERSION} & Found {0}")]
    InvalidVersion(i16),

    #[error("Invalid Object Typecode Found: {0} at buffer {1}")]
    InvalidObjectTypecode(u8,usize),

    #[error("Index Out of Bounds: Index: {0}; Array Length: {1}")]
    IndexOutOfBounds(usize, usize),

    #[error("Genuine Apologies, but {0} has not be implemented yet because I am confused...")]
    Unimplemented(String)
}

pub struct Deserializer<'a> {
    buf: usize,
    handles: Vec<Handle<'a>>,
    contents: Vec<Content>
}

impl<'a> Deserializer<'a> {

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

        Ok(self.contents.clone())
    }

    pub fn read_contents (&mut self, bytes: &[u8]) -> Result<(),DeserializeError> {
        while self.buf < bytes.len() {self.read_content(bytes)?}
        Ok(())
    }

    pub fn read_content (&mut self, bytes: &[u8]) -> Result<(),DeserializeError> {
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
        self.contents.push(content);
        Ok(())
    }

    pub fn read_object (&mut self, bytes: &[u8]) -> Result<Object,DeserializeError> {
        let object: Object = match bytes[self.buf] {
            TC_OBJECT => Object::NewObject(self.read_new_object(bytes)?),
            TC_CLASS => Object::NewClass(self.read_new_class(bytes)?),
            TC_ARRAY => Object::NewArray(self.read_new_array(bytes)?),
            TC_STRING => Object::NewString(self.read_new_string(bytes)?),
            TC_LONGSTRING => Object::NewString(self.read_new_string(bytes)?),
            TC_ENUM => Object::NewEnum(self.read_new_enum(bytes)?),
            TC_CLASSDESC => Object::NewClassDesc(self.read_new_class_desc(bytes)?),
            TC_PROXYCLASSDESC => Object::NewClassDesc(self.read_new_class_desc(bytes)?),
            TC_REFERENCE => Object::PrevObject(self.read_reference(bytes)?),
            TC_NULL => { self.buf += 1; Object::Null },
            TC_EXCEPTION => return Err(DeserializeError::Unimplemented(String::from("Exceptions"))),
            TC_RESET => return Err(DeserializeError::Unimplemented(String::from("Resets"))),
            _ => return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf))
            
        };

        Ok(object)
    }

    pub fn read_blockdata (&mut self, bytes: &[u8]) -> Result<BlockData,DeserializeError> {
        let block_data: BlockData = match bytes[self.buf] {
            TC_BLOCKDATA => {
                self.buf += 1;
                let byte: u8 = bytes[self.buf];
                self.buf += 1;
                let arr: Vec<u8> = self.get_arr(bytes, byte as usize)?;
                BlockData::BlockDataShort(BlockDataShort { size: byte, block_data: arr })

            }
            TC_BLOCKDATALONG => {
                self.buf += 1;
                let int: i32 = i32_fs(self.buf, bytes);
                self.buf += 4;
                let arr: Vec<u8> = self.get_arr(bytes, int as usize)?;
                BlockData::BlockDataLong(BlockDataLong { size: int, block_data: arr })
            }
            _ => return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf))
        };

        Ok(block_data)

    }

    pub fn read_new_object (&mut self, bytes: &[u8]) -> Result<NewObject,DeserializeError> {
        if bytes[self.buf] != TC_OBJECT { return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf)) }
        self.buf += 1;

        let class_desc: ClassDesc = self.read_class_desc(bytes)?;
        let mut new_object: NewObject = NewObject {class_desc: class_desc, class_data: None};
        //self.handles.push(Handle::NewObject(&new_object));
        new_object.class_data = Some(self.read_class_data(bytes)?);

        return Ok(new_object);
    }

    pub fn read_new_class (&mut self, bytes: &[u8]) -> Result<NewClass,DeserializeError> {

        if bytes[self.buf] != TC_CLASS { return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf)) }
        self.buf += 1;

        let class_desc: ClassDesc = self.read_class_desc(bytes)?;
        let new_class: NewClass = NewClass { class_desc: class_desc };
        //self.handles.push(Handle::NewClass(&new_class));

        return Ok(new_class)
    }

    pub fn read_new_array (&mut self, bytes: &[u8]) -> Result<NewArray,DeserializeError> {
        if bytes[self.buf] != TC_ARRAY { return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf)) }
        self.buf += 1;

        let class_desc: ClassDesc = self.read_class_desc(bytes)?;
        let mut new_array: NewArray = NewArray { class_desc: class_desc, size: None, values: None };
        //self.handles.push(Handle::NewArray(&new_array));

        new_array.size = Some(i32_fs(self.buf, bytes));
        self.buf += 4;

        let values: Vec<Value> = Vec::new();
        for i in 0..new_array.size.unwrap() { //Check for correct indexing
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
        match bytes[self.buf] {
            TC_STRING => {
                let mut new_string: NewString = NewString::String(None);
                //self.handles.push(Handle::NewString(&new_string));
                self.buf += 1;
                let sh: i16 = i16_fs(self.buf, bytes);
                self.buf += 2;
                new_string = NewString::String(Some(str_fs(self.buf, bytes, sh as i32)));
                self.buf += sh as usize;
                Ok(new_string)
            },
            TC_LONGSTRING => {
                let mut new_string: NewString = NewString::LongString(None);
                //self.handles.push(Handle::NewString(&new_string));
                self.buf += 1;
                let int: i32 = i32_fs(self.buf, bytes);
                self.buf += 4;
                new_string = NewString::String(Some(str_fs(self.buf, bytes, int)));
                self.buf += int as usize;
                Ok(new_string)
            },
            _ => return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf))
        }

    }

    pub fn read_new_enum (&mut self, bytes: &[u8]) -> Result<NewEnum,DeserializeError> {
        if bytes[self.buf] != TC_ENUM { return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf)) }
        self.buf += 1;

        let class_desc: ClassDesc = self.read_class_desc(bytes)?;
        let mut new_enum: NewEnum = NewEnum { class_desc: class_desc, enum_constant_name: None };
        //self.handles.push(Handle::NewEnum(&new_enum));
        new_enum.enum_constant_name = Some(self.get_string(bytes)?); //For strings of this type, are they utf?
        Ok(new_enum)
    }

    pub fn read_new_class_desc (&mut self, bytes: &[u8]) -> Result<NewClassDesc,DeserializeError> {
        match bytes[self.buf] {
            TC_CLASSDESC => {
                self.buf += 1;
                let class_name: String = self.get_string(bytes)?;
                let serial_version_uuid: i64 = i64_fs(self.buf, bytes);
                self.buf += 8;
                let mut new_class_desc: NewClassDesc = NewClassDesc::ClassDesc(class_name, serial_version_uuid, None);
                //self.handles.push(Handle::NewClassDesc(&new_class_desc));
                let class_desc_info: ClassDescInfo = self.read_class_desc_info(bytes);
                new_class_desc = NewClassDesc::ClassDesc(class_name, serial_version_uuid, Some(class_desc_info));
                Ok(new_class_desc)
            },
            TC_PROXYCLASSDESC => {
                self.buf += 1;
                let mut new_proxy_class_desc: NewClassDesc = NewClassDesc::ProxyClassDesc(None);
                //self.handles.push(Handle::NewClassDesc(&new_proxy_class_desc));
                let proxy_class_desc_info: ProxyClassDescInfo = self.read_proxy_class_desc_info(bytes);
                new_proxy_class_desc = NewClassDesc::ProxyClassDesc(Some(proxy_class_desc_info));
                Ok(new_proxy_class_desc)
            },
            _ => return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf], self.buf))

        }

    }

    pub fn read_reference (&mut self, bytes: &[u8]) -> Result<i32,DeserializeError> {
        //This should eventually return a reference to an existing object, and I'm not entirely sure how to handle that quite yet
        return Err(DeserializeError::Unimplemented(String::from("Handles")))
    }

    pub fn read_class_desc (&mut self, bytes: &[u8]) -> Result<ClassDesc,DeserializeError> {
        match bytes[self.buf] {
            TC_CLASSDESC => Ok(ClassDesc::NewClassDesc(self.read_new_class_desc(bytes)?)),
            TC_NULL => { self.buf += 1; Ok(ClassDesc::Null) },
            TC_REFERENCE => Ok(ClassDesc::PrevObject(self.read_reference(bytes)?)),
            _ => return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf], self.buf))
        }

    }

    /*pub enum ClassData {
        // SC_SERIALIZABLE & classDescFlag && !(SC_WRITE_METHOD & classDescFlags)
        NoWrClass(Vec<Value>), 
        // SC_SERIALIZABLE & classDescFlag && SC_WRITE_METHOD & classDescFlags
        WrClass(Vec<Value>, ObjectAnnotation),
        // SC_EXTERNALIZABLE & classDescFlag && !(SC_BLOCKDATA  & classDescFlags
        ExternalContents(Vec<ExternalContent>),
        // SC_EXTERNALIZABLE & classDescFlag && SC_BLOCKDATA & classDescFlags
        ObjectAnnotation(ObjectAnnotation)
    }*/

    pub fn read_class_data (&mut self, bytes: &[u8]) -> Result<ClassData,DeserializeError> {

    }

    pub fn get_string (&mut self, bytes: &[u8]) -> Result<String,DeserializeError> {
        match bytes[self.buf] {
            TC_STRING => {
                self.buf += 1;
                let sh: i16 = i16_fs(self.buf, bytes);
                self.buf += 2;
                let string: String = str_fs(self.buf, bytes, sh as i32);
                self.buf += sh as usize;
                Ok(string)
            },
            TC_LONGSTRING => {
                self.buf += 1;
                let int: i32 = i32_fs(self.buf, bytes);
                self.buf += 4;
                let string: String = str_fs(self.buf, bytes, int);
                self.buf += int as usize;
                Ok(string)
            },
            _ => return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf))
        }
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