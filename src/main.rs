use std::time::{Duration, Instant};
use std::thread::sleep;
use std::collections::HashMap;
use PCS3446::routines::create_event_to_routine;
use PCS3446::event_loop::event_loop;
use PCS3446::populate_list::populate_list;
use PCS3446::system_abstractions::{Memory, ControlModule, SharedState, SystemEntryQueue, ExecQueue, MemoryAllocQueue, CPUAllocQueue};

fn main() {
    // Define the number of timesteps and the time delay in milliseconds
    let num_timesteps = 1000;
    let timestep_duration_ms = 1000;

    // Initialize the current timestep and a start time
    let mut current_timestep = 0;
    let start_time = Instant::now();

    // Build event_to_routine hashmap
    let event_to_routine = create_event_to_routine();

    // Build event list
    // let mut event_list: PCS3446::event_list::EventList<Option<i32>> = EventList::new();

    // Populate event list
    let event_list = populate_list(1);

    // Create the control module and its requirements
    let system_entry_queue = SystemEntryQueue::new();
    let memory_alloc_queue = MemoryAllocQueue::new();
    let cpu_alloc_queue = CPUAllocQueue::new();
    let exec_queue = ExecQueue::new();
    let memory = Memory::new(128);

    let shared_state = SharedState::new(event_list, system_entry_queue, memory_alloc_queue, cpu_alloc_queue, exec_queue, memory, current_timestep);

    let mut control_module = ControlModule::new(shared_state);

    // Enter the event loop
    while current_timestep < num_timesteps {
        // Perform actions for the current timestep
        let new_timestep = process_current_timestep(&event_to_routine, current_timestep, &control_module);

        // Calculate the elapsed time since the start
        let elapsed_time = start_time.elapsed();

        // Calculate the desired duration for the current timestep
        let desired_duration = Duration::from_millis(timestep_duration_ms as u64);

        // Calculate the remaining time to sleep to maintain the desired timestep duration
        let remaining_time = if elapsed_time < desired_duration {
            desired_duration - elapsed_time
        } else {
            Duration::from_millis(0)
        };

        // Sleep to maintain the desired timestep duration
        sleep(remaining_time);

        // Increment the current timestep
        current_timestep = new_timestep;
        control_module.update_current_timestep(current_timestep);
    }
}

fn process_current_timestep(event_to_routine: &HashMap<&str, &str>, timestep: i32, control_module: &ControlModule) -> i32 {
    // Your code for processing the current timestep goes here
    println!("Instante de simulacao: {}", timestep);
    // println!("Event list: {:?}", event_list);
    //println!("Hashmap: {:?}", event_to_routine);

    if let Some(new_timestep) = event_loop(event_to_routine, timestep, control_module) {
        new_timestep
    } else {
        timestep+1
    }
}

