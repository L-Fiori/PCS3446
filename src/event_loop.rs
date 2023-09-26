use crate::event_list::EventList;
use crate::routines::{select_routine, create_routine};
use std::collections::HashMap;

fn event_loop<T> (mut event_list: EventList<T>, event_to_routine: HashMap<&str, &str>) {
    // first it'll be implemented a single iteration of
    // the loop.

    // Extract the first event of the list
    if let Some(event) = event_list.pop() {
        let time = &event.time;
        let name = &event.name;
        // let _metadata = &event.metadata;

        // Generate event log
        // todo: create a better log interface and a real log file
        println!("Event name: {}", name);
        println!("Event time: {}", time);
        // println!("Event metadata: {}", metadata);

        // Select the function which will handle the event
        let routine = select_routine(&event_to_routine, &name);
        let runnable = create_routine(&routine);

        // Execute the function
        runnable.run();
    }
}
