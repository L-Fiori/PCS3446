struct Event<T> {
    value: T,
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
    pub fn push(&mut self, value: T) {
        let new_event = Box::new(Event {
            value,
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
}

// Define an iterator for the event list
pub struct EventListIter<'a, T> {
    current: Option<&'a Event<T>>,
}

impl<'a, T> Iterator for EventListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|event| {
            self.current = event.next.as_ref().map(|event| &**event);
            &event.value
        })
    }
}

fn main() {
    let mut list = EventList::new();
    list.push((1, "one"));
    list.push((2, "two"));
    list.push((3, "three"));

    for item in list.iter() {
        println!("{:?}", item);
    }
}    

