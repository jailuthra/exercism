// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

struct Node<T> {
    data: T,
    prev: Option<*mut Node<T>>,
    next: Option<*mut Node<T>>,
}

impl<T> Node<T> {
    pub fn take(&self) -> &T {
        &self.data
    }

    pub fn next(&self) -> Option<&Node<T>> {
        if let Some(node) = self.next {
            unsafe {
                Some(&*node)
            }
        } else {
            None
        }
    }
}

pub struct LinkedList<T> {
    back: Option<*mut Node<T>>,
    front: Option<*mut Node<T>>,
}

pub struct Cursor<'a, T> {
    pos: Option<*mut Node<T>>,
    list: &'a mut LinkedList<T>,
}

pub struct Iter<'a, T>{
    node: Option<&'a Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList{back: None, front: None}
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.front == None || self.back == None
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor { pos: self.front, list: self }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor { pos: self.back, list: self }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        if let Some(node) = self.front {
            unsafe {
                Iter { node: Some(&*node)}
            }
        } else {
            Iter { node: None }
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        if !self.is_empty() {
            let mut pos = self.front;
            while let Some(node) = pos {
                unsafe {
                    pos = (*node).next;
                    Box::from_raw(node);
                }
            }
            self.front = None;
            self.back = None;
        }
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.pos.map(|node| unsafe {
            &mut (*node).data
        })
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        self.pos.map(|node| unsafe {
            self.pos = (*node).next;
        });
        self.peek_mut()
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        self.pos.map(|node| unsafe {
            self.pos = (*node).prev;
        });
        self.peek_mut()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        self.pos.map(|node| unsafe {
            let mut new_pos: Option<*mut Node<T>> = None;
            // Update previous node (or list.front)
            if let Some(prev) = (*node).prev {
                (*prev).next = (*node).next;
                new_pos = Some(prev);
            } else if Some(node) == self.list.front {
                self.list.front = (*node).next;
            }
            // Update next node (or list.back)
            if let Some(next) = (*node).next {
                (*next).prev = (*node).prev;
                new_pos = Some(next);
            } else if Some(node) == self.list.back {
                self.list.back = (*node).prev;
            }
            // Update cursor (or list.front and list.back)
            if new_pos.is_some() {
                self.pos = new_pos;
            } else {
                self.list.front = None;
                self.list.back = None;
            }
            // Return node data
            let b = Box::from_raw(node);
            b.data
        })
    }

    pub fn insert_after(&mut self, element: T) {
        let mut node = Box::into_raw(Box::new(Node{data: element, prev: self.pos, next: None}));
        if let Some(old_pos) = self.pos {
            unsafe {
                (*node).next = (*old_pos).next;
                (*old_pos).next = Some(node);
                if let Some(old_next) = (*node).next {
                    (*old_next).prev = Some(node);
                } else if self.pos == self.list.back {
                    self.list.back = Some(node);
                }
            }
        } else {
            if self.list.back.is_none() {
                self.list.back = Some(node)
            }
            if self.list.front.is_none() {
                self.list.front = self.list.back;
            }
        }
    }

    pub fn insert_before(&mut self, element: T) {
        let mut node = Box::into_raw(Box::new(Node{data: element, prev: None, next: self.pos}));
        if let Some(old_pos) = self.pos {
            unsafe {
                (*node).prev = (*old_pos).prev;
                (*old_pos).prev = Some(node);
                if let Some(old_prev) = (*node).prev {
                    (*old_prev).next = Some(node);
                } else if self.pos == self.list.front {
                    self.list.front = Some(node);
                }
            }
        } else {
            if self.list.front.is_none() {
                self.list.front = Some(node)
            }
            if self.list.back.is_none() {
                self.list.back = self.list.front;
            }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if let Some(node) = self.node {
            self.node = node.next();
            Some(node.take())
        } else {
            None
        }
    }
}
