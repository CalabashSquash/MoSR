use std::io::{self, Write};

use rand::Rng;

#[derive(Debug)]
struct Exercise {
    id: u64,
    name: String,
    current_interval_minutes: u64,
    kind: ExerciseRest,
}

#[derive(Debug)]
enum ExerciseRest {
    NoRest,
    MultiDayRest,
    SingleDayRest,
}

impl Exercise {
    fn next_interval(&self) -> u64 {
        let interval_float: f64 = self.current_interval_minutes as f64;
        (interval_float * 1.5).ceil() as u64
    }
}

fn main() {
    println!("Hello, world!");
    let name = new_exercise_name_cli();
    println!("Hello, {name}er!");
    let rest = new_exercise_rest_cli();
    println!("Hello, {name}er!");
    let id = new_exercise_id();
    println!("ID: {id}");

    let new_exercise = Exercise {
        id,
        name,
        current_interval_minutes: 1,
        kind: rest
    };

    println!("{:#?}", new_exercise)
}

fn new_exercise_id() -> u64 {
    rand::thread_rng().gen()
}

fn new_exercise_name_cli() -> String {
    let mut name = String::new();
    print!("New exercise name: ");
    io::stdout().flush().expect("Error flushing stdout");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name1 = name.trim_end();

    String::from(name1)
}

fn new_exercise_rest_cli() -> ExerciseRest {
    let mut name = String::new();
    // io::stdin().read_line(name).inspect(| x | println!({x})).unwrap()
    print!("exercise minimum rest required (Acceptable values: none, single, multi) (default: none): ");
    io::stdout().flush().expect("Error flushing stdout");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    // let (name1, _) = name.split_at(name.len() - 1);
    let name1 = name.trim_end();

    match name1 {
        "none" => ExerciseRest::NoRest,
        "single" => ExerciseRest::SingleDayRest,
        "multi" => ExerciseRest::MultiDayRest,
        _ => panic!("Invalid rest type")
    }
}