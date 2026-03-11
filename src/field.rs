//! Provides abstractions for working with typed fields in a
//! type-erased, yet type-safe manner.
//!
//! A [`Field`] represents a way to identify and a field path within a
//! data structure.
//!
//! This module is fully independent and can be used in isolation.
//! It is intended to serve as a flexible building block for systems
//! that need to store, compare, or retrieve fields dynamically.

use core::any::TypeId;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;

// For docs.
#[expect(unused_imports)]
use crate::field;

/// A statically typed field path from a source type `S` to a target
/// type `T`.
///
/// It uniquely identifies a target field path within a source `struct`
/// through the `field_path`. The type parameters encode both the
/// source type `S` and the resolved target type `T`.
///
/// A `Field` can also be created at compile time, allowing us to
/// create `const` or `static` fields.
///
/// # Validation
///
/// The [`field!`] macro ensures that both the path and type used are
/// valid. Constructing `Field` manually may result in mismatches.
///
/// # Example
/// ```
/// use field_path::field::Field;
/// use field_path::field;
/// use field_path::stringify_field;
///
/// struct Player {
///     name: String,
///     age: u32,
/// }
///
/// const PLAYER_AGE: Field<Player, u32> = Field::new(
///     stringify_field!(::age)
/// );
///
/// assert_eq!(PLAYER_AGE.field_path(), "::age");
/// ```
#[derive(Debug)]
pub struct Field<S, T> {
    /// The path of the target field in the source.
    ///
    /// Example: `Transform::translation::x` will have a field path
    /// of `"::translation::x"`.
    field_path: &'static str,
    _marker: PhantomData<(S, T)>,
}

impl<S, T> Field<S, T> {
    /// A field with a placeholder field path. This does not correspond
    /// to a vaild field path!
    pub const PLACEHOLDER: Self = Self::new("$");

    /// Construct a new [`Field`] from a raw field path string.
    ///
    /// Prefer the [`field!`] macro for type safety!
    #[inline]
    pub const fn new(field_path: &'static str) -> Self {
        Self {
            field_path,
            _marker: PhantomData,
        }
    }

    /// Returns the raw field path string.
    pub const fn field_path(&self) -> &'static str {
        self.field_path
    }
}

impl<S, T> Field<S, T>
where
    S: 'static,
    T: 'static,
{
    /// Erases the type by converting it into an [`UntypedField`].
    #[inline]
    pub const fn untyped(&self) -> UntypedField {
        UntypedField::new::<S, T>(self.field_path)
    }
}

impl<S, T> Hash for Field<S, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.field_path.hash(state);
    }
}

impl<S, T> PartialEq for Field<S, T> {
    fn eq(&self, other: &Self) -> bool {
        self.field_path == other.field_path
    }
}

impl<S, T> Eq for Field<S, T> {}

impl<S, T> PartialOrd for Field<S, T> {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<S, T> Ord for Field<S, T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.field_path.cmp(other.field_path)
    }
}

impl<S, T> Clone for Field<S, T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<S, T> Copy for Field<S, T> {}

/// Builder used internally by the [`field!`] macro to construct
/// [`Field`]s.
///
/// Ensures type correctness by using a marker function signature.
pub struct _FieldBuilder<S, T> {
    /// A marker function to identify the validity of the path
    /// in the [`field!`] macro.
    _field_marker: fn(source: S) -> T,
    /// See [`Field::field_path`]
    field_path: &'static str,
}

impl<S, T> _FieldBuilder<S, T> {
    #[inline]
    pub const fn new(
        field_marker: fn(source: S) -> T,
        field_path: &'static str,
    ) -> Self {
        Self {
            field_path,
            _field_marker: field_marker,
        }
    }

    /// Creates the [`Field`] struct.
    #[inline]
    pub const fn build(self) -> Field<S, T> {
        Field {
            field_path: self.field_path,
            _marker: PhantomData,
        }
    }
}

/// Creates a [`Field`] with path and type safety.
///
/// # Example
///
/// ```
/// use field_path::field::Field;
/// use field_path::field;
///
/// struct Player {
///     name: String,
///     age: u32,
/// }
///
/// const PLAYER_NAME: Field<Player, String> = field!(<Player>::name);
/// let PLAYER_AGE: Field<Player, u32> = field!(<Player>::age);
///
/// assert_ne!(PLAYER_NAME.untyped(), PLAYER_AGE.untyped());
/// ```
#[macro_export]
macro_rules! field {
    (<$source:ty>$(::$field:tt)*) => {
        $crate::field::_FieldBuilder::new(
            |source: $source| source$(.$field)*,
            $crate::stringify_field!($(::$field)*),
        )
        .build()
    };
}

