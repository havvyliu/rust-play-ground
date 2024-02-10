
// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
use std::{marker::PhantomData, mem, ptr::NonNull};


pub struct Node<T> {
    data: T,
    pub next: Option<NonNull<Node<T>>>,
    pub prev: Option<NonNull<Node<T>>>,
}

pub struct LinkedList<T> {
    pub head: Option<NonNull<Node<T>>>,
    pub tail: Option<NonNull<Node<T>>>,
}

pub struct Cursor<'a, T: 'a> {
    linked_list: &'a mut LinkedList<T>,
    cursor: Option<NonNull<Node<T>>>,
}

pub struct Iter<'a, T> {
    marker: PhantomData<&'a T>,
    cursor: Option<NonNull<Node<T>>>,
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            cursor: self.head,
            linked_list: self,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            cursor: self.tail,
            linked_list: self,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            marker: PhantomData,
            cursor: self.head,
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cursor = self.cursor_front();
        while cursor.take().is_some() {}
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        Some(&mut unsafe { self.cursor?.as_mut() }.data)
    }

    fn next_node(&self) -> Option<NonNull<Node<T>>> {
        unsafe { self.cursor?.as_ref() }.next
    }

    fn prev_node(&self) -> Option<NonNull<Node<T>>> {
        unsafe { self.cursor?.as_ref() }.prev
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        self.cursor = self.next_node();
        self.peek_mut()
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        self.cursor = self.prev_node();
        self.peek_mut()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        let cur_node = self.cursor?;

        unsafe {
            let prev = cur_node.as_ref().prev;
            let next = cur_node.as_ref().next;

            match next {
                Some(mut n) => n.as_mut().prev = prev,
                None => self.linked_list.tail = prev,
            }

            match prev {
                Some(mut p) => p.as_mut().next = next,
                None => self.linked_list.head = next,
            }

            self.cursor = next.or(prev);
            Some(Box::from_raw(cur_node.as_ptr()).data)
        }
    }

    pub fn insert_after(&mut self, _element: T) {
        let new = Box::leak(Box::new(Node {
            data: _element,
            next: self.next_node(),
            prev: self.cursor,
        }));

        let new_node = NonNull::new(new);
        if let Some(mut cursor) = self.cursor {
            unsafe {
                if let Some(mut n) = mem::replace(
                    &mut cursor.as_mut().next,
                    new_node
                ){
                    n.as_mut().prev = new_node;
                } else {
                    self.linked_list.tail = new_node;
                }
            }
        } else {
            self.linked_list.head = new_node;
            self.linked_list.tail = new_node;
            self.cursor = new_node;
        }
    }

    pub fn insert_before(&mut self, _element: T) {
        let new = Box::leak(Box::new(Node {
            data: _element,
            prev: self.prev_node(),
            next: self.cursor,
        }));

        let new_node = NonNull::new(new);
        if let Some(mut cursor) = self.cursor {
            unsafe {
                if let Some(mut p) = mem::replace(
                    &mut cursor.as_mut().prev,
                    new_node,
                ) {
                    p.as_mut().next = new_node;
                } else {
                    self.linked_list.head = new_node;
                }
            }
        } else {
            self.linked_list.head = new_node;
            self.linked_list.tail = new_node;
            self.cursor = new_node;
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let cur_node = unsafe { self.cursor?.as_ref() };
        self.cursor = cur_node.next;

        Some(&cur_node.data)
    }
}
