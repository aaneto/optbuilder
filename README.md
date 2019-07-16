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
optbuilder = "0.1.0"
```