use std::sync::Arc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Arc<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    // It is possible to have both forms:
    //  - fn prepend(&self, elem: T) -> Self { ... }
    //  - fn prepend(&self, elem: T) -> List<T> { ... }
    //
    // Self is an alias for `List<T>`. Using `List<T>` might however be less
    // confusing as here we return a new List and not `self`.
    pub fn prepend(&self, elem: T) -> Self {
        List { head: Some(Arc::new( Node { elem, next: self.head.clone() } )) }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn tail(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Arc::try_unwrap(node) {
                head = node.next.take();
            }
            else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {

    #[derive(Debug, PartialEq, Clone)]
    struct Test(i32);

    #[test]
    fn basics() {
        use super::List;

        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail().tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        use super::List;

        let list = List::new();

        let list = list.prepend(Test(1)).prepend(Test(2)).prepend(Test(3));
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&Test(3)));
        assert_eq!(iter.next(), Some(&Test(2)));
        assert_eq!(iter.next(), Some(&Test(1)));
        assert_eq!(iter.next(), None);

        assert_eq!(list.head(), Some(&Test(3)));
    }
}