# r-lombok-macros

[![Crates.io](https://img.shields.io/crates/v/r-lombok-macros)](https://crates.io/crates/r-lombok-macros)
[![Documentation](https://docs.rs/r-lombok-macros/badge.svg)](https://docs.rs/r-lombok-macros)

Macros for [`r-lombok-macros`]. it's a rust macros that automatically plugs into your editor and build tools,like java
lombok

More information about this crate can be found in the [crate documentation][docs].

## Usage example

```rust
use std::fmt::Debug;
use r_lombok_macros::{Getter, Setter};

#[derive(Debug, Getter, Setter)]
struct Company {
    name: &'static str,
    boss: &'static str,
    number: u32,
    department: Vec<String>,
}

#[derive(Getter, Setter)]
struct CompanyGen<T> where T: Debug {
    name: T,
    boss: &'static str,
    number: u32,
    department: Vec<String>,
}

#[derive(Getter, Setter)]
struct CompanyWrap {
    sub_company: CompanyGen<Company>,
    name: &'static str,
}

// Unit struct
#[derive(Getter, Setter, Debug)]
struct UnitStruct {}

fn main() {
    let mut xp = Company {
        name: "xp",
        boss: "Big Brother",
        number: u32::MAX,
        department: vec!["HR".to_owned(), "Finance".to_owned()],
    };
    println!("xp name = {:?} boss = {:?} number = {:?} department = {:?}", xp.get_name(), xp.get_boss(), xp.get_number(), xp.get_department());
    xp.set_name("set_name");
    xp.set_boss("set_boss");
    xp.set_number(u32::MIN);
    xp.set_department(vec!["department".to_owned()]);


    let xp_t = CompanyGen::<Company> {
        name: xp,
        boss: "Big Brother",
        number: u32::MAX,
        department: vec!["HR".to_owned(), "Finance".to_owned()],
    };
    println!("xp_t name = {:?} boss = {:?} number = {:?} department = {:?}", xp_t.get_name(), xp_t.get_boss(), xp_t.get_number(), xp_t.get_department());

    let xp_wrap = CompanyWrap {
        sub_company: xp_t,
        name: "xp_wrap",
    };
    println!("xp_wrap name = {:?} sub_company = {:?}", xp_wrap.get_name(), xp_wrap.get_sub_company().get_name());
    let unit = UnitStruct {};
    println!("unit = {:?}", unit);
}

```

see [test][test] for more example

## Getting Help

You're also welcome to open an [issue] with your question.

## License

This project is licensed under the [MIT license][license].

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `r-lombok-macros` by
you, shall be licensed as MIT, without any additional terms or conditions.

[docs]: https://docs.rs/r-lombok-macros

[issue]: https://github.com/dengzhq/r-lombok-macros/issues/new

[test]: https://github.com/dengzhq/r-lombok-macros/tree/master/tests

[license]: https://github.com/dengzhq/r-lombok-macros/blob/master/LICENSE