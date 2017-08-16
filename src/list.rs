use std::mem;

type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Option<Box<Node<T>>>
}

struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>
}

impl<T> List<T> {
    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut cursor: &Option<Box<Node<T>>> = &self.head;
        loop {
            match *cursor {
                Some(ref node) => {
                    len += 1;
                    cursor = &node.next;
                }
                None => {
                    break;
                }
            }
        }
        len
    }

    pub fn new() -> Self {
        List { head: None }
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(ref node) = self.head {
            return Some(&node.item)
        }
        None
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if let Some(ref mut node) = self.head {
            Some(&mut node.item)
        } else {
            None
        }
    }

    pub fn push(&mut self, item: T) {
        self.head = Some(Box::new(Node {
            item,
            next: self.head.take()
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map( | boxed_node | {
            let node = *boxed_node;
            self.head = node.next;
            node.item
        })
    }

    fn back(&mut self) -> &mut Link<T> {
        let mut cursor = &mut self.head;
        loop {
            match { cursor } {
                &mut Some(ref mut node) => cursor = &mut node.next,
                last => return last
            }
        }
    }

    pub fn push_back(&mut self, item: T) {
        let mut cursor = &mut self.head;
        loop {
            match { cursor } {
                &mut Some(ref mut node) => cursor = &mut node.next,
                last => {
                    *last = Some(Box::new(Node { item, next: None }));
                    return;
                }
            }
        }
    }

    pub fn push_back_rec(&mut self, item: T) {
        fn append<T>(node: &mut Option<Box<Node<T>>>, item: T) {
            match *node {
                None => {
                    *node = Some(Box::new(Node { item, next: None }))
                }
                Some(ref mut node) => {
                    append(&mut node.next, item)
                }
            }
        }
        append(&mut self.head, item)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        while let Some(mut node) = cur_link {
            cur_link = mem::replace(&mut node.next, None);
        }
    }
}

pub struct ListIntoIter<T>(List<T>);

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = ListIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIntoIter(self)
    }
}

impl<T> Iterator for ListIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct ListIter<'a, T>
    where T: 'a {
    cursor: &'a Option<Box<Node<T>>>
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = ListIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIter { cursor: &self.head }
    }
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cursor {
            &None => None,
            &Some(ref node) => {
                self.cursor = &node.next;
                Some(&node.item)
            }
        }
    }
}

pub struct ListIterMut<'a, T: 'a> {
   cursor: Option<&'a mut Node<T>>
}

impl <'a, T> IntoIterator for &'a mut List<T> {
    type Item = &'a mut T;
    type IntoIter = ListIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIterMut {
            cursor: match self.head {
                None => None,
                Some(ref mut node) => {
                    Some(node)
                }
            }
        }
    }
}

impl<'a, T> Iterator for ListIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor.take().map(|node| {
            self.cursor = node.next.as_mut().map(|node| &mut **node );
            &mut node.item
        })
    }
}

#[test]
fn test_1() {
    let reps = 1_000_000;
    let mut list = List::new();
    for _ in 0..reps {
        list.push(0);
    }
    for value in &mut list {
        *value += 1;
    }
    let mut total = 0;
    for value in &list {
        total += *value;
    }
    assert!(total == reps);
}

#[test]
fn test_2() {
    let reps = 1_000_000;
    let mut list = List::new();
    for i in 1..reps {
        list.push(i);
        assert!(*list.peek().unwrap() == i);
        {
            let mut n = list.peek_mut().unwrap();
            *n += 1;
        }
        assert!(*list.peek().unwrap() == i + 1);
    }
    for i in (2..reps + 1).rev() {
        let n = list.pop().unwrap();
         assert!(i == n);
    }
}

#[test]
fn test_len() {
    let mut list = List::new();
    assert!(list.len() == 0);
    list.push(0);
    list.push(0);
    list.push(0);
    assert!(list.len() == 3);
    list.push(0);
    assert!(list.len() == 4);
    list.pop();
    assert!(list.len() == 3);
    list.pop();
    list.pop();
    list.pop();
    assert!(list.len() == 0);
}
