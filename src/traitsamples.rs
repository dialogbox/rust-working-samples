mod animal {
    pub trait Animal {
        // Static method signature; `Self` refers to the implementor type.
        fn new(name: &'static str) -> Self;

        // Instance method signatures; these will return a string.
        fn name(&self) -> &'static str;
        fn noise(&self) -> &'static str;

        // Traits can provide default method definitions.
        fn talk(&self) {
            println!("{} says {}", self.name(), self.noise());
        }
    }
}

mod sheep {
    use super::animal::*;

    #[derive(Debug, Clone)]
    pub struct Sheep { naked: bool, name: &'static str }

    impl Sheep {
        fn is_naked(&self) -> bool {
            self.naked
        }

        fn shear(&mut self) {
            if self.is_naked() {
                // Implementor methods can use the implementor's trait methods.
                println!("{} is already naked...", self.name());
            } else {
                println!("{} gets a haircut!", self.name);

                self.naked = true;
            }
        }
    }

    // Implement the `Animal` trait for `Sheep`.
    impl Animal for Sheep {
        // `Self` is the implementor type: `Sheep`.
        fn new(name: &'static str) -> Sheep {
            Sheep { name, naked: false }
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn noise(&self) -> &'static str {
            if self.is_naked() {
                "baaaaah?"
            } else {
                "baaaaah!"
            }
        }

        // Default trait methods can be overridden.
        fn talk(&self) {
            // For example, we can add some quiet contemplation.
            println!("{} pauses briefly... {}", self.name, self.noise());
        }
    }
}

mod dog {
    use super::animal::*;

    #[derive(Debug, Clone)]
    pub struct Dog { name: &'static str }

    // Implement the `Animal` trait for `Sheep`.
    impl Animal for Dog {
        // `Self` is the implementor type: `Sheep`.
        fn new(name: &'static str) -> Dog {
            Dog { name  }
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn noise(&self) -> &'static str {
            "Woof!"
        }
    }
}

#[cfg(test)]
mod test {
    mod dogtest {
        #[test]
        fn talk_test() {
            use super::super::dog::*;
            use super::super::animal::*;

            let bella: Dog = Animal::new("Bella");

            bella.talk();
        }
    }

    mod sheeptest {
        #[test]
        fn talk_test() {
            use super::super::sheep::*;
            use super::super::animal::*;

            let molly: Sheep = Animal::new("Molly");

            molly.talk();
        }
    }

    mod bothtest {
        #[test]
        fn talk_test() {
            use super::super::sheep::*;
            use super::super::dog::*;
            use super::super::animal::*;

            let bella: Dog = Animal::new("Bella");
            let molly: Sheep = Animal::new("Molly");

            bella.talk();
            molly.talk();

            // Error because Animal is not an object safe trait
            // https://doc.rust-lang.org/book/second-edition/ch17-02-trait-objects.html#object-safety-is-required-for-trait-objects
            // let animals: Vec<Box<Animal>> = vec![Box::new(bella), Box::new(molly)];
        }
    }
}