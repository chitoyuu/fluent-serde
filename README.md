# fluent-serde

Provides third-party `serde` implementations for `fluent` argument and value types.

*This crate is not affiliated with Mozilla or Project Fluent in any way.*

```toml
[dependencies]
fluent = "0.16.0"
fluent-serde = "0.1.0"
```

# Example

```rust
use std::borrow::Cow;

use fluent::FluentValue;
use fluent::types::{FluentNumber, FluentNumberOptions};
use fluent_serde::ser::ArgsSerializer;
use serde::Serialize;

#[derive(Serialize)]
struct Foo {
    foo: i32,
}

#[derive(Serialize)]
struct Bar {
    bar: String,
}

let mut ser = ArgsSerializer::new();
Foo { foo: 42 }.serialize(&mut ser);
Bar { bar: "bar".into() }.serialize(&mut ser);
let args = ser.done();

assert_eq!(
    &FluentValue::Number(FluentNumber::new(42.0, FluentNumberOptions::default())),
    args.get("foo").unwrap(),
);

assert_eq!(
    &FluentValue::String(Cow::Owned("bar".into())),
    args.get("bar").unwrap(),
);
```

# License

MIT OR Apache-2.0
