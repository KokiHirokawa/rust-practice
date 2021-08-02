use std::ops::Deref;

// - SeeAlso: https://doc.rust-lang.org/std/ops/trait.Deref.html
struct DerefExample<T> {
    value: T,
}

impl<T> Deref for DerefExample<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn main() {
    {
        let hello = String::from("Hello");

        fn takes_str(_: &str) {}
        takes_str(&hello);
    }

    {
        let x = DerefExample { value: 'a' };
        assert_eq!('a', *x);
    }

    {
        let hello = String::from("Hello");
        assert_eq!(*"Hello", *hello);
    }
}