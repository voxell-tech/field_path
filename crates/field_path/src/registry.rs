//! Accessor registry system for mapping source structures to target
//! fields via the [`AccessorRegistry`].

use core::hash::Hash;
use hashbrown::HashMap;

use crate::accessor::{Accessor, UntypedAccessor};
use crate::field::{Field, UntypedField};

/// An [`AccessorRegistry`] using [`UntypedField`] as the key type.
pub type FieldAccessorRegistry = AccessorRegistry<UntypedField>;

impl FieldAccessorRegistry {
    /// Registers a [`Field`] and [`Accessor`] pair in a type-safe
    /// manner.
    pub fn register_typed<S, T>(
        &mut self,
        field: Field<S, T>,
        accessor: Accessor<S, T>,
    ) {
        self.register(field.untyped(), accessor);
    }
}

/// A registry mapping keys to [`UntypedAccessor`]s.
///
/// Provides convenient registration of typed accessors and
/// retrieval as typed [`Accessor`]s with runtime checking.
///
/// # Example
/// ```
/// use field_path::accessor;
/// use field_path::registry::AccessorRegistry;
///
/// struct Foo { value: i32 }
///
/// let mut registry = AccessorRegistry::new();
/// registry.register("foo", accessor!(<Foo>::value));
///
/// let accessor = registry.get::<Foo, i32>(&"foo").unwrap();
/// let mut foo = Foo { value: 123 };
///
/// assert_eq!(accessor.get_ref(&foo), &123);
/// *accessor.get_mut(&mut foo) = 999;
/// assert_eq!(foo.value, 999);
/// ```
#[derive(Debug)]
pub struct AccessorRegistry<K> {
    accessors: HashMap<K, UntypedAccessor>,
}

impl<K> AccessorRegistry<K> {
    /// Construct an empty [`AccessorRegistry`].
    pub fn new() -> Self {
        Self {
            accessors: HashMap::new(),
        }
    }
}

impl<K: Eq + Hash> AccessorRegistry<K> {
    /// Registers an [`UntypedAccessor`] for a given key.
    ///
    /// Will overwrite existing accessor.
    pub fn register(
        &mut self,
        key: K,
        accessor: impl Into<UntypedAccessor>,
    ) {
        self.accessors.insert(key, accessor.into());
    }

    /// Retrieve a typed [`Accessor`] from the registry.
    ///
    /// Returns an [`AccessorRegErr`] if the key does not exist or
    /// if the types do not match.
    pub fn get<S: 'static, T: 'static>(
        &self,
        key: &K,
    ) -> Result<Accessor<S, T>, AccessorRegErr> {
        self.accessors
            .get(key)
            .ok_or(AccessorRegErr::KeyNotFound)?
            .typed()
            .ok_or(AccessorRegErr::TypeMismatch)
    }
}

impl<K> Default for AccessorRegistry<K> {
    fn default() -> Self {
        Self {
            accessors: HashMap::new(),
        }
    }
}

unsafe impl<K> Send for AccessorRegistry<K> {}
unsafe impl<K> Sync for AccessorRegistry<K> {}

/// Possible error variants when getting an [`Accessor`]
/// from the [`AccessorRegistry`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessorRegErr {
    /// The requested key was not found in the registry.
    KeyNotFound,
    /// The [`Accessor`] exists but the source/target types did
    /// not match.
    TypeMismatch,
}

#[cfg(test)]
mod tests {
    use crate::accessor;

    use super::*;

    #[derive(Debug, PartialEq)]
    struct Foo {
        x: i32,
        y: f32,
    }

    #[test]
    fn registry_register_and_get_success() {
        let mut registry: AccessorRegistry<&'static str> =
            AccessorRegistry::new();

        registry.register("foo_x", accessor!(<Foo>::x));
        registry.register("foo_y", accessor!(<Foo>::y));

        let mut foo = Foo { x: 10, y: 1.5 };

        let x_accessor = registry.get::<Foo, i32>(&"foo_x").unwrap();
        assert_eq!(x_accessor.get_ref(&foo), &10);

        let y_accessor = registry.get::<Foo, f32>(&"foo_y").unwrap();
        assert_eq!(y_accessor.get_ref(&foo), &1.5);

        // Mutate via accessor
        *x_accessor.get_mut(&mut foo) = 77;
        *y_accessor.get_mut(&mut foo) = 2.5;

        assert_eq!(foo.x, 77);
        assert_eq!(foo.y, 2.5);
    }

    #[test]
    fn registry_key_not_found_error() {
        let registry: AccessorRegistry<&'static str> =
            AccessorRegistry::new();

        let res = registry.get::<Foo, i32>(&"missing");
        assert!(matches!(res, Err(AccessorRegErr::KeyNotFound)));
    }

    #[test]
    fn registry_type_mismatch_error() {
        let mut registry: AccessorRegistry<&'static str> =
            AccessorRegistry::new();

        registry.register("foo_x", accessor!(<Foo>::x));

        let res = registry.get::<Foo, f32>(&"foo_x");
        assert!(matches!(res, Err(AccessorRegErr::TypeMismatch)));
    }
}

#[macro_magic::export_tokens]
#[derive(Debug, PartialEq)]
pub struct Loo {
    pub l: i32,
    pub o: u32,
}

#[macro_magic::export_tokens]
#[derive(Debug, PartialEq)]
pub struct Moo {
    pub m: i32,
    pub o: u32,
}
