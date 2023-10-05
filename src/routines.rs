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

pub fn create_routine(routine: &str) -> Box<dyn Runnable> {
    match routine {
        "JobArrival" => Box::new(JobArrival),
        "JobEntrance" => Box::new(JobEntrance),
        "RequestMemory" => Box::new(RequestMemory),
        "RequestCPU" => Box::new(RequestCPU),
        "EndProcess" => Box::new(EndProcess),
        "FreeCPU" => Box::new(FreeCPU),
        "FreeMemory" => Box::new(FreeMemory),
        "ExitSystem" => Box::new(ExitSystem),
        _ => Box::new(DefaultRoutine), // Handle unknown routines
    }
}

pub fn create_event_to_routine() -> HashMap<&'static str, &'static str> {
    // Create an empty HashMap
    let mut event_to_routine = HashMap::new();

    // Insert key-value pairs into the HashMap
    event_to_routine.insert("Chegada de job", "JobArrival");
    event_to_routine.insert("Ingresso de job", "JobEntrance");
    event_to_routine.insert("Requisicao de memoria de job", "RequestMemory");
    event_to_routine.insert("Requisicao de processador de job", "RequestCPU");
    event_to_routine.insert("Fim de processamento de job", "EndProcess");
    event_to_routine.insert("Liberacao de processador job", "FreeCPU");
    event_to_routine.insert("Liberacao de memoria job", "FreeMemory");
    event_to_routine.insert("Saida do sistema job", "ExitSystem");
    //event_to_routine.insert("", "");
    
    event_to_routine
}

//==================== ACTUAL ROUTINE IMPLS ====================

struct DefaultRoutine;
impl Runnable for DefaultRoutine {
    fn run(&self) {
        println!("DefaultRoutine is running!");
    }
}

struct JobArrival;
impl Runnable for JobArrival {
    fn run(&self) {
        println!("JobArrival is running!");
    }
}

struct JobEntrance;
impl Runnable for JobEntrance {
    fn run(&self) {
        println!("JobEntrance is running!");
    }
}

struct RequestMemory;
impl Runnable for RequestMemory {
    fn run(&self) {
        println!("RequestMemory is running!");
    }
}

struct RequestCPU;
impl Runnable for RequestCPU {
    fn run(&self) {
        println!("RequestCPU is running!");
    }
}

struct EndProcess;
impl Runnable for EndProcess {
    fn run(&self) {
        println!("EndProcess is running!");
    }
}

struct FreeCPU;
impl Runnable for FreeCPU {
    fn run(&self) {
        println!("FreeCPU is running!");
    }
}

struct FreeMemory;
impl Runnable for FreeMemory {
    fn run(&self) {
        println!("FreeMemory is running!");
    }
}

struct ExitSystem;
impl Runnable for ExitSystem {
    fn run(&self) {
        println!("ExitSystem is running!");
    }
}

