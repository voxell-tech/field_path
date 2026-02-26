//! Auto registration via the [`linkme`] crate.

use crate::accessor::UntypedAccessor;
use crate::field::UntypedField;
use linkme::distributed_slice;

pub use linkme;
pub use macro_magic;

#[distributed_slice]
pub static FIELD_REGISTRATIONS: [FieldRegistration];

#[derive(Clone)]
pub struct FieldRegistration {
    pub field: UntypedField,
    pub accessor: UntypedAccessor,
}

pub trait FieldNames {
    fn create_registrations<S>(
        registration: FieldRegistration,
    ) -> &'static [FieldRegistration];
}

// #[macro_export]
// macro_rules! use_full {
//     (<$($path:tt)::+>::{$name:ident}) => {
//         use $($path::)+$name;
//         macro_rules! $name {
//             () => {
//                 $($path::)+$name
//             };
//         }
//     };
// }

pub trait FieldReg {
    fn field_registrations() -> &'static [FieldRegistration];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[field_path_derive::remember_path]
    use crate::registry::{Loo, Moo};

    // #[combine_structs(super::super::registry::Loo)]
    #[macro_magic::export_tokens]
    #[derive(Debug, PartialEq)]
    struct Foo {
        pub x: i32,
        pub y: i32,
        pub loo: Loo,
    }

    fn test() {
        let result = macro_magic::forward_tokens!(Foo, stringify);
        let loo: ___full_path_Loo!() = Loo { l: 1, o: 2 };
        let moo: ___full_path_Moo!() = Moo { m: 3, o: 4 };
    }

    // impl FieldNames for Foo {
    //     // fn field_names() -> &'static [&'static str] {
    //     //     &["x", "y"]
    //     // }

    //     fn create_registrations<S>(
    //         registration: AutoFieldRegistration,
    //     ) -> &'static [AutoFieldRegistration] {
    //         unsafe {
    //             let ref_fn = registration
    //                 .accessor
    //                 .typed_unchecked::<S, Self>()
    //                 .ref_fn();

    //             &[AutoFieldRegistration {
    //                 field: registration.field.concat("x"),
    //                 accessor: Accessor::<S, i32>::new(
    //                     |s| &ref_fn(s).x,
    //                     |s| {},
    //                 )
    //                 .untyped(),
    //             }]
    //         }
    //     }
    // }

    #[test]
    fn auto_reg_creates_correct_registration() {
        // let x = Foo {
        //     x: 1,
        //     y: 2,
        //     z: 3,
        //     a: 1,
        // };

        // panic!("{:?}", Foo::recursive_fields());
    }
}
