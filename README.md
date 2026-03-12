# uncollate
[![Static Badge](https://img.shields.io/badge/github-repo-blue?logo=github)
](https://github.com/wdjcodes/uncollate)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/wdjcodes/uncollate/ci.yml?logo=github)
](https://github.com/wdjcodes/uncollate/actions)
[![docs.rs](https://img.shields.io/docsrs/uncollate?logo=rust)
](https://docs.rs/uncollate/latest/uncollate/)
[![Crates.io Version](https://img.shields.io/crates/v/uncollate?logo=rust)
](https://crates.io/crates/uncollate)
![Crates.io License](https://img.shields.io/crates/l/uncollate)

Uncollate an array of structs into arrays of fields. 

This crate provides a collection of macros to cut down on repetitive complex chains of iterator functions on arrays of structs.
## Examples
The derive macro is helpful when you need to own the fields after uncollating.
It creates an uncollated version of the struct and implements Uncollate for structs that impl `IntoIter` for the corresponding struct.

The vec is consumed by uncollate.
```rs
use uncollate::Uncollate;

#[derive(Uncollate)]
pub struct Dog {
    name: String,
    breed: String
}

fn main() {
    let dog1 = Dog { name: "Bandit", breed: "Beagle" };
    let dog2 = Dog { name: "Scout", breed: "Pug"};
    
    let dogs = vec![dog1, dog2]
    let uncol_dogs: UncollatedDog = dogs.uncollate();
    //&Vec<String>["Bandit", "Scout"]
    uncol_dogs.name();
    
    //&mut Vec<String>["Beagle", "Pug"]
    uncol_dogs.breed();
}
```

Declarative macros are provided when only references to the data are needed, or the original vec must be preserved.
These macros just hide the chain of iterator functions under the hood.
```rs
use uncollate::{uncollate, uncollate_req};

struct Greet {
    greeting: String,
    who: Option<String>,
}

let greetings = vec![
    Greet { greeting: "Hello".into(), who: Some("world!".into()) },
    Greet { greeting: "Howdy".into(), who: None }
];

let opts: Vec<Option<String>> = uncollate!(greetings.who);
// Will throw an error as the vec contains a None who
let whos:Vec<String> = uncollate_req!(greetings.who)?;
```
