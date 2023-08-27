use std::mem;

// pub says we want people outside this module to be able to use List
// ABadLayoutList has several problems regarding its layout in memory.
pub enum ABadLayoutList {
    Empty,
    Elem(i32, Box<ABadLayoutList>),
}

pub struct List {
    head: Link,
}

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern does not match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // `boxed_node` goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

// The following code is an attempt at understanding how the compiler will implement the Drop
// trait for the object we declared here.
// impl Drop for Link {
//     fn drop(&mut self) {
//         match *self {
//             Link::Empty => {}; // Done!
//             Link::More(ref mut boxed_node) => {
//                 boxed_node.drop();  // tail recursive - good!
//             }
//         }
//     }
// }

// impl Drop for Box<Node> {
//     fn drop(&mut self) {
//         self.ptr.drop();  // uh oh, not tail recursive;
//         deallocate(self.ptr);
//     }
// }

// impl Drop for Node {
//     fn drop(&mut self) {
//         self.next.drop();
//     }
// }

mod test {

    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}