use std::collections::HashMap;

pub fn select_routine<'a>(event_to_routine: &'a HashMap<&'a str, &'a str>, event_name: &'a str) -> &'a str {
    match event_to_routine.get(event_name) {
        Some(&routine) => routine,
        None => "DefaultRoutine",
    }
}

pub trait Runnable {
    fn run(&self);
}

struct RoutineA;
impl Runnable for RoutineA {
    fn run(&self) {
        println!("Routine A is running!");
    }
}

struct DefaultRoutine;
impl Runnable for DefaultRoutine {
    fn run(&self) {
        println!("DefaultRoutine is running!");
    }
}

pub fn create_routine(routine: &str) -> Box<dyn Runnable> {
    match routine {
        "A" => Box::new(RoutineA),
        _ => Box::new(DefaultRoutine), // Handle unknown routines
    }
}

pub fn create_event_to_routine() -> HashMap<&'static str, &'static str> {
    // Create an empty HashMap
    let mut event_to_routine = HashMap::new();

    // Insert key-value pairs into the HashMap
    event_to_routine.insert("EventA", "RoutineA");

    event_to_routine
}