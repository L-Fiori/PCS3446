// This file is supposed to contain the different
// test cases for the event list. Thus, it contains
// functions that populate the event list before the
// trigger of the event loop in different cases.
use crate::event_list::*;

pub fn populate_list(test_case: i32) -> EventList<Option<i32>>{
    match test_case {
        1 => test_1(),
        _ => EventList::new(),
    }
}

pub fn test_1() -> EventList<Option<i32>> {
    let mut event_list: EventList<Option<i32>> = EventList::new();
    event_list.push(999, String::from("Encerramento"), None);
    event_list.push(240, String::from("Chegada de job"), None);
    event_list.push(220, String::from("Chegada de job"), None);
    event_list.push(20, String::from("Chegada de job"), None);
    event_list.push(20, String::from("Chegada de job"), None);

    event_list
}

