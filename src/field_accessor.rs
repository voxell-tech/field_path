//! This module defines the [`FieldAccessor`] type, which pairs a
//! [`Field`] (representing the static path name) with an
//! [`Accessor`] (providing functional pointers for data access).
//!
//! Use the [`field_accessor!`] macro to ensure that the path name and
//! the access logic are always synchronized to the same field.

use core::hash::{Hash, Hasher};

use crate::accessor::Accessor;
use crate::field::Field;

// For docs.
#[expect(unused_imports)]
use crate::field_accessor;

/// A specialized container pairing a [`Field`] with its [`Accessor`].
#[derive(Debug, Clone, Copy)]
pub struct FieldAccessor<S, T> {
    pub field: Field<S, T>,
    pub accessor: Accessor<S, T>,
}

impl<S, T> FieldAccessor<S, T> {
    #[inline]
    pub const fn new(
        field: Field<S, T>,
        accessor: Accessor<S, T>,
    ) -> Self {
        Self { field, accessor }
    }
}

impl<S, T> Hash for FieldAccessor<S, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.field.hash(state);
    }
}

impl<S, T> PartialEq for FieldAccessor<S, T> {
    fn eq(&self, other: &Self) -> bool {
        self.field.eq(&other.field)
    }
}

impl<S, T> Eq for FieldAccessor<S, T> {}

// impl<S, T> for

/// Creates a [`FieldAccessor`] that ensures both [`Field`] and
/// [`Accessor`] are pointing to the same field path.
///
/// ## Example
///
/// ```
/// use field_path::field_accessor;
/// use field_path::field_accessor::FieldAccessor;
///
/// struct Foo { value: i32 }
///
/// const FOO_FIELD_ACC: FieldAccessor<Foo, i32> = field_accessor!(<Foo>::value);
///
/// assert_eq!(FOO_FIELD_ACC.field.field_path(), "::value");
///
/// let mut foo = Foo { value: 42 };
///
/// assert_eq!(FOO_FIELD_ACC.accessor.get_ref(&foo), &42);
/// *FOO_FIELD_ACC.accessor.get_mut(&mut foo) = 999;
/// assert_eq!(foo.value, 999);
/// ```
#[macro_export]
macro_rules! field_accessor {
    (<$source:ty>$(::$field:tt)*) => {
        $crate::field_accessor::FieldAccessor::new(
            $crate::field!(<$source>$(::$field)*),
            $crate::accessor!(<$source>$(::$field)*),
        )
    };
}
