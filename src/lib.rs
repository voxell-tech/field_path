#![doc = include_str!("../README.md")]
#![no_std]

pub mod accessor;
pub mod field;
#[cfg(feature = "registry")]
pub mod registry;
