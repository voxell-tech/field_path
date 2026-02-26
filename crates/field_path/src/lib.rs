#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "auto_registry")]
extern crate alloc;

pub mod accessor;
pub mod field;

#[cfg(feature = "registry")]
pub mod registry;

#[cfg(feature = "auto_registry")]
pub mod auto_registry;
