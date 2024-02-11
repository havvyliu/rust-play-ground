// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.

use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;


type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T> {
    prev: Link<T>,
    next: Link<T>,
    val: Option<T>,
}


impl<T> Node<T> {

    pub fn new_default() -> Self {
        Node {
            prev: None,
            next: None,
            val: None
        }
    }

    pub fn new(val: T) -> Self {
        Node {
            prev: None,
            next: None,
            val: Some(val)
        }
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}

#[derive(Debug)]
pub struct Cursor<'a, T> {
    cur: Link<T>,
    list: &'a mut LinkedList<T>
}


pub struct Iter<'a, T> {
    cur: Link<T>,
    _marker: PhantomData<&'a T>,
}

impl<T> Drop for LinkedList<T> {

    fn drop(&mut self) {
        let mut cursor = self.cursor_back();
        while cursor.cur.is_some() {
            cursor.take();
        };
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        // let head_node = Node::new_default();
        // let tail_node = Node::new_default();
        // let head = Rc::new(RefCell::new(head_node));
        // let tail = Rc::new(RefCell::new(tail_node));
        // head.borrow_mut().next = Some(Rc::clone(&tail));
        // tail.borrow_mut().prev = Some(Rc::clone(&head));
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }


    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<T> {
        Cursor {
            cur: if self.tail.is_some() {self.tail.clone()} else {None},
            list: self
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            cur: if self.head.is_some() {self.head.clone()} else {None},
            list: self
        }
    }

