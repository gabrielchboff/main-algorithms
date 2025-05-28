// A node in the linked list
struct Node {
    next: Option<Box<Node>>,
    data: i32,
}

/// A singly linked list implementation in Rust
pub struct LinkedList {
    head: Option<Box<Node>>,
    size: usize,
}

impl LinkedList {
    /// Creates a new empty linked list
    pub fn new() -> Self {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    /// Returns the number of elements in the list
    pub fn len(&self) -> usize {
        self.size
    }

    /// Returns true if the list is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Adds a new element to the front of the list
    pub fn push_front(&mut self, data: i32) {
        let new_node = Box::new(Node {
            next: self.head.take(),
            data,
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    /// Removes the first element from the list and returns it, or None if the list is empty
    pub fn pop_front(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.data
        })
    }

    /// Returns a reference to the first element, or None if the list is empty
    pub fn peek_front(&self) -> Option<&i32> {
        self.head.as_ref().map(|node| &node.data)
    }

    /// Adds a new element to the end of the list
    pub fn push_back(&mut self, data: i32) {
        // If the list is empty, use push_front
        if self.head.is_none() {
            self.push_front(data);
            return;
        }

        // Create a new node
        let new_node = Box::new(Node {
            next: None,
            data,
        });

        // Find the last node
        let mut current = &mut self.head;
        while let Some(ref mut node) = *current {
            if node.next.is_none() {
                // We found the last node, append the new node
                node.next = Some(new_node);
                self.size += 1;
                return;
            }
            current = &mut node.next;
        }
    }

    /// Checks if the list contains a specific value
    pub fn contains(&self, data: i32) -> bool {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.data == data {
                return true;
            }
            current = &node.next;
        }
        false
    }

    /// Clears the list, removing all elements
    pub fn clear(&mut self) {
        self.head = None;
        self.size = 0;
    }

    /// Returns an iterator over the elements of the list
    pub fn iter(&self) -> Iter {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

/// An iterator over the elements of a LinkedList
pub struct Iter<'a> {
    next: Option<&'a Node>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}

// Implement Drop to avoid stack overflow on recursive drop of large lists
impl Drop for LinkedList {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}

// Implement Display for LinkedList to make it easy to print
impl std::fmt::Display for LinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for item in self.iter() {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
            first = false;
        }
        write!(f, "]")
    }
}

// Implement Debug for LinkedList
impl std::fmt::Debug for LinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop_front() {
        let mut list = LinkedList::new();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.len(), 3);
        assert!(!list.is_empty());
        assert_eq!(list.peek_front(), Some(&3));

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_push_back() {
        let mut list = LinkedList::new();

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
    }

    #[test]
    fn test_contains() {
        let mut list = LinkedList::new();

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert!(list.contains(1));
        assert!(list.contains(2));
        assert!(list.contains(3));
        assert!(!list.contains(4));
    }

    #[test]
    fn test_clear() {
        let mut list = LinkedList::new();

        list.push_front(1);
        list.push_front(2);

        assert_eq!(list.len(), 2);

        list.clear();

        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_iter() {
        let mut list = LinkedList::new();

        list.push_front(3);
        list.push_front(2);
        list.push_front(1);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }
}
