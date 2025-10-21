# Field Path

**`field_path`** provides a lightweight and type-safe abstraction
for referencing and accessing nested fields within structs.

The crate is designed to make it easier to generically inspect or
mutate fields without relying on heavy reflection systems or
unsafe code. It does this through a combination of field
identifiers and accessors that preserve type information.

## Core Concepts

- `Field`: Represents a unique, type-safe identifier for a
  field path within a struct.
- `Accessor`: A generic wrapper providing read and write
  access to a field.
- `FieldAccessorRegistry`: A mapping between fields and their
  accessors for lookup and dynamic use.

Together, these components allow building flexible systems that
can access or manipulate struct data without tightly coupling to
specific types.

## Example

```rs
use field_path::accessor::{Accessor, FieldAccessorRegistry};
use field_path::field::field;

#[derive(Default)]
struct Vec2<T> {
    pub x: T,
    pub y: T,
}

let mut registry = FieldAccessorRegistry::default();
let field = field!(<Vec2<f32>>::x).untyped();

// Register accessors.
registry.register(
    field,
    Accessor {
        ref_fn: |v: &Vec2<f32>| &v.x,
        mut_fn: |v: &mut Vec2<f32>| &mut v.x,
    },
);

// Access field generically.
let mut v = Vec2::default();
let accessor = registry.get::<Vec2<f32>, f32>(&field).unwrap();

*(accessor.mut_fn)(&mut v) = 42.0;
assert_eq!(*(accessor.ref_fn)(&v), 42.0);

```

## Join the community!

You can join us on the [Voxell discord server](https://discord.gg/Mhnyp6VYEQ).

## License

`field_path` is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.
