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