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
        // todo: Change none to the start event
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
    let mut event_list: EventList<Option<i32>> = EventList::new();
    event_list.push(999, String::from("Encerramento"), None);
    event_list.push(0, String::from("Partida"), None);

    for item in event_list.iter() {
        println!("{:?}", item);
    }
}    

