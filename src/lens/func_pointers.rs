/// A type-erased immutable field accessor function pointer.
#[derive(Debug, Clone, Copy)]
pub struct RefFnPtr(*const ());

unsafe impl Send for RefFnPtr {}
unsafe impl Sync for RefFnPtr {}

impl RefFnPtr {
    /// Creates a new erased type of [`RefFn<S, T>`].
    pub const fn new<S, T>(f: RefFn<S, T>) -> Self {
        Self(f as *const ())
    }

    /// Re-interprets this pointer as a typed [`RefFn`] without
    /// checking type correctness.
    ///
    /// # Safety
    ///
    /// Undefined behavior if `S` and `T` do not match the types used
    /// when constructing this pointer.
    pub const unsafe fn typed_unchecked<S, T>(&self) -> RefFn<S, T> {
        unsafe {
            core::mem::transmute::<*const (), RefFn<S, T>>(self.0)
        }
    }
}

/// A type-erased mutable field accessor function pointer.
#[derive(Debug, Clone, Copy)]
pub struct MutFnPtr(*const ());

unsafe impl Send for MutFnPtr {}
unsafe impl Sync for MutFnPtr {}

impl MutFnPtr {
    /// Creates a new erased type of [`MutFn<S, T>`].
    pub const fn new<S, T>(f: MutFn<S, T>) -> Self {
        Self(f as *const ())
    }

    /// Re-interprets this pointer as a typed [`MutFn`] without
    /// checking type correctness.
    ///
    /// # Safety
    ///
    /// Undefined behavior if `S` and `T` do not match the types used
    /// when constructing this pointer.
    pub const unsafe fn typed_unchecked<S, T>(&self) -> MutFn<S, T> {
        unsafe {
            core::mem::transmute::<*const (), MutFn<S, T>>(self.0)
        }
    }
}

/// Immutable field accessor function: `fn(&S) -> &T`.
pub type RefFn<S, T> = fn(&S) -> &T;

/// Mutable field accessor function: `fn(&mut S) -> &mut T`.
pub type MutFn<S, T> = fn(&mut S) -> &mut T;

#[cfg(test)]
mod tests {
    use super::*;

    struct Foo {
        x: i32,
        y: f32,
    }

    fn foo_x_ref(s: &Foo) -> &i32 {
        &s.x
    }
    fn foo_x_mut(s: &mut Foo) -> &mut i32 {
        &mut s.x
    }
    fn foo_y_ref(s: &Foo) -> &f32 {
        &s.y
    }

    #[test]
    fn ref_fn_ptr_roundtrip() {
        let ptr = RefFnPtr::new(foo_x_ref);
        let f: RefFn<Foo, i32> = unsafe { ptr.typed_unchecked() };
        let foo = Foo { x: 42, y: 1.5 };
        assert_eq!(f(&foo), &42);
    }

    #[test]
    fn mut_fn_ptr_roundtrip() {
        let ptr = MutFnPtr::new(foo_x_mut);
        let f: MutFn<Foo, i32> = unsafe { ptr.typed_unchecked() };
        let mut foo = Foo { x: 0, y: 1.5 };
        *f(&mut foo) = 99;
        assert_eq!(foo.x, 99);
    }

    #[test]
    fn ref_fn_ptr_preserves_identity() {
        let ptr_x = RefFnPtr::new(foo_x_ref);
        let ptr_y = RefFnPtr::new(foo_y_ref);
        let foo = Foo { x: 7, y: 1.5 };

        let fx: RefFn<Foo, i32> = unsafe { ptr_x.typed_unchecked() };
        let fy: RefFn<Foo, f32> = unsafe { ptr_y.typed_unchecked() };

        assert_eq!(fx(&foo), &7);
        assert_eq!(fy(&foo), &1.5);
    }
}
