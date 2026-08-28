//! This module defines the [`Path`] type, which pairs a
//! [`Field`] (representing the static path name) with an
//! [`Lens`] (providing functional pointers for data access).
//!
//! Use the [`path!`] macro to ensure that the path name and
//! the access logic are always synchronized to the same field.

use core::hash::{Hash, Hasher};

use crate::field::Field;
use crate::lens::Lens;

// For docs.
#[expect(unused_imports)]
use crate::path;

/// A specialized container pairing a [`Field`] with its [`Lens`].
#[derive(Debug, Clone, Copy)]
pub struct Path<S, T> {
    pub field: Field<S, T>,
    pub lens: Lens<S, T>,
}

impl<S, T> Path<S, T> {
    #[inline]
    pub const fn new(field: Field<S, T>, lens: Lens<S, T>) -> Self {
        Self { field, lens }
    }
}

impl<S, T> Hash for Path<S, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.field.hash(state);
    }
}

impl<S, T> PartialEq for Path<S, T> {
    fn eq(&self, other: &Self) -> bool {
        self.field.eq(&other.field)
    }
}

impl<S, T> Eq for Path<S, T> {}

// impl<S, T> for

/// Creates a [`Path`] that ensures both [`Field`] and
/// [`Lens`] are pointing to the same field path.
///
/// ## Example
///
/// ```
/// use field_path::path;
/// use field_path::path::Path;
///
/// struct Foo { value: i32 }
///
/// const FOO_FIELD_ACC: Path<Foo, i32> = path!(<Foo>::value);
///
/// assert_eq!(FOO_FIELD_ACC.field.field_path(), "::value");
///
/// let mut foo = Foo { value: 42 };
///
/// assert_eq!(FOO_FIELD_ACC.lens.get_ref(&foo), &42);
/// *FOO_FIELD_ACC.lens.get_mut(&mut foo) = 999;
/// assert_eq!(foo.value, 999);
/// ```
#[macro_export]
macro_rules! path {
    (<$source:ty>$(::$field:tt)*) => {
        $crate::path::Path::new(
            $crate::field!(<$source>$(::$field)*),
            $crate::lens!(<$source>$(::$field)*),
        )
    };
}
