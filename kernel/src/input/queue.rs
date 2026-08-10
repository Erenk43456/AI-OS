use spin::Mutex;

use super::event::InputEvent;

const QUEUE_SIZE: usize = 128;

struct InputQueue {
    buffer: [Option<InputEvent>; QUEUE_SIZE],
    read: usize,
    write: usize,
}

impl InputQueue {
    const fn new() -> Self {
        Self {
            buffer: [None; QUEUE_SIZE],
            read: 0,
            write: 0,
        }
    }

    fn push(&mut self, event: InputEvent) {
        let next = (self.write + 1) % QUEUE_SIZE;

        if next == self.read {
            return;
        }

        self.buffer[self.write] = Some(event);
        self.write = next;
    }

    fn pop(&mut self) -> Option<InputEvent> {
        if self.read == self.write {
            return None;
        }

        let event = self.buffer[self.read].take();

        self.read = (self.read + 1) % QUEUE_SIZE;

        event
    }
}

static QUEUE: Mutex<InputQueue> = Mutex::new(InputQueue::new());

pub fn push(event: InputEvent) {
    QUEUE.lock().push(event);
}

pub fn pop() -> Option<InputEvent> {
    QUEUE.lock().pop()
}
