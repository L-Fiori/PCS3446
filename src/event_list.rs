use crate::system_abstractions::{Job};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Metadata {
    JobArrival(i32, i32, i32),
    JobEntrance(Job),
    RequestMemory(Job),
    RequestCPU(Job),
    EndProcess(Job),
    FreeCPU(Job),
    FreeMemory(Job),
    ExitSystem(Job),
    PauseJob(Job),
    DefaultRoutine,
}

pub struct Event {
    pub time: i32,
    pub name: String,
    pub metadata: Metadata,
    pub next: Option<Box<Event>>,
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Event {{ time: {}, name: \"{}\", metadata: {:?} }}",
            self.time, self.name, self.metadata
        )
    }
}

pub struct EventList {
    head: Option<Box<Event>>,
}

impl fmt::Debug for EventList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EventList: [\n")?;
        if let Some(head) = &self.head {
            write!(f, "{:?}", head)?;
            let mut current = &head.next;
            while let Some(event) = current {
                write!(f, ",\n{:?}", event)?;
                current = &event.next;
            }
        }
        write!(f, "\n]")
    }
}

impl EventList {
    pub fn new() -> Self {
        EventList { head: None }
    }

    pub fn push(&mut self, time: i32, name: String, metadata: Metadata) {
        let mut new_event = Box::new(Event {
            time,
            name,
            metadata,
            next: None, // Initialize next as None for the new event
        });

        // Check if the list is empty or if the new event should be inserted at the beginning
        if self.head.is_none() || time < self.head.as_ref().unwrap().time {
            new_event.next = self.head.take(); // Set new_event.next to the current head
            self.head = Some(new_event); // Update the head
        } else {
            let mut current = &mut self.head;
            
            // Find the appropriate position to insert the new event
            while let Some(event) = current {
                if event.next.is_none() || time < event.next.as_ref().unwrap().time {
                    new_event.next = event.next.take(); // Set new_event.next to the following event
                    event.next = Some(new_event); // Update the current event's next to the new event
                    break;
                }
                current = &mut event.next;
            }
        }
    }

    // Get an iterator over the event list
    pub fn iter(&self) -> EventListIter {
        EventListIter {
            current: self.head.as_ref().map(|event| &**event),
        }
    }

    // Pops the event list
    pub fn pop(&mut self) -> Option<Box<Event>> {
        self.head.take().map(|mut old_head| {
            self.head = old_head.next.take();
            old_head
            })
    }

    // Push an event back into the event list
    pub fn push_back(&mut self, event: Event) {
        let mut new_event = Box::new(event);
        new_event.next = self.head.take();
        self.head = Some(new_event);
    }
}

// Define an iterator for the event list
pub struct EventListIter<'a> {
    current: Option<&'a Event>,
}

impl<'a> Iterator for EventListIter<'a> {
    type Item = &'a Event; 

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|event| {
            self.current = event.next.as_ref().map(|event| &**event);
            event
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_empty_list() {
        // Create an empty event list
        let mut event_list: EventList = EventList::new();

        // Push an event
        event_list.push(999, String::from("Encerramento"), Metadata::JobArrival(1));

        // Assert the event list has the correct length
        assert_eq!(event_list.iter().count(), 1);
    }

    #[test]
    fn test_push_multiple_events() {
        // Create an empty event list
        let mut event_list: EventList = EventList::new();

        // Push multiple events
        event_list.push(999, String::from("Encerramento"), Metadata::JobArrival(1));
        event_list.push(0, String::from("Partida"), Metadata::JobArrival(1));

        // Assert the event list has the correct length
        assert_eq!(event_list.iter().count(), 2);
    }

    #[test]
    fn test_iter_empty_list() {
        // Create an empty event list
        let event_list: EventList = EventList::new();

        // Iterate over the list and collect items
        let events: Vec<_> = event_list.iter().collect();

        // Assert that there are no events in the collected vector
        assert_eq!(events.len(), 0);
    }

    #[test]
    fn test_iter_multiple_events() {
        // Create an event list with events
        let mut event_list: EventList = EventList::new();
        event_list.push(999, String::from("Encerramento"), Metadata::JobArrival(1));
        event_list.push(0, String::from("Partida"), Metadata::JobArrival(1));

        // Iterate over the list and collect items
        let events: Vec<_> = event_list.iter().collect();

        // Assert that the collected events match the expected names
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].name, "Partida");
        assert_eq!(events[1].name, "Encerramento");
    }

    #[test]
    fn test_pop_empty_list() {
        // Create an empty event list
        let mut event_list: EventList = EventList::new();

        // Try to pop an event from the list
        let popped_event = event_list.pop();

        // Assert that the result is None
        assert!(popped_event.is_none());
    }

    #[test]
    fn test_pop_multiple_events() {
        // Create an event list with events
        let mut event_list: EventList = EventList::new();

        event_list.push(999, String::from("Encerramento"), Metadata::JobArrival(1));
        event_list.push(0, String::from("Partida"), Metadata::JobArrival(1));
        // Pop events from the list
        let popped_event1 = event_list.pop();
        let popped_event2 = event_list.pop();

        // Assert that the popped events have the correct names
        assert_eq!(popped_event1.unwrap().name, "Partida");
        assert_eq!(popped_event2.unwrap().name, "Encerramento");
    }
}
