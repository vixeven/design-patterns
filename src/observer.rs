use std::collections::HashMap;

use crate::Expense;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Event {
    Add,
    Update,
}

pub type Subscriber = fn(expense: Expense);

trait Observer {
    fn update(&self, expense: Expense);
}

#[derive(Default, Clone, Debug)]
pub struct Publisher {
    events: HashMap<Event, Vec<Subscriber>>,
}

impl Publisher {
    pub fn subscribe(&mut self, event_type: Event, listener: Subscriber) {
        self.events.entry(event_type.clone()).or_default();
        self.events.get_mut(&event_type).unwrap().push(listener);
    }

    pub fn unsubscribe(&mut self, event_type: Event, listener: Subscriber) {
        self.events
            .get_mut(&event_type)
            .unwrap()
            .retain(|&x| x != listener);
    }

    pub fn notify(&self, event_type: Event, expense: Expense) {
        if !self.events.contains_key(&event_type) {
            return;
        }

        let listeners = self.events.get(&event_type).unwrap();

        for listener in listeners {
            listener(expense.clone());
        }
    }
}
