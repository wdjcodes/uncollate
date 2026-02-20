# uncollate
Uncollate an array of structs into arrays of fields.

## Example
```rs
#[deriver(Uncollate)]
pub struct Dog {
    name: String,
    breed: String
}

let dog1 = Dog { name: "Bandit", breed: "Beagle" };
let dog2 = Dog { name: "Scout", breed: "Pug"};

let dogs = vec![dog1, dog2]

//Vec<String> ["Bandit", "Scout"]
dogs.uncoll_name();

//Vec<String> ["Beagle", "Pug"]
dogs.uncoll_breed();
```
