use std::mem;

pub struct List<'a, T> {
    head: Link<T>,
    tail: Option<&'a mut Node<T>>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<'a, T> List<'a ,T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None
        }
    }

    pub fn push(&'a mut self, elem: T) {
        let new_tail = Box::new(Node {
            elem: elem,
            next: None,
        });

        // Put Box at the right place and grab reference to its node
        let new_tail = match self.tail.take() {
            // Non empty list
            Some(old_tail) => {
                old_tail.next = Some(new_tail);
                old_tail.next.as_deref_mut()
            }
            // Empty list
            None => {
                self.head = Some(new_tail);
                self.head.as_deref_mut()
            }
        };
        self.tail = new_tail;
    }

    pub fn pop(&'a mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            // Check if there is no more head
            if self.head.is_none() {
                self.tail = None;
            }
            head.elem
        })
    }
}