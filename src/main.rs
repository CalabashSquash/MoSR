use std::{fmt::{self, Debug}, io::{self, Write}};

use rand::Rng;

enum Note {
    Queued(QueuedNote),
    Active(ActiveNote),
    Suspended(SuspendedNote),
    Paused(PausedNote),
}

// implement Into/From for switching between notes

struct QueuedNote {}

#[derive(Debug)]
struct ActiveNote {
    id: u64,
    name: String,
    current_interval_minutes: u64,
    kind: IntervalType,
}

struct SuspendedNote {}

struct PausedNote {}

#[derive(Debug)]
enum IntervalType {
    NoRest,
    MultiDayRest,
    SingleDayRest,
}

impl ActiveNote {
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
    let id = new_exercise_id();

    let new_exercise = ActiveNote {
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

fn new_exercise_rest_cli() -> IntervalType {
    let mut name = String::new();
    // io::stdin().read_line(name).inspect(| x | println!({x})).unwrap()
    print!(
        "exercise minimum rest required (Acceptable values: none, single, multi) (default: none): "
    );
    io::stdout().flush().expect("Error flushing stdout");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    // let (name1, _) = name.split_at(name.len() - 1);
    let name1 = name.trim_end();

    match name1 {
        "none" => IntervalType::NoRest,
        "single" => IntervalType::SingleDayRest,
        "multi" => IntervalType::MultiDayRest,
        _ => panic!("Invalid rest type"),
    }
}
