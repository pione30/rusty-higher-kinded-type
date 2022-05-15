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

trait Monad<U>: Functor<U> {
    fn pure(value: U) -> Self::MU;
    fn bind<F>(self, f: F) -> Self::MU
    where
        F: FnOnce(Self::C) -> Self::MU;
}

impl<T, U> HKT<U> for Option<T> {
    type C = T;
    type MU = Option<U>;
}

impl<T, U> Functor<U> for Option<T> {
    fn map<F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> U,
    {
        self.map(f)
    }
}

fn main() {
    println!("Hello, world!");
}
