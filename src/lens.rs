//! Lenses for mapping source structures to target fields.
//!
//! This module defines the core [`Lens`] and [`UntypedLens`]
//! types, providing a way to abstract over field access within a
//! source structure.
//!
//! Use the [`lens!`] macro to safely generate lenses. It
//! ensures that the immutable and mutable paths to a field are
//! identical, preventing logical errors.

use core::any::TypeId;

use func_pointers::{MutFn, MutFnPtr, RefFn, RefFnPtr};

pub mod func_pointers;

// For docs.
#[expect(unused_imports)]
use crate::lens;

/// A typed lens to a field of type `T` within a source type `S`.
///
/// This holds both immutable and mutable function pointers, which
/// allows retrieving references to the target field inside a source.
///
/// ## Validation
///
/// The [`lens!`] macro ensures that both immutable and mutable
/// references are pointing towards the same field. Constructing
/// `Lens` manually may result in mismatches.
///
/// ## Example
///
/// ```
/// use field_path::lens::Lens;
///
/// struct Foo { value: i32 }
///
/// fn ref_fn(s: &Foo) -> &i32 { &s.value }
/// fn mut_fn(s: &mut Foo) -> &mut i32 { &mut s.value }
///
/// const FOO_ACC: Lens<Foo, i32> = Lens::new(ref_fn, mut_fn);
/// let mut foo = Foo { value: 42 };
///
/// assert_eq!(FOO_ACC.get_ref(&foo), &42);
/// *FOO_ACC.get_mut(&mut foo) = 999;
/// assert_eq!(foo.value, 999);
/// ```
#[derive(Debug)]
pub struct Lens<S, T> {
    ref_fn: RefFn<S, T>,
    mut_fn: MutFn<S, T>,
}

impl<S, T> Lens<S, T> {
    #[inline]
    pub const fn new(
        ref_fn: fn(&S) -> &T,
        mut_fn: fn(&mut S) -> &mut T,
    ) -> Self {
        Self { ref_fn, mut_fn }
    }

    /// Get an immutable reference to the target type.
    #[inline]
    pub fn get_ref<'a>(&self, source: &'a S) -> &'a T {
        (self.ref_fn)(source)
    }

    /// Get a mutable reference to the target type.
    #[inline]
    pub fn get_mut<'a>(&self, source: &'a mut S) -> &'a mut T {
        (self.mut_fn)(source)
    }
}

impl<S, T> Lens<S, T>
where
    S: 'static,
    T: 'static,
{
    /// Erases the type by converting it into an [`UntypedLens`].
    #[inline]
    pub const fn untyped(&self) -> UntypedLens {
        UntypedLens::new(self.ref_fn, self.mut_fn)
    }
}

impl<S, T> Clone for Lens<S, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S, T> Copy for Lens<S, T> {}

/// Creates a [`Lens`] that ensures the fields being accessed are
/// correct for both immutable and mutable reference.
///
/// ## Example
///
/// ```
/// use field_path::lens;
/// use field_path::lens::Lens;
///
/// struct Foo { value: i32 }
///
/// const FOO_ACC: Lens<Foo, i32> = lens!(<Foo>::value);
/// let mut foo = Foo { value: 42 };
///
/// assert_eq!(FOO_ACC.get_ref(&foo), &42);
/// *FOO_ACC.get_mut(&mut foo) = 999;
/// assert_eq!(foo.value, 999);
/// ```
#[macro_export]
macro_rules! lens {
    (<$source:ty>) => {
        $crate::lens::Lens::new(
            #[inline(always)]
            |s: &$source| s,
            #[inline(always)]
            |s: &mut $source| s,
        )
    };
    (<$source:ty>$(::$field:tt)+) => {
        $crate::lens::Lens::new(
            #[inline(always)]
            |s: &$source| &s$(.$field)+,
            #[inline(always)]
            |s: &mut $source| &mut s$(.$field)+
        )
    };
}

/// A type-erased version of [`Lens`].
///
/// Stores the raw function pointers along with [`TypeId`]s of both
/// source and target. This allows dynamically checking and restoring
/// the original [`Lens`].
#[derive(Debug, Clone, Copy)]
pub struct UntypedLens {
    ref_fn: RefFnPtr,
    mut_fn: MutFnPtr,
    source_id: TypeId,
    target_id: TypeId,
}

impl UntypedLens {
    /// Create a new type-erased lens from a typed lens pair.
    #[inline]
    pub const fn new<S, T>(
        ref_fn: fn(&S) -> &T,
        mut_fn: fn(&mut S) -> &mut T,
    ) -> Self
    where
        S: 'static,
        T: 'static,
    {
        Self {
            ref_fn: RefFnPtr::new(ref_fn),
            mut_fn: MutFnPtr::new(mut_fn),
            source_id: TypeId::of::<S>(),
            target_id: TypeId::of::<T>(),
        }
    }

    /// Re-interpret this lens as a typed [`Lens`] without
    /// checking [`TypeId`]s. Caller must guarantee type correctness.
    ///
    /// # Safety
    ///
    /// Undefined behavior if `S` and `T` do not match the types used
    /// when constructing this lens.
    pub const unsafe fn typed_unchecked<S, T>(self) -> Lens<S, T> {
        unsafe {
            Lens {
                ref_fn: self.ref_fn.typed_unchecked::<S, T>(),
                mut_fn: self.mut_fn.typed_unchecked::<S, T>(),
            }
        }
    }

    /// Attempt to re-interpret this lens as a typed [`Lens`],
    /// returning `None` if the [`TypeId`]s do not match.
    pub fn typed<S, T>(self) -> Option<Lens<S, T>>
    where
        S: 'static,
        T: 'static,
    {
        if self.source_id == TypeId::of::<S>()
            && self.target_id == TypeId::of::<T>()
        {
            // SAFETY: The types are checked beforehand.
            unsafe {
                return Some(self.typed_unchecked());
            }
        }

        None
    }
}

impl<S, T> From<Lens<S, T>> for UntypedLens
where
    S: 'static,
    T: 'static,
{
    #[inline]
    fn from(lens: Lens<S, T>) -> Self {
        lens.untyped()
    }
}

impl<S, T> From<&Lens<S, T>> for UntypedLens
where
    S: 'static,
    T: 'static,
{
    #[inline]
    fn from(lens: &Lens<S, T>) -> Self {
        lens.untyped()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Foo {
        x: i32,
        y: f32,
    }

    #[test]
    fn lens_roundtrip_typed_untyped() {
        let acc: Lens<Foo, i32> = lens!(<Foo>::x);

        let untyped = acc.untyped();
        let typed_back: Lens<Foo, i32> = untyped.typed().unwrap();

        let mut foo = Foo { x: 42, y: 1.5 };

        assert_eq!((typed_back.ref_fn)(&foo), &42);

        let x_mut = (typed_back.mut_fn)(&mut foo);
        *x_mut = 99;

        assert_eq!(foo.x, 99);
    }

    #[test]
    fn untyped_typed_mismatch_fails() {
        let acc: Lens<Foo, i32> = lens!(<Foo>::x);

        let untyped = acc.untyped();

        // Mismatched type parameters should return None
        let wrong: Option<Lens<Foo, f32>> = untyped.typed();
        assert!(wrong.is_none());
    }
}
