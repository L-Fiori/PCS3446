use std::time::{Duration, Instant};
use std::thread::sleep;
use std::collections::HashMap;
use PCS3446::routines::create_event_to_routine;
use PCS3446::event_list::*;
use PCS3446::event_loop::event_loop;

fn main() {
    // Define the number of timesteps and the time delay in milliseconds
    let num_timesteps = 9;
    let timestep_duration_ms = 100;

    // Initialize the current timestep and a start time
    let mut current_timestep = 0;
    let start_time = Instant::now();

    // Build event_to_routine hashmap
    let event_to_routine = create_event_to_routine();

    // Build event list
    let mut event_list: PCS3446::event_list::EventList<Option<i32>> = EventList::new();

    // Enter the event loop
    while current_timestep < num_timesteps {
        // Perform actions for the current timestep
        let new_timestep = process_current_timestep(&mut event_list, &event_to_routine, current_timestep);

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
    }
}

fn process_current_timestep<T>(event_list: &mut EventList<T>, event_to_routine: &HashMap<&str, &str>, timestep: i32) -> i32 {
    // Your code for processing the current timestep goes here
    println!("Instante de simulacao: {}", timestep);

    if let Some(new_timestep) = event_loop(event_list, event_to_routine, timestep) {
        new_timestep
    } else {
        timestep+1
    }
}

