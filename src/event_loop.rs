use crate::event_list::EventList;
use crate::routines::{select_routine, create_routine};
use std::collections::HashMap;

pub fn event_loop<T> (event_list: &mut EventList<T>, event_to_routine: &HashMap<&str, &str>, timestep: i32) -> bool {
    // first it'll be implemented a single iteration of
    // the loop.

    // Extract the first event of the list
    if let Some(event) = event_list.pop() {
        let time = &event.time;
        let name = &event.name;
        let _metadata = &event.metadata;

        if time >= &timestep {
            event_list.push_back(*event);
            return false;
        }

        // Generate event log
        // todo: create a better log interface and a real log file
        println!("Event name: {}", name);
        println!("Event time: {}", time);
        // println!("Event metadata: {}", metadata);

        // Select the function that will handle the event
        let routine = select_routine(event_to_routine, &name);
        let runnable = create_routine(&routine);

        // Execute the function
        runnable.run();
    }
    true
}
