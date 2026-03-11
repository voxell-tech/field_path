use crate::accessor::Accessor;
use crate::field::Field;

pub struct FieldAccessor<S, T>
where
    S: 'static,
    T: 'static,
{
    pub field: Field<S, T>,
    pub accessor: Accessor<S, T>,
}

impl<S, T> FieldAccessor<S, T>
where
    S: 'static,
    T: 'static,
{
    pub const fn new(
        field: Field<S, T>,
        accessor: Accessor<S, T>,
    ) -> Self {
        Self { field, accessor }
    }
}

/// Creates a [`FieldAccessor`] that ensures both [`Field`] and
/// [`Accessors`] are pointing to the same field path.
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
