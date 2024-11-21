use crate::from_stream::u16_fs;



pub struct JavaClass {

}

impl JavaClass {
    pub fn new () -> Self {
        JavaClass {  }
    }
}

pub fn deserialize (bytes: &[u8]) -> JavaClass {
    let buf: usize = 0;
    let magic: u16 = u16_fs(buf, bytes);
    println!("{}",buf);
    if magic != 0xaced {return JavaClass::new()}

    let version: u16 = u16_fs(buf, bytes);
    if version != 5 {return JavaClass::new()}


    return JavaClass::new()
}