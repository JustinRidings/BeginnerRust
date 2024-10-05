use rand::Rng;
use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

impl Person {
    fn new(name: &str, age: u8) -> Person {
        Person { name: name.to_string(), age}
    }

    fn greet(&self) {
        println!(
            "Hello, my name is {} and I am {} years old.",
            self.name, self.age
        );
    }
}

fn generate_random_name() -> String {
    let mut rng = rand::thread_rng();
    let name_length = rng.gen_range(6..=10);
    let letters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    (0..name_length)
        .map(|_| {
            let idx = rng.gen_range(0..letters.len());
            letters[idx]
        })
        .collect()
}

fn generate_random_age() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=100)
}

fn generate_random_person() -> Person {
    let name = generate_random_name();
    let age = generate_random_age();

    Person::new(&name, age)
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, Age: {}", self.name, self.age)
    }
}

fn main() {
    let random_person = generate_random_person();
    println!("{}", random_person); // Debug printing
    random_person.greet();
}
