#[derive(Debug)]
pub struct Event {
    time: i32,
    name: String,
    next: Option<Box<Event>>,
}

pub struct EventList {
    head: Option<Box<Event>>,
}

impl EventList {
    pub fn new() -> Self {
        // todo: Change none to the start event
        EventList { head: None }
    }

    // Add a new event to the event list
    pub fn push(&mut self, time: i32, name: String) {
        let new_event = Box::new(Event {
            time,
            name,
            next: self.head.take(),
        });
        self.head = Some(new_event);
    }

    // Get an iterator over the event list
    pub fn iter(&self) -> EventListIter {
        EventListIter {
            current: self.head.as_ref().map(|event| &**event),
        }
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

fn main() {
    let mut list = EventList::new();
    list.push(0, String::from("Partida"));
    list.push(999, String::from("Encerramento"));

    for item in list.iter() {
        println!("{:?}", item);
    }
}    

