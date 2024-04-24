// https://github.com/TheAlgorithms/Rust/blob/master/src/data_structures/linked_list.rs
// https://cglab.ca/~abeinges/blah/too-many-lists/book/
use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct Node<T> {
    pub val: T,
    pub next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}

pub struct LinkedList<T> {
    pub length: u32,
    pub head: Option<NonNull<Node<T>>>,
    pub tail: Option<NonNull<Node<T>>>,
    marker: PhantomData<Box<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
            marker: PhantomData,
        }
    }

    pub fn insert_at_head(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));

        node.next = self.head;
        let node_ptr = NonNull::new(Box::into_raw(node));
        match self.head {
            None => self.tail = node_ptr,
            Some(head_ptr) => unsafe { (*head_ptr.as_ptr()).prev = node_ptr },
        }
        self.head = node_ptr;
        self.length += 1;
    }

    pub fn insert_at_head2(&mut self, obj: T) {
        let mut node = Node::new(obj);

        node.next = self.head;
        let node_ptr = NonNull::new(&mut node);
        match self.head {
            None => self.tail = node_ptr,
            Some(head_ptr) => unsafe { (*head_ptr.as_ptr()).prev = node_ptr },
        }
        self.head = node_ptr;
        self.length += 1;
    }

    pub fn insert_at_tail(&mut self, obj: T) {}

    pub fn insert_at_ith(&mut self, index: u32, obj: T) {}

    pub fn delete_head(&mut self) -> Option<T> {}

    pub fn delete_tail(&mut self) -> Option<T> {}

    pub fn delete_ith(&mut self, index: u32) -> Option<T> {}

    pub fn get(&self, index: i32) -> Option<&T> {}

    fn get_ith_node(node: Option<NonNull<Node<T>>>, index: i32) -> Option<NonNull<Node<T>>> {}
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // Pop items until there are none left
        while self.delete_head().is_some() {}
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.head {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}
