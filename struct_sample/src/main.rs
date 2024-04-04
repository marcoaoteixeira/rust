struct Animal;

struct Dog {
    parent: Animal,
}

impl Animal {
    fn eat(&self) {
        println!("Animal eating...");
    }
}

impl Dog {
    fn bark(&self) {
        println!("Woof!");
    }

    fn eat(&self) {
        println!("Dog eating...");
    }
}

fn main() {
    let dog = Dog { parent: Animal };

    dog.bark();
    dog.eat();
    dog.parent.eat();
}
