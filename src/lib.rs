use blake2::{Blake2b, Digest};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use chrono::prelude::DateTime;
use chrono::Utc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hash(pub [u8; 64]);
pub type Hasher = Blake2b;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Timestamp(pub u64);

impl Timestamp {
    pub fn bytes(&self) -> [u8; 8] {
        self.0.bytes()
    }
    pub fn now() -> Self {
        let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        Self(duration.as_secs() as u64 * 1000 + duration.subsec_millis() as u64)
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let duration = UNIX_EPOCH + Duration::from_millis(self.0);
        let datetime = DateTime::<Utc>::from(duration);
        write!(formatter, "{}", datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string())
    }
}

impl std::default::Default for Hash {
    fn default() -> Self {
        Self([0; 64])
    }
}

impl std::fmt::Display for Hash {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", hex::encode(&self.0))
    }
}

impl Hash {

    pub fn difficulty(&self) -> u128 {
        (self.0[31] as u128) << 0xf * 8
            | (self.0[30] as u128) << 0xe * 8
            | (self.0[29] as u128) << 0xd * 8
            | (self.0[28] as u128) << 0xc * 8
            | (self.0[27] as u128) << 0xb * 8
            | (self.0[26] as u128) << 0xa * 8
            | (self.0[25] as u128) << 0x9 * 8
            | (self.0[24] as u128) << 0x8 * 8
            | (self.0[23] as u128) << 0x7 * 8
            | (self.0[22] as u128) << 0x6 * 8
            | (self.0[21] as u128) << 0x5 * 8
            | (self.0[20] as u128) << 0x4 * 8
            | (self.0[19] as u128) << 0x3 * 8
            | (self.0[18] as u128) << 0x2 * 8
            | (self.0[17] as u128) << 0x1 * 8
            | (self.0[16] as u128) << 0x0 * 8
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

pub trait Digestable {
    fn digest(&self, hasher: &mut Hasher);
}

pub trait Hashable {
    fn hash(&self) -> Hash;
}

impl<T: Digestable> Digestable for &[T] {
    fn digest(&self, hasher: &mut Hasher) {
        for item in self.iter() {
            item.digest(hasher);
        }
    }
}

impl<T: Digestable> Hashable for T {
    fn hash(&self) -> Hash {
        let mut hasher = Hasher::new();
        self.digest(&mut hasher);
        let hash = hasher.finalize();
        unsafe { std::mem::transmute(hash) }
    }
}

pub trait Int16Ext {
    fn bytes(self) -> [u8; 2];
}

impl Int16Ext for u16 {
    fn bytes(self) -> [u8; 2] {
        [(self >> 8 * 0x0) as u8, (self >> 8 * 0x1) as u8]
    }
}

pub trait Int32Ext {
    fn bytes(self) -> [u8; 4];
}

impl Int32Ext for u32 {
    fn bytes(self) -> [u8; 4] {
        [
            (self >> 8 * 0x0) as u8,
            (self >> 8 * 0x1) as u8,
            (self >> 8 * 0x2) as u8,
            (self >> 8 * 0x3) as u8,
        ]
    }
}

pub trait Int64Ext {
    fn bytes(self) -> [u8; 8];
}

impl Int64Ext for u64 {
    fn bytes(self) -> [u8; 8] {
        [
            (self >> 8 * 0x0) as u8,
            (self >> 8 * 0x1) as u8,
            (self >> 8 * 0x2) as u8,
            (self >> 8 * 0x3) as u8,
            (self >> 8 * 0x4) as u8,
            (self >> 8 * 0x5) as u8,
            (self >> 8 * 0x6) as u8,
            (self >> 8 * 0x7) as u8,
        ]
    }
}

pub trait Int128Ext {
    fn bytes(self) -> [u8; 16];
}

impl Int128Ext for u128 {
    fn bytes(self) -> [u8; 16] {
        [
            (self >> 8 * 0x0) as u8,
            (self >> 8 * 0x1) as u8,
            (self >> 8 * 0x2) as u8,
            (self >> 8 * 0x3) as u8,
            (self >> 8 * 0x4) as u8,
            (self >> 8 * 0x5) as u8,
            (self >> 8 * 0x6) as u8,
            (self >> 8 * 0x7) as u8,
            (self >> 8 * 0x8) as u8,
            (self >> 8 * 0x9) as u8,
            (self >> 8 * 0xa) as u8,
            (self >> 8 * 0xb) as u8,
            (self >> 8 * 0xc) as u8,
            (self >> 8 * 0xd) as u8,
            (self >> 8 * 0xe) as u8,
            (self >> 8 * 0xf) as u8,
        ]
    }
}

pub mod block;
pub mod chain;
pub mod transaction;
