// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html

pub mod methods_same_names {

    pub mod colliding_methods {
        trait Pilot {
            fn fly(&self);
        }

        trait Wizard {
            fn fly(&self);
        }

        struct Human;

        impl Pilot for Human {
            fn fly(&self) {
                println!("This is your captain speaking.");
            }
        }

        impl Wizard for Human {
            fn fly(&self) {
                println!("Up!");
            }
        }

        impl Human {
            fn fly(&self) {
                println!("*waving arms furiously*");
            }
        }

        pub fn main() {
            let person = Human;
            person.fly(); // by default, uses the method on the type

            let person = Human;
            Pilot::fly(&person);
            Wizard::fly(&person);
            person.fly();
        }
    }

    pub mod associated_functions {

        // This is a type with an "associated function", because it doesn't take self:
        trait Animal {
            fn baby_name() -> String;
        }

        struct Dog;

        impl Dog {
            fn baby_name() -> String {
                String::from("Spot")
            }
        }

        impl Animal for Dog {
            fn baby_name() -> String {
                String::from("puppy")
            }
        }

        pub fn main() {
            println!("A baby dog is called a {}", Dog::baby_name()); // "Spot"

            // instead:
            println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // "puppy"


            // In general, fully qualified syntax is defined as follows:
            // <Type as Trait>::function(receiver_if_method, next_arg, ...);

            // For associated functions, there would not be a receiver: there would only be the list
            // of other arguments.
        }
    }
}

pub mod super_traits {

    use std::fmt;

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
    struct Point {
        x: i32,
        y: i32,
    }

    impl OutlinePrint for Point {}

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    pub fn main() {
        let p = Point { x: 2, y: 3 };
        p.outline_print();
        // println!("point: {}", p);
    }
}

pub mod newtype_pattern {
    // useful when implementing a trait on a type that isn't defined in our local crate.
    // ie: this is a workaround for the "orphan rule"

    use std::fmt;

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    pub fn main() {
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
    }
}
