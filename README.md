# Optbuilder

This crate was born out of my laziness while implementing a wrapper for the Telegram API. Many methods of the API required parameters that were mostly optional, leaving me in a situation where I would write code like this:

```rust
struct Foo {
    name: String,
    age: u32,
    address: Option<String>,
    job: Option<String>,
    pet_name: Option<String>
}

impl Foo {
    fn with_address(self, addr: String) -> self {
        self.address = Some(addr);
    }

    fn with_job(self, job: String) -> self {
        self.job = Some(job);
    }

    fn with_pet_name(self, pet_name: String) -> self {
        self.pet_name = Some(pet_name);
    }
}
```

instead of implementing all the methods to inject address, job and pet_name a macro was created for that purpose:

```rust
extern crate optbuilder;
use optbuilder::OptionalBuilder;

#[derive(OptionalBuilder)]
struct Foo {
    name: String,
    age: u32,
    address: Option<String>,
    // We don't want to codegen job for some reason
    #[optbuilder(skip)]
    job: Option<String>,
    pet_name: Option<String>
}
```

## How to use this?

Add the following lines to your Cargo.toml
```toml
[dependencies]
optbuilder = "0.1.1"
```

## Why not use a builder crate?

Basically because many times I had a default implementation or a reasonable constructor and the optional fields were defaulted to None.

## Issues

In some situations, the compiler may complain about types, since the From trait is used on the builder arguments, errros like this one can happen:

```rust
use optbuilder::OptionalBuilder;

#[derive(Debug, Default, OptionalBuilder)]
struct Foo {
    name: String,
    id: Option<i32>,
    uid: Option<u32>,
}

fn main() {
    let f = Foo::default().with_id(2).with_uid(3);

    println!("{:?}", f);
}

error[E0277]: the trait bound `u32: std::convert::From<i32>` is not satisfied
  --> src/main.rs:11:39
   |
11 |     let f = Foo::default().with_id(2).with_uid(3);
   |                                       ^^^^^^^^ the trait `std::convert::From<i32>` is not implemented for `u32`
```
