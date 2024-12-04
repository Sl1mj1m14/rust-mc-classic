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
pub const TC_LONGSTRING: u8 =  0x7C;
pub const BASE_WIRE_HANDLE: i32 = 0x7E0000;
pub const CC_ARRAYLIST: &str = "java.util.ArrayList";

#[derive(Clone,Debug)]
pub enum Object {
    NewObject(NewObject),
    NewClass(NewClass),
    NewArray(NewArray),
    NewString(NewString), 
    NewClassDesc(NewClassDesc),
    //TC_REFERENCE
    Null, //TC_NULL
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

    pub fn get_new_class_desc (&self) -> Result<NewClassDesc,DeserializeError> {
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
pub struct NewClassDesc {
        //TC_CLASSDESC 
        pub class_name: String,
        pub uuid: i64,
        //newHandle
        pub class_desc_info: Option<ClassDescInfo>
}

#[derive(Clone,Debug)]
pub enum ClassDesc {
    NewClassDesc(NewClassDesc),
    Null
}

impl ClassDesc {
    pub fn get_new_class_desc (&self) -> Result<NewClassDesc,DeserializeError> {
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
pub struct ClassData {
    pub values: Vec<Value>
}

#[derive(Clone,Debug)]
pub struct ClassDescInfo {
    pub class_desc_flags: u8,
    pub fields: Fields,
    pub class_annotation: ClassAnnotation,
    pub super_class_desc: Box<ClassDesc>
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
        Vec<Object>
        //TC_ENDBLOCKDATA
    )
}

impl ClassAnnotation {
    pub fn get_contents (&self) -> Result<Vec<Object>,DeserializeError> {
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
    Array(Vec<Value>),
    Object(Object)
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
    contents: Vec<Object>,
}

impl Deserializer {

    pub fn new () -> Self {
        let buf: usize = 0;
        let handles: Vec<Object> = Vec::new();
        let contents: Vec<Object> = Vec::new();
        
        Deserializer {
            buf,
            handles,
            contents,
        }
    }

    pub fn deserialize (&mut self, bytes: &[u8]) -> Result<Vec<Object>,DeserializeError> {
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
            let object: Object = self.read_object(bytes)?;
            self.contents.push(object);
        }
        Ok(())
    }

    pub fn read_object (&mut self, bytes: &[u8]) -> Result<Object,DeserializeError> {
        let object: Object = match bytes[self.buf] {
            TC_OBJECT => Object::NewObject(self.read_new_object(bytes)?),
            TC_CLASS => Object::NewClass(self.read_new_class(bytes)?),
            TC_ARRAY => Object::NewArray(self.read_new_array(bytes)?),
            TC_STRING => Object::NewString(self.read_new_string(bytes)?),
            TC_LONGSTRING => Object::NewString(self.read_new_string(bytes)?),
            TC_CLASSDESC => Object::NewClassDesc(self.read_new_class_desc(bytes)?),
            TC_REFERENCE => self.read_reference(bytes)?,
            TC_NULL => { self.buf += 1; Object::Null },
            _ => return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf)) 
        };
        Ok(object)
    }

    pub fn read_new_object (&mut self, bytes: &[u8]) -> Result<NewObject,DeserializeError> {
        if bytes[self.buf] != TC_OBJECT { return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf)) }
        self.buf += 1;
        
        let class_desc: ClassDesc = self.read_class_desc(bytes)?;
        let mut new_object: NewObject = NewObject {class_desc: class_desc.clone(), class_data: None};

        let index: usize = self.handles.len();
        self.handles.push(Object::NewObject(new_object.clone())); 

        new_object.class_data = Some(self.read_class_data(bytes,class_desc)?);

        self.handles[index] = Object::NewObject(new_object.clone()); 