    pub fn new_cursor(&mut self, link: &Link<T>) -> Cursor<'_, T> {
        match link.as_ref() {
            None => {
                Cursor {
                    cur: None,
                    list: self,
                }
            }
            Some(cur) => {
                Cursor {
                    cur: Some(Rc::clone(cur)),
                    list: self,
                }
            }
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            cur: self.tail.clone(),
            _marker: PhantomData,
        }
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {

    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            // match &self.cur {
            //     None => None,
            //     Some(node) => {
            //         // let mut node_clone = Rc::clone(node);
            //         // let mut n = *node_clone.as_ptr();
            //         // Some(&mut n.val)
            //         Some(&mut (*node.as_ptr()).val)
            //     }
            // }
            self.cur.as_mut().map(|node_rc| {
                let mut node_ref = node_rc.as_ptr();
                &mut (*node_ref).val
            }).unwrap().as_mut()
        }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn prev(&mut self) -> Option<&mut T> {
        // If there is no current node, there is nothing to advance to.
        if self.cur.is_none() {
            return None;
        }

        // Safely attempt to move to the next node.
        // `cur` is an `Option<Rc<RefCell<Node<T>>>>`
        let next = self.cur.as_ref().and_then(|node| {
            // Borrow the current node to access its `next` field.
            let borrowed_node = node.borrow();
            borrowed_node.next.clone()
        });

        // Update the cursor's current node to the next node.
        self.cur = next;

        // Attempt to return a mutable reference to the value of the new current node.
        self.cur.as_ref().map(|node| unsafe {
            // We must borrow_mut to mutate the value through RefCell safely.
            &mut (*node.as_ptr()).val
        }).unwrap().as_mut()

    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn next(&mut self) -> Option<&mut T> {
        if self.cur.is_none() {
            return None;
        }
        let prev = self.cur.as_ref().and_then(|node| {
            node.borrow().prev.clone()
        });
        self.cur = prev;
        self.cur.as_ref().map(|node| unsafe {
            &mut (*node.as_ptr()).val
        }).unwrap().as_mut()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        match self.cur.take() {
            None => {
                return None;
            }
            Some(cur) => {
                let prev = cur.borrow().prev.clone();
                let next = cur.borrow().next.clone();
                match next.clone() {
                    None => {
                        if self.list.head.is_some() && Rc::ptr_eq(&cur, self.list.head.as_ref().unwrap()) {
                            self.list.head = None;
                        }
                    }
                    Some(rc) => {
                        rc.borrow_mut().prev = prev.clone();
                        self.cur = Some(rc.clone());
                        if Rc::ptr_eq(&cur, self.list.head.as_ref().unwrap()) {
                            self.list.head = Some(rc);
                        }
                    }
                };
                match prev {
                    None => {
                        if self.list.tail.is_some() && Rc::ptr_eq(&cur, self.list.tail.as_ref().unwrap()) {
                            self.list.tail = None;
                        }
                    }
                    Some(rc) => {
                        rc.borrow_mut().next = next;
                        self.cur = Some(rc.clone());
                        if Rc::ptr_eq(&cur, self.list.tail.as_ref().unwrap()) {
                            self.list.tail = Some(rc);
                        }
                    }
                };

                //Idea from: https://rust-unofficial.github.io/too-many-lists/fourth-breaking.html
                self.list.size -= 1;
                Rc::try_unwrap(cur).ok().unwrap().into_inner().val
            }
        }

    }

    pub fn insert_after(&mut self, _element: T) {
        //Second iteration.
        let new_link  = Rc::new(RefCell::new(Node::new(_element)));
        match self.cur.as_ref() {
            None => {
                self.cur = Some(new_link.clone());
                self.list.head = Some(new_link.clone());
                if self.list.tail.is_none() {self.list.tail = Some(new_link.clone())};
                if self.list.head.is_none() {self.list.head = Some(new_link)};
            }
            Some(old_link) => {
                new_link.borrow_mut().next = Some(old_link.clone());
                if Rc::ptr_eq(&old_link.clone(), self.list.head.as_ref().unwrap()) {
                    self.list.head = Some(new_link.clone());
                };
                if old_link.borrow_mut().prev.is_some() {
                    new_link.borrow_mut().prev = old_link.borrow_mut().prev.clone();
                    old_link.borrow_mut().prev.clone().unwrap().borrow_mut().next = Some(new_link.clone());
                }
                old_link.borrow_mut().prev = Some(new_link.clone());
            }
        }
        self.list.size += 1;
        //First iteration

        // let next = self.cur.as_ref().and_then(|node| {
        //     node.borrow().next.clone()
        // });
        // next.map(|node| {
        //     unsafe {
        //         (*node.as_ptr()).prev = Some(Rc::clone(&new_link));
        //         new_link.borrow_mut().next = Some(Rc::clone(&node));
        //     }
        // });
        // let cur = self.cur.as_mut();
        // cur.map(|rc| {
        //     rc.borrow_mut().next = Some(Rc::clone(&new_link));
        //     new_link.borrow_mut().prev = Some(Rc::clone(&rc))
        // });
        // self.list.size += 1;
        // let res = self.cur.as_ref().take().unwrap().borrow().next.as_ref().unwrap().borrow().val.as_ref().unwrap();
        // let re2 = self.cur.as_ref().take().unwrap().borrow().next.as_ref().unwrap().borrow().val.as_ref().unwrap();
    }

    pub fn insert_before(&mut self, _element: T) {
        let new_link  = Rc::new(RefCell::new(Node::new(_element)));
        match self.cur.as_ref() {
            None => {
                self.cur = Some(new_link.clone());
                self.list.tail = Some(new_link.clone());
                if self.list.tail.is_none() {self.list.tail = Some(new_link.clone())}
                if self.list.head.is_none() {self.list.head = Some(new_link)}
            }
            Some(old_link) => {
                new_link.borrow_mut().prev = Some(old_link.clone());
                if Rc::ptr_eq(old_link, self.list.tail.as_ref().unwrap()) {
                    self.list.tail = Some(new_link.clone());
                }
                if old_link.borrow_mut().next.is_some() {
                    new_link.borrow_mut().next = old_link.borrow_mut().next.clone();
                    old_link.borrow_mut().next.clone().unwrap().borrow_mut().prev = Some(new_link.clone());
                }
                old_link.borrow_mut().next = Some(new_link.clone());
            }
        }
        self.list.size += 1;
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;


    fn next(&mut self) -> Option<&'a T> {
        let to_return = self.peek();
        match self.cur.clone() {
            None => {
                self.cur = None;
            }
            Some(rc) => {
                self.cur = rc.borrow_mut().prev.clone();
            }
        };
        to_return
    }
}

impl<'a, T> Iter<'a, T> {
    pub fn peek(&mut self) -> Option<&'a T> {
        match self.cur.as_ref() {
            None => None,
            Some(rc) => {
                unsafe { (*rc.as_ptr()).val.as_ref() }
            }
        }
    }

}



