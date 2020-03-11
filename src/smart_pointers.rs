pub mod deref {

    // Let's re-implement Box in a similar way, to see how the Deref trait works. Note: One big
    // difference between MyBox<T> and the actual Box<T> is that MyBox doesn't store data on the
    // heap.

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    use std::ops::Deref;

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    pub fn main() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);

        assert_eq!(5, *y);

        // Without the Deref trait, the compiler can only dereference & references.

        // *y is this behind the scenes: *(y.deref())

        println!("deref test passed!");
    }

    pub mod implicit_coercion {
        use super::*;

        fn hello(name: &str) {
            println!("Hello, {}!", name);
        }

        pub fn main() {
            let s = MyBox::new(String::from("test_1!"));

            // Rust can turn &MyBox<String> into &String by calling deref. The standard library
            // provides an implementation of Deref on String that returns a string slice.
            hello(&s);

            // If Rust didn’t implement deref coercion, we would have to write:
            let s = MyBox::new(String::from("test_2!"));
            hello(&(*s)[..]);
        }
    }
}
