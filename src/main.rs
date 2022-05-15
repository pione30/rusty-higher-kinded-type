#[allow(clippy::upper_case_acronyms)]
trait HKT<U> {
    type C; // Current type
    type MU; // Type with C mapped to U
}

impl<T, U> HKT<U> for Option<T> {
    type C = T;
    type MU = Option<U>;
}

fn main() {
    println!("Hello, world!");
}
