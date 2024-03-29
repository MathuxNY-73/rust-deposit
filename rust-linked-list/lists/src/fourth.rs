use std::rc::Rc;
use std::cell::RefCell;

#[derive(Default)]
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Default)]
pub struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { elem, next: None, prev: None }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn push_front(&mut self, elem: T) {
        // new node needs +2 links, everything else should be +0
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                // non-empty list, need to connect the old_head
                old_head.borrow_mut().prev = Some(new_head.clone());  // +1 new_head
                new_head.borrow_mut().next = Some(old_head);          // +1 old_head
                self.head = Some(new_head);                           // +1 new_head, -1 old_head
                // total: +2 new_head, +0 old_head
            }
            None => {
                // empty list, need to set the tail
                self.tail = Some(new_head.clone());                   // +1 new_head
                self.head = Some(new_head);                           // +1 new_head
                // total: +2 new_head
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        // need to take the old head, ensuring it's -2
        self.head.take().map(|old_head| {  // -1 old_head
            match old_head.borrow_mut().next.take() {            // -1 new_head
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();           // -1 old_head
                    self.head = Some(new_head);                  // +1 new_head
                    // total: -2 old_head, +0 new_head
                }
                None => {
                    self.tail.take();                            // -1 old_head
                    // total: -2 old_head
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    // pub fn peek_front(&self) -> Option<Ref<T>> {
    //     self.head.as_ref().map(|head_ref| Ref::map(head_ref.borrow(), |head| &node.elem))
    // }

    // pub fn peek_front_mut() -> Option<RefMut<T>> {
    // }
}

impl<T> Drop for List<T> {
    fn  drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
}
