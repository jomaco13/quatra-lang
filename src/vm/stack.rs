use crate::qud::Qud;
use std::collections::VecDeque;

pub struct QudStack {
    items: VecDeque<Qud>,
}

impl QudStack {
    pub fn with_capacity(cap: usize) -> Self {
        QudStack {
            items: VecDeque::with_capacity(cap),
        }
    }

    pub fn push(&mut self, value: Qud) {
        self.items.push_back(value);
    }

    pub fn pop(&mut self) -> Qud {
        self.items.pop_back().unwrap_or(Qud::Zero)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}