        Ok(new_object)
    }

    pub fn read_new_class (&mut self, bytes: &[u8]) -> Result<NewClass,DeserializeError> {
        if bytes[self.buf] != TC_CLASS { return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf)) }
        self.buf += 1;

        let class_desc: ClassDesc = self.read_class_desc(bytes)?;
        let new_class: NewClass = NewClass { class_desc: class_desc };

        self.handles.push(Object::NewClass(new_class.clone()));

        Ok(new_class)
    }

    pub fn read_new_array (&mut self, bytes: &[u8]) -> Result<NewArray,DeserializeError> {
        if bytes[self.buf] != TC_ARRAY { return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf)) }
        self.buf += 1;

        let class_desc: ClassDesc = self.read_class_desc(bytes)?;
        let mut new_array: NewArray = NewArray { class_desc: class_desc.clone(), size: None, values: None };
        
        let index: usize = self.handles.len();
        self.handles.push(Object::NewArray(new_array.clone()));

        new_array.size = Some(i32_fs(self.buf, bytes));
        self.buf += 4;

        let mut values: Vec<Value> = Vec::new();
        let code: char = class_desc.get_new_class_desc()?.class_name.chars().nth(1).unwrap();

        for _ in 0..new_array.size.unwrap() { 
            let value: Value = self.read_value(bytes, code)?;
            values.push(value);
        }
        new_array.values = Some(values);

        self.handles[index] = Object::NewArray(new_array.clone());

        Ok(new_array)
    }

    //Clean up string
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

    pub fn read_new_class_desc (&mut self, bytes: &[u8]) -> Result<NewClassDesc,DeserializeError> {
        if bytes[self.buf] != TC_CLASSDESC { return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf], self.buf)) }

        self.buf += 1;
        let len: i16 = i16_fs(self.buf, bytes);
        self.buf += 2;
        let class_name: String = str_fs(self.buf, bytes, len as i32);
        self.buf += len as usize;
        let uuid: i64 = i64_fs(self.buf, bytes);
        self.buf += 8;

        let mut new_class_desc: NewClassDesc = NewClassDesc {class_name: class_name.clone(), uuid: uuid, class_desc_info: None};

        let index = self.handles.len();
        self.handles.push(Object::NewClassDesc(new_class_desc.clone()));

        let class_desc_info: ClassDescInfo = self.read_class_desc_info(bytes)?;
        new_class_desc = NewClassDesc {class_name: class_name.clone(), uuid: uuid, class_desc_info: Some(class_desc_info) };

        self.handles[index] = Object::NewClassDesc(new_class_desc.clone());

        Ok(new_class_desc)
    }

    pub fn read_reference (&mut self, bytes: &[u8]) -> Result<Object,DeserializeError> {
        if bytes[self.buf] != TC_REFERENCE { return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf)) }

        self.buf += 1;
        let handle: i32 = i32_fs(self.buf, bytes);
        self.buf += 4;

        let index: usize = (handle - BASE_WIRE_HANDLE) as usize;
        
        if index >= self.handles.len() { return Err(DeserializeError::IndexOutOfBounds(index, self.handles.len()))}
        Ok(self.handles[index].clone())
    }

    pub fn read_class_desc (&mut self, bytes: &[u8]) -> Result<ClassDesc,DeserializeError> {
        match bytes[self.buf] {
            TC_CLASSDESC => Ok(ClassDesc::NewClassDesc(self.read_new_class_desc(bytes)?)),
            TC_NULL => { 
                self.buf += 1; 
                Ok(ClassDesc::Null) 
            },
            TC_REFERENCE => Ok(ClassDesc::NewClassDesc(self.read_reference(bytes)?.get_new_class_desc()?)),
            _ => return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf], self.buf))
        }

    }

    pub fn read_class_data (&mut self, bytes: &[u8], class_desc: ClassDesc) -> Result<ClassData,DeserializeError> {
        let mut values: Vec<Value> = Vec::new();
        let class_name: String = class_desc.get_new_class_desc()?.class_name;

        match class_name.as_str() {
            CC_ARRAYLIST => {
                let size = i32_fs(self.buf, bytes);
                self.buf += 4;

                values.push(Value::Integer(size));

                if bytes[self.buf] != TC_BLOCKDATA {return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf], self.buf))}
                self.buf += 6; //It moves an extra 5 as there is an unused int value

                let mut arr: Vec<Value> = Vec::new();
                for _ in 0..size as usize { arr.push(Value::Object(self.read_object(bytes)?)); }

                if bytes[self.buf] != TC_ENDBLOCKDATA {return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf], self.buf))}
                self.buf += 1;

                values.push(Value::Array(arr));

            },
            _ => {
                let mut fields: Vec<Vec<FieldDesc>> = Vec::new();
                fields.push(class_desc.get_new_class_desc()?.class_desc_info.unwrap().fields.field_descs);

                let mut super_class: ClassDesc = *class_desc.get_new_class_desc()?.class_desc_info.unwrap().super_class_desc;

                while !matches!(super_class, ClassDesc::Null) {
                    fields.push(super_class.get_new_class_desc()?.class_desc_info.unwrap().fields.field_descs);
                    super_class = *super_class.get_new_class_desc()?.class_desc_info.unwrap().super_class_desc;
                }

                for i in 0..fields.len() {
                    let list: Vec<FieldDesc> = fields[fields.len()-i-1].clone();

                    for field in list {
                        if bytes[self.buf] == TC_ENDBLOCKDATA { self.buf += 1; }
                        let code: char = field.get_typecode()?;
                        values.push(self.read_value(bytes,code)?); 
                    }
                }
            }
        }
        Ok(ClassData {values: values})    
    }

    pub fn read_class_desc_info (&mut self, bytes: &[u8]) -> Result<ClassDescInfo,DeserializeError> {

        let flag: u8 = bytes[self.buf];
        self.buf += 1;
        let fields: Fields = self.read_fields(bytes)?;
        let class_annotation: ClassAnnotation = self.read_class_annotation(bytes)?;
        let super_class_description: ClassDesc = self.read_class_desc(bytes)?;

        Ok(ClassDescInfo { 
            class_desc_flags: flag,
            fields: fields,
            class_annotation: class_annotation,
            super_class_desc: Box::new(super_class_description) 
        })

    }

    pub fn read_value (&mut self, bytes: &[u8], typecode: char) -> Result<Value,DeserializeError> {
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
            '[' =>  Value::Array(self.read_new_array(bytes)?.values.unwrap()),          
            'L' =>  Value::Object(self.read_object(bytes)?),
            _ => return Err(DeserializeError::InvalidObjectTypecode(bytes[self.buf],self.buf))
        };
        Ok(value)
    }

    pub fn read_fields (&mut self, bytes: &[u8]) -> Result<Fields,DeserializeError> {
        let count: i16 = i16_fs(self.buf, bytes);
        self.buf += 2;
        
        let mut field_descs: Vec<FieldDesc> = Vec::new();
        for _ in 0..count {
            field_descs.push(self.read_field_desc(bytes)?);
        }

        Ok(Fields { count: count, field_descs: field_descs })
    }

    pub fn read_field_desc (&mut self, bytes: &[u8]) -> Result<FieldDesc,DeserializeError> {
        let typecode: char = bytes[self.buf] as char;
        self.buf += 1;

        let len: i16 = i16_fs(self.buf, bytes);
        self.buf += 2;
        let name: String = str_fs(self.buf, bytes, len as i32);
        self.buf += len as usize;

        if typecode != '[' && typecode != 'L' { return Ok(FieldDesc::PrimitiveDesc(typecode, name)) }

        let ftype: String = self.read_new_string(bytes)?.string.unwrap();

        Ok(FieldDesc::ObjectDesc(typecode, name, ftype))
    }

    pub fn read_class_annotation (&mut self, bytes: &[u8]) -> Result<ClassAnnotation,DeserializeError> {
        let mut contents: Vec<Object> = Vec::new();
        while bytes[self.buf] != TC_ENDBLOCKDATA {
            contents.push(self.read_object(bytes)?);
        }
        if contents.len() > 0 {return Ok(ClassAnnotation::Contents(contents))}
        self.buf += 1;
        Ok(ClassAnnotation::EndBlockData)
    }
}