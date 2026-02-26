#![cfg_attr(not(feature = "std"), no_std)]

pub mod can;
pub mod status;
pub mod crc;
pub mod convert;
pub mod version;
pub mod spi;
pub mod modbus;

#[cfg(feature = "std")]
pub mod http;
