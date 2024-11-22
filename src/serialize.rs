/**
 * To Do:
 * Make Handle Map (somehow...)
 * Understand Arrays
 * Handle Exceptions TC_EXCEPTION
 */

use crate::from_stream::u16_fs;

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
pub const BASE_WIRE_HANDLE: u32 = 0x7E0000;

pub const SC_WRITE_METHOD: u8 = 0x01; //if SC_SERIALIZABLE
pub const SC_BLOCK_DATA: u8 = 0x08;    //if SC_EXTERNALIZABLE
pub const SC_SERIALIZABLE: u8 = 0x02;
pub const SC_EXTERNALIZABLE: u8 = 0x04;
pub const SC_ENUM: u8 = 0x10;

//pub buf: usize = 0;
//pub contents: Vec<Content> = Vec::new();

pub enum Content {
    Object(Object),
    BlockData(BlockData)
}

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
    Reset, //TC_RESET
}

pub enum BlockData {
    BlockDataShort(BlockDataShort), 
    BlockDataLong(BlockDataLong)
}

pub struct NewObject {
    //TC_OBJECT
    class_desc: ClassDesc,
    //newHandle
    classdata: ClassData
}

pub struct NewClass {
    //TC_CLASS
    class_desc: ClassDesc
    //newHandle
}

pub struct NewArray {
    //TC_ARRAY
    class_desc: ClassDesc,
    //newHandle
    size: i32,
    array: Vec<Value>
}

pub enum NewString {
    //TC_STRING || TC_LONGSTRING
    String(
       //TC_STRING
       //newHandle
       String //When decoding, the first 2 bytes are used to determine length of string
    ),
    LongString(
        //TC_LONGSTRING
        //newHandle
        String //When decoding, the first 8 bytes are used to determine length of string
     )
}

pub struct NewEnum {
    //TC_ENUM
    class_desc: ClassDesc,
    //newHandle
    enum_constant_name: String
}

pub enum NewClassDesc {
    ClassDesc(
        //TC_CLASSDESC 
        String, //className 
        i64, //serialVersionUID
        //newHandle
        ClassDescInfo
    ),
    ProxyClassDesc (
        //TC_PROXYCLASSDESC
        //newHandle
        ProxyClassDescInfo
    )
}

pub struct BlockDataShort {
    //TC_BLOCKDATA
    size: u8,
    block_data: Vec<u8>
}

pub struct BlockDataLong {
    //TC_BLOCKDATALONG
    size: i32,
    block_data: Vec<u8> //Array length is size
}

pub enum ClassDesc {
    NewClassDesc,
    Null,
    PrevObject
}

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

pub struct ClassDescInfo {
    class_desc_flags: u8,
    fields: Fields,
    class_annotation: ClassAnnotation,
    super_class_desc: ClassDesc
}

pub struct ProxyClassDescInfo {
    count: i32,
    proxy_interface_names: Vec<String>, //Array length is count
    class_annotation: ClassAnnotation,
    super_class_desc: ClassDesc
}

pub enum ObjectAnnotation {
    EndBlockData, //TC_ENDBLOCKDATA
    Contents(
        Vec<Content>
        //TC_ENDBLOCKDATA
    )
}

pub enum ExternalContent {
    Bytes(Vec<u8>),
    Object(Object)
}

pub struct Fields {
    count: u16,
    field_descs: Vec<FieldDesc> //Array length is count
}

pub enum ClassAnnotation {
    EndBlockData, //TC_ENDBLOCKDATA
    Contents(
        Vec<Content>
        //TC_ENDBLOCKDATA
    )
}

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

pub struct JavaClass {

}

impl JavaClass {
    pub fn new () -> Self {
        JavaClass {  }
    }
}

pub fn deserialize (bytes: &[u8]) -> JavaClass {
    let mut buf: usize = 0;
    let magic: u16 = u16_fs(buf, bytes);
    buf += 2;
    println!("{}",buf);
    println!("{}", magic);
    if magic != STREAM_MAGIC {return JavaClass::new()}

    let version: u16 = u16_fs(buf, bytes);
    buf += 2;
    if version != STREAM_VERSION {return JavaClass::new()}



    println!("{:?}", &bytes[buf..buf + 5]);


    return JavaClass::new()
}