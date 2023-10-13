use crate::routines::{select_routine, create_routine};
use crate::system_abstractions::ControlModule;
use std::collections::HashMap;

pub fn event_loop (event_to_routine: &HashMap<&str, &str>, timestep: i32, control_module: &ControlModule) -> Option<i32> {

    let mut continue_processing = true;
    let shared_state = &control_module.shared_state;
    let raw_event_list = shared_state.get_event_list();
    let mut event_list = raw_event_list.lock().unwrap();

    while continue_processing {
        // Extract the first event of the list
        if let Some(event) = event_list.pop() {
            let time = event.time;
            let name = &event.name;
            let metadata = &event.metadata;

            if time > timestep {
                event_list.push_back(*event);
                return Some(time);
            }

            // Generate event log
            // todo: create a better log interface and a real log file
            println!("Event name: {}", name);
            println!("Event time: {}", time);
            println!("Event metadata: {:?}", metadata);

            // Select the function that will handle the event
            let routine = select_routine(event_to_routine, &name);
            let runnable = create_routine(&routine, metadata);

            // Execute the function
            runnable.run(control_module);
        } else {
            continue_processing = false;
        }
    }
    None
}
