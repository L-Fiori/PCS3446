#[derive(Debug)]
pub struct Event<T> {
    time: i32,
    name: String,
    metadata: T,
    next: Option<Box<Event<T>>>,
}

pub struct EventList<T> {
    head: Option<Box<Event<T>>>,
}

impl<T> EventList<T> {
    pub fn new() -> Self {
        EventList { head: None }
    }

    // Add a new event to the event list
    pub fn push(&mut self, time: i32, name: String, metadata: T) {
        let new_event = Box::new(Event {
            time,
            name,
            metadata,
            next: self.head.take(),
        });
        self.head = Some(new_event);
    }

    // Get an iterator over the event list
    pub fn iter(&self) -> EventListIter<T> {
        EventListIter {
            current: self.head.as_ref().map(|event| &**event),
        }
    }

    // Pops the event list
    pub fn pop(&mut self) -> Option<Box<Event<T>>> {
        self.head.take().map(|mut old_head| {
            self.head = old_head.next.take();
            old_head
            })
    }
}

// Define an iterator for the event list
pub struct EventListIter<'a, T> {
    current: Option<&'a Event<T>>,
}

impl<'a, T> Iterator for EventListIter<'a, T> {
    type Item = &'a Event<T>; 

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|event| {
            self.current = event.next.as_ref().map(|event| &**event);
            event
        })
    }
}

fn main() {
}

fn event_loop(event_list: EventList<T>) {
    // first it'll be implemented a single iteration of
    // the loop.

    // Extract the first event of the list
    let event = event_list.pop();

    // Unpack the event's elements
    let time = event.unwrap().time;
    let name = event.unwrap().name;
    let metadata = event.unwrap().metadata;

    // Generate event log
    // todo: create a better log interface and a real log file
    println!("Event name: {}", name);
    println!("Event time: {}", time);
    println!("Event metadata: {}", metadata);

    // Select the function which will handle the event
    let routine = select_routine();

    // Execute the function
    exec_routine(routine);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_empty_list() {
        // Create an empty event list
        let mut event_list: EventList<Option<i32>> = EventList::new();

        // Push an event
        event_list.push(999, String::from("Encerramento"), None);

        // Assert the event list has the correct length
        assert_eq!(event_list.iter().count(), 1);
    }

    #[test]
    fn test_push_multiple_events() {
        // Create an empty event list
        let mut event_list: EventList<Option<i32>> = EventList::new();

        // Push multiple events
        event_list.push(999, String::from("Encerramento"), None);
        event_list.push(0, String::from("Partida"), None);

        // Assert the event list has the correct length
        assert_eq!(event_list.iter().count(), 2);
    }

    #[test]
    fn test_iter_empty_list() {
        // Create an empty event list
        let event_list: EventList<Option<i32>> = EventList::new();

        // Iterate over the list and collect items
        let events: Vec<_> = event_list.iter().collect();

        // Assert that there are no events in the collected vector
        assert_eq!(events.len(), 0);
    }

    #[test]
    fn test_iter_multiple_events() {
        // Create an event list with events
        let mut event_list: EventList<Option<i32>> = EventList::new();
        event_list.push(999, String::from("Encerramento"), None);
        event_list.push(0, String::from("Partida"), None);

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
        let mut event_list: EventList<Option<i32>> = EventList::new();

        // Try to pop an event from the list
        let popped_event = event_list.pop();

        // Assert that the result is None
        assert!(popped_event.is_none());
    }

    #[test]
    fn test_pop_multiple_events() {
        // Create an event list with events
        let mut event_list: EventList<Option<i32>> = EventList::new();
        event_list.push(999, String::from("Encerramento"), None);
        event_list.push(0, String::from("Partida"), None);

        // Pop events from the list
        let popped_event1 = event_list.pop();
        let popped_event2 = event_list.pop();

        // Assert that the popped events have the correct names
        assert_eq!(popped_event1.unwrap().name, "Partida");
        assert_eq!(popped_event2.unwrap().name, "Encerramento");
    }
}