/// A type-erased version of [`Field`]. It uniquely identifies a
/// target field path within a source `struct`.
///
/// # Validation
///
/// The field path used must be valid. To prevent using the wrong
/// path, create one via [`Field::untyped`].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct UntypedField {
    /// The [`TypeId`] of the source type.
    source_id: TypeId,
    /// The [`TypeId`] of the target type.
    target_id: TypeId,
    /// See [`Field::field_path`].
    field_path: &'static str,
}

impl UntypedField {
    #[inline]
    pub const fn new<S, T>(field_path: &'static str) -> Self
    where
        S: 'static,
        T: 'static,
    {
        Self {
            source_id: TypeId::of::<S>(),
            target_id: TypeId::of::<T>(),
            field_path,
        }
    }

    /// Creates a placeholder.
    ///
    /// This does not represent any valid path.
    #[inline]
    pub const fn placeholder() -> Self {
        Self::placeholder_with_path("$")
    }

    /// Creates a placeholder with a custom path.
    ///
    /// The path may be valid, but the types will be empty.
    #[inline]
    pub const fn placeholder_with_path(
        field_path: &'static str,
    ) -> Self {
        Self::new::<(), ()>(field_path)
    }

    /// Get the [`TypeId`] of the source type.
    #[inline]
    pub const fn source_id(&self) -> TypeId {
        self.source_id
    }

    /// Get the [`TypeId`] of the target type.
    #[inline]
    pub const fn target_id(&self) -> TypeId {
        self.target_id
    }

    /// See [`Field::field_path`].
    #[inline]
    pub const fn field_path(&self) -> &'static str {
        self.field_path
    }

    /// Attempt to re-interpret this accessor as a typed [`Field`],
    /// returning `None` if the [`TypeId`]s do not match.
    #[inline]
    pub fn typed<S, T>(self) -> Option<Field<S, T>>
    where
        S: 'static,
        T: 'static,
    {
        if self.source_id == TypeId::of::<S>()
            && self.target_id == TypeId::of::<T>()
        {
            return Some(self.typed_unchecked());
        }

        None
    }

    /// Converts into a typed [`Field<S, T>`] without type checks.
    #[inline]
    pub const fn typed_unchecked<S, T>(self) -> Field<S, T> {
        Field::new(self.field_path)
    }
}

impl<S, T> From<Field<S, T>> for UntypedField
where
    S: 'static,
    T: 'static,
{
    #[inline]
    fn from(field: Field<S, T>) -> Self {
        field.untyped()
    }
}

impl<S, T> From<&Field<S, T>> for UntypedField
where
    S: 'static,
    T: 'static,
{
    #[inline]
    fn from(field: &Field<S, T>) -> Self {
        field.untyped()
    }
}

/// Stringify a field path into its canonical string form.
///
/// This macro is used within the [`field!`] macro for supporting
/// auto-completion of nested fields while still being able to generate
/// "stringify" field paths from raw tokens!
///
/// # Example
///
/// ```
/// use field_path::stringify_field;
///
/// let stringify = stringify_field!(::translation::x);
/// assert_eq!(stringify, "::translation::x");
/// ```
#[macro_export]
macro_rules! stringify_field {
    ($(::$field:tt)*) => {
        concat!($("::", stringify!($field),)*)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Debug, Clone)]
    struct Foo(u32);

    #[derive(PartialEq, Debug, Clone)]
    struct NestedFoo {
        inner: Foo,
    }

    #[test]
    fn field_path_matches() {
        const FIELD: Field<Foo, Foo> = field!(<Foo>);
        assert_eq!(FIELD.field_path, "");

        const FIELD_0: Field<Foo, u32> = field!(<Foo>::0);
        assert_eq!(FIELD_0.field_path, stringify_field!(::0), "::0");

        const FIELD_INNER_0: Field<NestedFoo, u32> =
            field!(<NestedFoo>::inner::0);
        assert_eq!(
            FIELD_INNER_0.field_path,
            stringify_field!(::inner::0),
            "::inner::0"
        );
    }
}
