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
    event_list.push(999, String::from("Encerramento"), Metadata::JobArrival(0, 0, 0));
    event_list.push(240, String::from("Chegada de job"), Metadata::JobArrival(4, 40, 40));
    event_list.push(220, String::from("Chegada de job"), Metadata::JobArrival(3, 80, 80));
    event_list.push(20, String::from("Chegada de job"), Metadata::JobArrival(2, 100, 120));
    event_list.push(20, String::from("Chegada de job"), Metadata::JobArrival(1, 30, 60));

    event_list
}

