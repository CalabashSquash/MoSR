use std::{
    error::Error,
    fmt::Debug,
    io::{self, Error as IoError, ErrorKind as IoErrorKind, Write},
    num::ParseIntError,
};

use rand::Rng;

// TODO currently comments say csv should include last viewed but that does not accurately reflect code

const NOTE_STRUCT_FIELDS: u8 = 5;

#[derive(Debug)]
struct Note {
    id: u64,
    name: String,
    current_interval_minutes: u64,
    kind: IntervalType,
    state: NoteState,
    // Also need a last_answered, to use in combination with current time and current_interval_minutes to see if it's ready for review
}

#[derive(Debug)]
enum NoteState {
    Uninitialized,
    Queued,
    Active,
    Suspended,
    Paused,
}

#[derive(Debug)]
struct InvalidNoteState;

impl NoteState {
    // TODO this can probably be part of the From trait thingo?
    fn from(note_state: &str) -> Result<Self, InvalidNoteState> {
        match note_state.to_lowercase().as_str() {
            "uninitialized" => Ok(Self::Uninitialized),
            "queued" => Ok(Self::Queued),
            "active" => Ok(Self::Active),
            "suspended" => Ok(Self::Suspended),
            "paused" => Ok(Self::Paused),
            _ => Err(InvalidNoteState),
        }
    }
}

#[derive(Debug)]
enum IntervalType {
    NoRest,
    MultiDayRest,
    SingleDayRest,
}

impl IntervalType {
    // TODO this can probably be part of the From trait thingo?
    fn from(interval_type: &str) -> Self {
        match interval_type {
            "none" => IntervalType::NoRest,
            "single" => IntervalType::SingleDayRest,
            "multi" => IntervalType::MultiDayRest,
            _ => panic!("Invalid rest type"),
        }
    }
}

#[derive(Debug)]
enum ParseError {
    ParseIntError(ParseIntError),
    IoError(IoError),
    InvalidNoteState,
}

fn not_enough_elements_error() -> Result<Note, ParseError> {
    Err(ParseError::IoError(IoError::new(
        IoErrorKind::InvalidData,
        "Not enough CSV columns",
    )))
}

fn too_many_elements_error() -> Result<Note, ParseError> {
    Err(ParseError::IoError(IoError::new(
        IoErrorKind::InvalidData,
        "Too many CSV columns",
    )))
}

// TODO idea: implement (maybe `From`?) trait for each type in the Note struct in order to convert from
// String.

impl Note {
    fn empty() -> Self {
        Note {
            id: 0,
            name: "".to_string(),
            current_interval_minutes: 0,
            kind: IntervalType::NoRest,
            state: NoteState::Uninitialized,
        }
    }

    // TODO just google how this should be done.
    fn from_csv_line(line: &str) -> Result<Self, ParseError> {
        let mut fields_iter = line.split(',').into_iter();
        let mut note = Note::empty();

        note.id = match fields_iter.next() {
            Some(id) => match id.parse::<u64>() {
                Ok(id_parsed) => id_parsed,
                Err(err) => return Err(ParseError::ParseIntError(err)),
            },
            None => return not_enough_elements_error(),
        };

        note.name = match fields_iter.next() {
            Some(name) => name.to_string(),
            None => return not_enough_elements_error(),
        };

        note.current_interval_minutes = match fields_iter.next() {
            Some(current_interval_minutes) => match current_interval_minutes.parse::<u64>() {
                Ok(interval_parsed) => interval_parsed,
                Err(err) => return Err(ParseError::ParseIntError(err)),
            },
            None => return not_enough_elements_error(),
        };

        note.kind = match fields_iter.next() {
            Some(interval_type) => IntervalType::from(interval_type),
            None => return not_enough_elements_error(),
        };

        note.state = match fields_iter.next() {
            Some(state) => match NoteState::from(state) {
                Ok(state) => match state {
                    NoteState::Uninitialized => return Err(ParseError::InvalidNoteState),
                    any_other_state => any_other_state,
                },
                Err(_) => return Err(ParseError::InvalidNoteState),
            },
            None => return not_enough_elements_error(),
        };

        println!("{:#?}", note);

        match fields_iter.next() {
            None => {}
            _ => {
                return too_many_elements_error();
            }
        };

        // Note this has to be updated each time we add a new field to Note

        Ok(note)
    }

    fn next_interval(&self) -> u64 {
        let interval_float: f64 = self.current_interval_minutes as f64;
        (interval_float * 1.5).ceil() as u64
    }
}


fn main() {
    // println!("Hello, world!");
    // let name = new_exercise_name_cli();
    // println!("Hello, {name}er!");
    // let rest = new_exercise_interval_type_cli();
    // let id = new_exercise_id();

    // let new_exercise = ActiveNote {
    //     id,
    //     name,
    //     current_interval_minutes: 1,
    //     kind: rest
    // };

    // println!("{:#?}", new_exercise)

    let notes = match load_notes("hello_broken.txt") {
        Ok(x) => {
            println!("Done reading");
            x
        }
        Err(ParseError::InvalidNoteState) => {
            println!("bonk");
            return ();
        },
        Err(ParseError::IoError(err)) => {
            println!("{}", err);
            return ();
        }
        Err(ParseError::ParseIntError(err)) => {
            println!("ParseIntError: {} (aka double bonk)", err);
            return ();
        },
    };

    // Next do something with the notes
}

// Loads notes from a file of format:
// id,name,current_interval_minutes,kind,state // ,last_answered
// repeated over multiple lines.
// This is temporary to help me practice, until we start using DBs instead.
fn load_notes(filename: &str) -> Result<Vec<Note>, ParseError> {
    let file = match std::fs::read_to_string(filename) {
        Ok(f) => f,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => panic!("File not found!"),
            _ => panic!("Unknown Error"),
        },
    };

    let mut notes: Vec<Note> = vec![];

    let lines = file.lines();
    for line in lines.into_iter() {
        println!("{}", line);
        let line_result = Note::from_csv_line(line)?;
            // .expect(format!("Parsing line {} in {} failed with error: ", index, filename).as_str());
        println!("{:#?}", line_result);
    }

    // println!("{}", file);

    unimplemented!();
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

fn new_exercise_interval_type_cli() -> IntervalType {
    let mut interval_type = String::new();
    print!(
        "exercise minimum rest required (Acceptable values: none, single, multi) (default: none): "
    );
    io::stdout().flush().expect("Error flushing stdout");

    io::stdin()
        .read_line(&mut interval_type)
        .expect("Failed to read line");
    let type1 = interval_type.trim_end();

    IntervalType::from(type1)
}
