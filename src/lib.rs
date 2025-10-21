//! [`Field`]: field::Field
//! [`Accessor`]: accessor::Accessor
//! [`FieldAccessorRegistry`]: accessor::FieldAccessorRegistry
//!
//! # Field Path
//!
//! **`field_path`** provides a lightweight and type-safe abstraction
//! for referencing and accessing nested fields within structs.
//!
//! The crate is designed to make it easier to generically inspect or
//! mutate fields without relying on heavy reflection systems or
//! unsafe code. It does this through a combination of field
//! identifiers and accessors that preserve type information.
//!
//! ## Core Concepts
//!
//! - **[`Field`]**: Represents a unique, type-safe identifier for a
//!   field path within a struct.
//! - **[`Accessor`]**: A generic wrapper providing read and write
//!   access to a field.
//! - **[`FieldAccessorRegistry`]**: A mapping between fields and their
//!   accessors for lookup and dynamic use.
//!
//! Together, these components allow building flexible systems that
//! can access or manipulate struct data without tightly coupling to
//! specific types.
//!
//! ## Example
//!
//! ```
//! use field_path::accessor::{FieldAccessorRegistry, accessor};
//! use field_path::field::field;
//!
//! #[derive(Default)]
//! struct Vec2<T> {
//!     pub x: T,
//!     pub y: T,
//! }
//!
//! let mut registry = FieldAccessorRegistry::default();
//! let field = field!(<Vec2<f32>>::x);
//!
//! // Register accessors.
//! registry.register_typed(field, accessor!(<Vec2<f32>>::x));
//!
//! // Access field generically.
//! let mut v = Vec2::default();
//! let accessor =
//!     registry.get::<Vec2<f32>, f32>(&field.untyped()).unwrap();
//!
//! *accessor.get_mut(&mut v) = 42.0;
//! assert_eq!(accessor.get_ref(&v), &42.0);
//! ```

#![no_std]

pub mod accessor;
pub mod field;
