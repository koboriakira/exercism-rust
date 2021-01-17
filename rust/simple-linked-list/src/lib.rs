use std::iter::FromIterator;

#[derive(Clone)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}
#[derive(Clone)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        if self.head.is_none() {
            0
        } else {
            let mut count = 0;
            let mut current_node = self.head.as_ref();
            while let Some(node) = current_node {
                count += 1;
                current_node = node.next.as_ref();
            }
            count as usize
        }
    }

    pub fn push(&mut self, _element: T) {
        self.head = Some(Box::new(Node {
            data: _element,
            next: self.head.take(),
        }))
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            None
        } else {
            let head = self.head.take().unwrap();
            self.head = head.next;
            Some(head.data)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.head.is_none() {
            None
        } else {
            Some(&(self.head.as_ref().unwrap().data))
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();

        let mut current_node = self.head;
        while let Some(node_ref) = current_node {
            list.push(node_ref.data);
            current_node = node_ref.next;
        }

        list
    }
}

impl<T> Node<T> {
    pub fn new(_element: T) -> Self {
        Node {
            data: _element,
            next: None,
        }
    }

    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }

    pub fn next(self) -> Option<Box<Node<T>>> {
        self.next
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut result = Self::new();
        for i in _iter {
            result.push(i);
        }
        result
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut result: Vec<T> = Vec::new();
        let mut cur = self;
        while let Some(data) = cur.pop() {
            result.push(data);
        }
        result.reverse();
        result
    }
}
