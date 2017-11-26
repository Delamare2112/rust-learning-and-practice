#[macro_use] extern crate text_io;

use std::io::{stdout,Write};

mod animals { // by putting things in a mod, we can control its visibility with pub.
    // Because we will use Animal like an interface, it is best to have our own clone trait.
    pub trait AnimalCloner {
        fn box_clone(&self) -> Box<Animal>;
    }
    pub trait Animal : AnimalCloner {
        fn make_sound(&self);
    }
    impl<T> AnimalCloner for T where T: 'static + Animal + Clone {
        fn box_clone(&self) -> Box<Animal> {
            Box::new((*self).clone())
        }
    }
    impl Clone for Box<Animal> {
        fn clone(&self) -> Box<Animal> {
            self.box_clone()
        }
    }

    #[derive(Clone,Copy)]
    pub struct Cat;
    impl Animal for Cat {
        fn make_sound(&self) {
            println!("MEOW!!");
        }
    }

    #[derive(Clone,Copy)]
    pub struct Dog;
    impl Animal for Dog {
        fn make_sound(&self) {
            println!("BARK!");
        }
    }
}

fn main() {
    use animals::*;
    print!("How many cats?\n> ");
    // sometimes the \n isn't flushed.
    let _=stdout().flush();
    let user_input: usize = read!();
    let mut creatures: Vec<Box<Animal>> = vec![Box::new(Cat); user_input];

    creatures.extend(vec![Box::new(Dog) as Box<Animal>; 2]);

    for animal in creatures.iter() {
        animal.make_sound();
    }
    println!("Hello, Trevor!");
}
h__