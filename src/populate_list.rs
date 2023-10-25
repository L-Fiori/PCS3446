// This file is supposed to contain the different
// test cases for the event list. Thus, it contains
// functions that populate the event list before the
// trigger of the event loop in different cases.
use crate::event_list::*;

pub fn populate_list(test_case: i32) -> EventList{
    match test_case {
        1 => test_1(),
        _ => EventList::new(),
    }
}

pub fn test_1() -> EventList {
    let mut event_list: EventList = EventList::new();
    event_list.push(999, String::from("Encerramento"), Metadata::JobArrival(1, 0));
    event_list.push(240, String::from("Chegada de job"), Metadata::JobArrival(1, 40));
    event_list.push(220, String::from("Chegada de job"), Metadata::JobArrival(1, 80));
    event_list.push(20, String::from("Chegada de job"), Metadata::JobArrival(1, 100));
    event_list.push(20, String::from("Chegada de job"), Metadata::JobArrival(1, 30));

    event_list
}

