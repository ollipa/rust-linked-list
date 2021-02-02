#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug, Default)]
pub struct LinkedList<T> {
    head: Option<Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, value: T) {
        if let Some(old_head) = self.head.take() {
            self.head = Some(Node {
                value,
                next: Some(Box::new(old_head)),
            });
        } else {
            self.head = Some(Node { value, next: None });
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.head = old_head.next.map(|boxed| *boxed);
            old_head.value
        })
    }

    pub fn clear(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take().map(|boxed| *boxed);
        }
    }

    pub fn iter(&self) -> LinkedListIter<'_, T> {
        LinkedListIter::new(self.head.as_ref())
    }

    pub fn iter_mut(&mut self) -> LinkedListIterMut<'_, T> {
        LinkedListIterMut::new(self.head.as_mut())
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        self.clear()
    }
}

pub struct LinkedListIter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> LinkedListIter<'a, T> {
    fn new(current: Option<&'a Node<T>>) -> Self {
        Self { current }
    }
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.next.as_deref();
            &node.value
        })
    }
}

pub struct LinkedListIterMut<'a, T> {
    current: Option<&'a mut Node<T>>,
}

impl<'a, T> LinkedListIterMut<'a, T> {
    fn new(current: Option<&'a mut Node<T>>) -> Self {
        Self { current }
    }
}

impl<'a, T> Iterator for LinkedListIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.next.as_deref_mut();
            &mut node.value
        })
    }
}

impl<T> Iterator for LinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_linked_list() -> LinkedList<u32> {
        let next = Some(Box::new(Node {
            value: 2,
            next: None,
        }));
        LinkedList {
            head: Some(Node { value: 1, next }),
        }
    }

    #[test]
    fn test_push() {
        let mut ll: LinkedList<u32> = LinkedList::new();
        ll.push(1);
        let head = ll.head.as_ref().unwrap();
        assert_eq!(head.value, 1);
        assert!(head.next.is_none());
        ll.push(2);
        let head = ll.head.as_ref().unwrap();
        assert_eq!(head.value, 2);
        assert_eq!(head.next.as_ref().unwrap().value, 1);
    }

    #[test]
    fn test_peek() {
        let ll = new_linked_list();
        assert_eq!(ll.peek(), Some(&1));
    }

    #[test]
    fn test_pop() {
        let mut ll = new_linked_list();
        assert_eq!(ll.pop(), Some(1));
        assert_eq!(ll.pop(), Some(2));
        assert_eq!(ll.pop(), None);
    }

    #[test]
    fn test_clear() {
        let mut ll = new_linked_list();
        ll.clear();
        assert!(ll.head.is_none());
    }

    #[test]
    fn test_iter() {
        let ll = new_linked_list();
        let mut iter = ll.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut ll = new_linked_list();
        let mut iter = ll.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_into_iter() {
        let mut ll = new_linked_list();
        assert_eq!(ll.next(), Some(1));
        assert_eq!(ll.next(), Some(2));
        assert_eq!(ll.next(), None);
    }

    #[test]
    fn test_combined() {
        let mut ll: LinkedList<u32> = LinkedList::new();
        ll.push(1);
        assert_eq!(ll.peek(), Some(&1));
        ll.pop();
        assert_eq!(ll.peek(), None);
        ll.push(1);
        ll.push(2);
        ll.push(3);
        assert_eq!(ll.peek(), Some(&3));
        ll.pop();
        assert_eq!(ll.peek(), Some(&2));
        ll.clear();
        assert!(ll.head.is_none());
    }
}