#[test]
fn is_generic() {
    struct Foo;
    LinkedList::<Foo>::new();
}

// ———————————————————————————————————————————————————————————
// Tests for Step 1: push / pop at front and back
// ———————————————————————————————————————————————————————————

#[test]
#[ignore]
fn basics_empty_list() {
    let list: LinkedList<i32> = LinkedList::new();
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

// push / pop at back ————————————————————————————————————————
#[test]
#[ignore]
fn basics_single_element_back() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_back(5);

    assert_eq!(list.len(), 1);
    assert!(!list.is_empty());

    assert_eq!(list.pop_back(), Some(5));

    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[test]
#[ignore]
fn basics_push_pop_at_back() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for i in 0..10 {
        list.push_back(i);
        assert_eq!(list.len(), i as usize + 1);
        assert!(!list.is_empty());
    }
    for i in (0..10).rev() {
        assert_eq!(list.len(), i as usize + 1);
        assert!(!list.is_empty());
        assert_eq!(i, list.pop_back().unwrap());
    }
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

// push / pop at front ———————————————————————————————————————
#[test]
#[ignore]
fn basics_single_element_front() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_front(5);

    assert_eq!(list.len(), 1);
    assert!(!list.is_empty());

    assert_eq!(list.pop_front(), Some(5));

    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[test]
#[ignore]
fn basics_push_pop_at_front() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for i in 0..10 {
        list.push_front(i);
        assert_eq!(list.len(), i as usize + 1);
        assert!(!list.is_empty());
    }
    for i in (0..10).rev() {
        assert_eq!(list.len(), i as usize + 1);
        assert!(!list.is_empty());
        assert_eq!(i, list.pop_front().unwrap());
    }
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[test]
#[ignore]
fn basics_push_back_pop_front() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for i in 0..10 {
        list.push_back(i);
        assert_eq!(list.len(), i as usize + 1);
        assert!(!list.is_empty());
    }
    for i in 0..10 {
        assert_eq!(list.len(), 10 - i as usize);
        assert!(!list.is_empty());
        assert_eq!(i, list.pop_front().unwrap());
    }
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

// ———————————————————————————————————————————————————————————
// Tests for Step 2: iteration
// ———————————————————————————————————————————————————————————

#[test]
#[ignore]
fn iter() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for num in 0..10 {
        list.push_back(num);
    }

    for (num, &entered_num) in (0..10).zip(list.iter()) {
        assert_eq!(num, entered_num);
    }
}

// ———————————————————————————————————————————————————————————
// Tests for Step 3: full cursor functionality
// ———————————————————————————————————————————————————————————

#[test]
#[ignore]
fn cursor_insert_before_on_empty_list() {
    // insert_after on empty list is already tested via push_back()
    let mut list = LinkedList::new();
    list.cursor_front().insert_before(0);
    assert_eq!(Some(0), list.pop_front());
}

#[test]
#[ignore]
fn cursor_insert_after_in_middle() {
    let mut list = (0..10).collect::<LinkedList<_>>();

    {
        let mut cursor = list.cursor_front();
        let didnt_run_into_end = cursor.seek_forward(4);
        assert!(didnt_run_into_end);

        for n in (0..10).rev() {
            cursor.insert_after(n);
        }
    }

    assert_eq!(list.len(), 20);

    let expected = (0..5).chain(0..10).chain(5..10);

    assert!(expected.eq(list.iter().cloned()));
}

