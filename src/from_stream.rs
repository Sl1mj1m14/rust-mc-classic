pub fn u16_fs (mut buf: usize, bytes: &[u8]) -> u16 {
    const LEN: usize = 2;
    let mut bits: [u8; LEN] = [0; LEN];
    for i in 0..bits.len() {
        bits[i] = bytes[buf];
        buf += 1;
    }   
    return u16::from_be_bytes(bits);
}

pub fn i16_fs (mut buf: usize, bytes: &[u8]) -> i16 {
    const LEN: usize = 2;
    let mut bits: [u8; LEN] = [0; LEN];
    for i in 0..bits.len() {
        bits[i] = bytes[buf];
        buf += 1;
    }   
    return i16::from_be_bytes(bits);
}

pub fn u32_fs (mut buf: usize, bytes: &[u8]) -> u32 {
    const LEN: usize = 4;
    let mut bits: [u8; LEN] = [0; LEN];
    for i in 0..bits.len() {
        bits[i] = bytes[buf];
        buf += 1;
    }   
    return u32::from_be_bytes(bits);
}

pub fn i32_fs (mut buf: usize, bytes: &[u8]) -> i32 {
    const LEN: usize = 4;
    let mut bits: [u8; LEN] = [0; LEN];
    for i in 0..bits.len() {
        bits[i] = bytes[buf];
        buf += 1;
    }   
    return i32::from_be_bytes(bits);
}

pub fn f32_fs (mut buf: usize, bytes: &[u8]) -> f32 {
    const LEN: usize = 4;
    let mut bits: [u8; LEN] = [0; LEN];
    for i in 0..bits.len() {
        bits[i] = bytes[buf];
        buf += 1;
    }   
    return f32::from_be_bytes(bits);
}

pub fn u64_fs (mut buf: usize, bytes: &[u8]) -> u64 {
    const LEN: usize = 8;
    let mut bits: [u8; LEN] = [0; LEN];
    for i in 0..bits.len() {
        bits[i] = bytes[buf];
        buf += 1;
    }   
    return u64::from_be_bytes(bits);
}

pub fn i64_fs (mut buf: usize, bytes: &[u8]) -> i64 {
    const LEN: usize = 8;
    let mut bits: [u8; LEN] = [0; LEN];
    for i in 0..bits.len() {
        bits[i] = bytes[buf];
        buf += 1;
    }   
    return i64::from_be_bytes(bits);
}

pub fn f64_fs (mut buf: usize, bytes: &[u8]) -> f64 {
    const LEN: usize = 8;
    let mut bits: [u8; LEN] = [0; LEN];
    for i in 0..bits.len() {
        bits[i] = bytes[buf];
        buf += 1;
    }   
    return f64::from_be_bytes(bits);
}

pub fn str_fs (mut buf: usize, bytes: &[u8], len: i32) -> String {
    let mut chars: Vec<char> = Vec::new();
    for _ in 0..len {
        chars.push(bytes[buf] as char);
        buf += 1;
    }   
    return chars.iter().collect();
}