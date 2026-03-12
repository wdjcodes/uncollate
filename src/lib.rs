/// This macro allows splitting a vector of structs into a struct of vectors over each field.
/// This is accomplished in a few steps.
///
/// 1. Creates an `UncollatedT` type which has a field corresponding to each field of `T` with type
///    `Vec<F>` and `F` is the type of field in `T`.
/// 2. Impls `Collated` for type `T`. The `Collated` trait is used to add the fields of a single
///    instance of `T` to corresponding vectors in a provided UncollatedT struct instance.
/// 3. The `Uncollate` trait is then able to be used on any type that impls `IntoIter<Item = T>`
///    which provides the `uncollate` method to get an `UncollatedT`
///
///
///
/// # Examples
///
/// ```
/// # use uncollate::Uncollate;
///
/// #[derive(Uncollate)]
/// struct Dog {
///     name: String,
///     breed: String
/// }
///
/// let dogs = vec![
///     Dog {name: "Bandit".to_string(), breed: "Beagle".to_string()},
///     Dog {name: "Scout".to_string(), breed: "Pug".to_string()}
/// ];
///
/// let uncol_dogs = dogs.uncollate();
///
/// assert_eq!(uncol_dogs.name, vec!["Bandit".to_string(), "Scout".to_string()]);
/// assert_eq!(uncol_dogs.breed, vec!["Beagle".to_string(), "Pug".to_string()]);
/// ```
///
/// /// ```
/// # use uncollate::Uncollate;
///
/// #[derive(Uncollate)]
/// struct Dog {
///     name: String,
///     breed: String
/// }
///
/// let dogs = vec![
///     Dog {name: "Bandit".to_string(), breed: "Beagle".to_string()},
///     Dog {name: "Scout".to_string(), breed: "Pug".to_string()}
/// ];
///
/// let mut uncol_dogs = dogs.uncollate();
/// uncol_dogs.name_mut()[1] = "Grumpy".to_string;
///
/// assert_eq!(uncol_dogs.name, vec!["Bandit".to_string(), "Grumpy".to_string()]);
/// assert_eq!(uncol_dogs.breed, vec!["Beagle".to_string(), "Pug".to_string()]);
/// ```
pub use uncollate_macro::Uncollate;

pub trait Uncollate<T> {
    fn uncollate(self) -> T;
}

pub trait Collated<T: Uncollated> {
    fn uncollate_one(self, uncollated: &mut T);
}

pub trait Uncollated: Default {}

impl<U: Uncollated, C: Collated<U>, T: IntoIterator<Item = C>> Uncollate<U> for T {
    fn uncollate(self) -> U {
        let mut uncollated: U = Default::default();
        for collated in self {
            collated.uncollate_one(&mut uncollated);
        }
        uncollated
    }
}

/// uncollates by reference
///
/// ```
/// use uncollate::uncollate;
///
/// struct Greet {
///     greeting: String,
///     who: String,
/// }
///
/// let greetings = vec![
///     Greet { greeting: "Hello".into(), who: "world!".into() },
///     Greet { greeting: "Howdy".into(), who: "partner".into() }
/// ];
///
/// let whos = uncollate!(&greetings.who);
/// assert_eq!(whos, vec![&greetings[0].who, &greetings[1].who])
/// ```
#[macro_export]
macro_rules! uncollate {
    (&$vec:ident.$field:ident) => {
        (&$vec)
            .into_iter()
            .map(|elem| &elem.$field)
            .collect::<::std::vec::Vec<_>>()
    };
    (&$vec:ident.$field:ident, $t:ty) => {
        (&$vec).into_iter().map(|elem| &elem.$field).collect::<$t>()
    };
    (&mut $vec:ident.$field:ident) => {
        (&mut $vec)
            .into_iter()
            .map(|elem| &mut elem.$field)
            .collect::<::std::vec::Vec<_>>()
    };
    (&mut $vec:ident.$field:ident, $t:ty) => {
        (&mut $vec)
            .into_iter()
            .map(|elem| &mut elem.$field)
            .collect::<$t>()
    };
}

/// Uncollate an optional field be reference, and return an error if any is None
///
/// ```
/// use uncollate::uncollate_req;
///
/// struct Greet {
///     greeting: String,
///     who: Option<String>,
/// }
///
/// let greetings = vec![
///     Greet { greeting: "Hello".into(), who: Some("world!".into()) },
///     Greet { greeting: "Howdy".into(), who: None }
/// ];
///
/// let whos = uncollate_req!(&greetings.who);
/// assert_eq!(whos, Err("value must not be None"))
/// ```
#[macro_export]
macro_rules! uncollate_req {
    (&$vec:ident.$field:ident) => {
        (&$vec)
            .into_iter()
            .map(|elem| {
                ::std::option::Option::ok_or(elem.$field.as_ref(), "value must not be None")
            })
            .collect::<::std::result::Result<::std::vec::Vec<_>, _>>()
    };
    (&$vec:ident.$field:ident, $t:ty) => {
        (&$vec)
            .into_iter()
            .map(|elem| {
                ::std::option::Option::ok_or(elem.$field.as_ref(), "value must not be None")
            })
            .collect::<::std::result::Result<$t, _>>()
    };
    (&mut $vec:ident.$field:ident) => {
        (&mut $vec)
            .into_iter()
            .map(|elem| {
                ::std::option::Option::ok_or(elem.$field.as_mut(), "value must not be None")
            })
            .collect::<::std::result::Result<::std::vec::Vec<_>, _>>()
    };
    (&mut $vec:ident.$field:ident, $t:ty) => {
        (&mut $vec)
            .into_iter()
            .map(|elem| {
                ::std::option::Option::ok_or(elem.$field.as_mut(), "value must not be None")
            })
            .collect::<::std::result::Result<$t, _>>()
    };
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn test_uncollate_by_ref() {
        struct Test {
            a: String,
        }
        let test = vec![
            Test { a: "Hello".into() },
            Test {
                a: "Goodbye".into(),
            },
        ];
        let uncol = uncollate!(&test.a);
        assert_eq!(vec![&test[0].a, &test[1].a], uncol);
    }

    #[test]
    pub fn test_uncollate_req() {
        struct Test {
            a: Option<String>,
        }
        let test = vec![
            Test {
                a: Some("Hello".into()),
            },
            Test { a: None },
        ];

        let uncol = uncollate_req!(&test.a);
        assert!(uncol.is_err());
    }
}