#[test]
#[ignore]
fn cursor_insert_before_in_middle() {
    let mut list = (0..10).collect::<LinkedList<_>>();

    {
        let mut cursor = list.cursor_back();
        let didnt_run_into_end = cursor.seek_backward(4);
        assert!(didnt_run_into_end);

        for n in 0..10 {
            cursor.insert_before(n);
        }
    }

    assert_eq!(list.len(), 20);

    let expected = (0..5).chain(0..10).chain(5..10);

    assert!(expected.eq(list.iter().cloned()));
}

// "iterates" via next() and checks that it visits the right elements
#[test]
#[ignore]
fn cursor_next_and_peek() {
    let mut list = (0..10).collect::<LinkedList<_>>();
    let mut cursor = list.cursor_front();

    assert_eq!(cursor.peek_mut(), Some(&mut 0));

    for n in 1..10 {
        let next = cursor.next().cloned();
        assert_eq!(next, Some(n));
        assert_eq!(next, cursor.peek_mut().cloned());
    }
}

// "iterates" via prev() and checks that it visits the right elements
#[test]
#[ignore]
fn cursor_prev_and_peek() {
    let mut list = (0..10).collect::<LinkedList<_>>();
    let mut cursor = list.cursor_back();

    assert_eq!(cursor.peek_mut(), Some(&mut 9));

    for n in (0..9).rev() {
        let prev = cursor.prev().cloned();
        assert_eq!(prev, Some(n));
        assert_eq!(prev, cursor.peek_mut().cloned());
    }
}

// removes all elements starting from the middle
#[test]
#[ignore]
fn cursor_take() {
    let mut list = (0..10).collect::<LinkedList<_>>();
    let mut cursor = list.cursor_front();
    cursor.seek_forward(5);

    for expected in (5..10).chain((0..5).rev()) {
        assert_eq!(cursor.take(), Some(expected));
    }
}

// ———————————————————————————————————————————————————————————
// Tests for Step 4: clean-up via `Drop`
// ———————————————————————————————————————————————————————————

// The leak tests that are also for this step are separated into
// their own files so that nothing else interferes with the allocator
// whilst they run

// checks number of drops
// may pass for incorrect programs if double frees happen
// exactly as often as destructor leaks
#[test]
#[ignore]
fn drop_no_double_frees() {
    use std::cell::Cell;
    struct DropCounter<'a>(&'a Cell<usize>);

    impl<'a> Drop for DropCounter<'a> {
        fn drop(&mut self) {
            let num = self.0.get();
            self.0.set(num + 1);
        }
    }

    const N: usize = 15;

    let counter = Cell::new(0);
    let list = std::iter::repeat_with(|| DropCounter(&counter))
        .take(N)
        .collect::<LinkedList<_>>();

    assert_eq!(list.len(), N);
    drop(list);
    assert_eq!(counter.get(), N);
}

#[test]
#[ignore]
fn drop_large_list() {
    drop((0..2_000_000).collect::<LinkedList<i32>>());
}

// ———————————————————————————————————————————————————————————
// Tests for Step 5 (advanced): covariance and Send/Sync
// ———————————————————————————————————————————————————————————

// These are compile time tests. They won't compile unless your
// code passes.
// Additional tests for code that must *not* compile are in
// pre_implemented.rs for technical reasons.

#[cfg(feature = "advanced")]
#[test]
#[ignore]
fn advanced_linked_list_is_send_sync() {
    trait AssertSend: Send {}
    trait AssertSync: Sync {}

    impl<T: Send> AssertSend for LinkedList<T> {}
    impl<T: Sync> AssertSync for LinkedList<T> {}
}

#[cfg(feature = "advanced")]
#[allow(dead_code)]
#[test]
#[ignore]
fn advanced_is_covariant() {
    fn a<'a>(x: LinkedList<&'static str>) -> LinkedList<&'a str> {
        x
    }

    fn a_iter<'a>(i: Iter<'static, &'static str>) -> Iter<'a, &'a str> {
        i
    }
}
