#[allow(clippy::upper_case_acronyms)]
trait HKT<U> {
    type C; // Current type
    type MU; // Type with C mapped to U
}

trait Functor<U>: HKT<U> {
    fn map<F>(self, f: F) -> Self::MU
    where
        F: FnOnce(Self::C) -> U;
}

impl<T, U> HKT<U> for Option<T> {
    type C = T;
    type MU = Option<U>;
}

fn main() {
    println!("Hello, world!");
}
