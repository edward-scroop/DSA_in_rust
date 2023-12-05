use std::cell::{Ref, RefCell};
use std::rc::Rc;

#[derive(Debug)]
pub struct Error {
    details: String,
}

impl Error {
    pub fn new(msg: &str) -> Error {
        Error {
            details: msg.to_string(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

pub struct LinkedList<T> {
    head_node: Option<Rc<RefCell<LinkedListNode<T>>>>,
    tail_node: Option<Rc<RefCell<LinkedListNode<T>>>>,
}

pub struct IntoIter<T>(LinkedList<T>);

struct LinkedListNode<T> {
    data: T,
    next_node: Option<Rc<RefCell<LinkedListNode<T>>>>,
    previous_node: Option<Rc<RefCell<LinkedListNode<T>>>>,
}

impl<T> LinkedListNode<T> {
    fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(LinkedListNode {
            data,
            next_node: None,
            previous_node: None,
        }))
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head_node: None,
            tail_node: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head_node.is_none()
    }

    pub fn insert_first(&mut self, data: T) {
        let new_head = LinkedListNode::new(data);
        match self.head_node.take() {
            Some(old_head) => {
                old_head.borrow_mut().previous_node = Some(new_head.clone());
                new_head.borrow_mut().next_node = Some(old_head);
                self.head_node = Some(new_head);
            }
            None => {
                self.tail_node = Some(new_head.clone());
                self.head_node = Some(new_head);
            }
        }
    }

    pub fn insert_last(&mut self, data: T) {
        let new_tail = LinkedListNode::new(data);
        match self.tail_node.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next_node = Some(new_tail.clone());
                new_tail.borrow_mut().previous_node = Some(old_tail);
                self.tail_node = Some(new_tail);
            }
            None => {
                self.tail_node = Some(new_tail.clone());
                self.head_node = Some(new_tail);
            }
        }
    }

    pub fn remove_first(&mut self) -> Result<T, Error> {
        match self.head_node.take() {
            Some(old_head) => {
                match old_head.borrow_mut().next_node.take() {
                    Some(new_head) => {
                        new_head.borrow_mut().previous_node.take();
                        self.head_node = Some(new_head);
                    }
                    None => self.tail_node = None,
                }
                Ok(Rc::try_unwrap(old_head).ok().unwrap().into_inner().data)
            }
            None => Err(Error::new("list is empty")),
        }
    }

    pub fn remove_last(&mut self) -> Result<T, Error> {
        match self.tail_node.take() {
            Some(old_tail) => {
                match old_tail.borrow_mut().previous_node.take() {
                    Some(new_tail) => {
                        new_tail.borrow_mut().next_node.take();
                        self.tail_node = Some(new_tail);
                    }
                    None => self.head_node = None,
                }
                Ok(Rc::try_unwrap(old_tail).ok().unwrap().into_inner().data)
            }
            None => Err(Error::new("list is empty")),
        }
    }

    pub fn peek_first(&self) -> Result<Ref<T>, Error> {
        self.head_node
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.data))
            .ok_or(Error::new("list is empty"))
    }

    pub fn peek_last(&self) -> Result<Ref<T>, Error> {
        self.tail_node
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.data))
            .ok_or(Error::new("list is empty"))
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.remove_first().is_ok() {}
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.remove_first().ok()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.remove_last().ok()
    }
}

mod tests {
    use super::*;

    #[test]
    fn linked_list_insert_first_and_remove_first() {
        let mut test_list = LinkedList::new();
        let (test_data_1, test_data_2, test_data_3, test_data_4, test_data_5) =
            (10, 42, 59492957, -2222, -9683491);

        test_list.insert_first(test_data_1.clone());
        assert!(
            test_list.peek_first().is_ok(),
            "failed to insert first data object"
        );
        test_list.insert_first(test_data_2.clone());
        assert!(
            test_list.peek_first().is_ok(),
            "failed to insert second data object"
        );
        test_list.insert_first(test_data_3.clone());
        assert!(
            test_list.peek_first().is_ok(),
            "failed to insert third data object"
        );
        test_list.insert_first(test_data_4.clone());
        assert!(
            test_list.peek_first().is_ok(),
            "failed to insert fourth data object"
        );
        test_list.insert_first(test_data_5.clone());
        assert!(
            test_list.peek_first().is_ok(),
            "failed to insert fifth data object"
        );

        assert!(
            test_list.remove_first().unwrap() == test_data_5,
            "first item doesn't match"
        );
        assert!(
            test_list.remove_first().unwrap() == test_data_4,
            "second item doesn't match"
        );
        assert!(
            test_list.remove_first().unwrap() == test_data_3,
            "third item doesn't match"
        );
        assert!(
            test_list.remove_first().unwrap() == test_data_2,
            "fourth item doesn't match"
        );
        assert!(
            test_list.remove_first().unwrap() == test_data_1,
            "fifth item doesn't match"
        );
        assert!(
            test_list.remove_first().is_err(),
            "item was removed from empty list"
        );
    }

    #[test]
    fn linked_list_insert_first_and_remove_last() {
        let mut test_list = LinkedList::new();
        let (test_data_1, test_data_2, test_data_3, test_data_4, test_data_5) =
            (10, 42, 59492957, -2222, -9683491);

        test_list.insert_last(test_data_1.clone());
        assert!(
            test_list.peek_first().is_ok(),
            "failed to insert first data object"
        );
        test_list.insert_first(test_data_2.clone());
        assert!(
            test_list.peek_first().is_ok(),
            "failed to insert second data object"
        );
        test_list.insert_first(test_data_3.clone());
        assert!(
            test_list.peek_first().is_ok(),
            "failed to insert third data object"
        );
        test_list.insert_first(test_data_4.clone());
        assert!(
            test_list.peek_first().is_ok(),
            "failed to insert fourth data object"
        );
        test_list.insert_first(test_data_5.clone());
        assert!(
            test_list.peek_first().is_ok(),
            "failed to insert fifth data object"
        );

        assert!(
            test_list.remove_last().unwrap() == test_data_1,
            "first item doesn't match"
        );
        assert!(
            test_list.remove_last().unwrap() == test_data_2,
            "second item doesn't match"
        );
        assert!(
            test_list.remove_last().unwrap() == test_data_3,
            "third item doesn't match"
        );
        assert!(
            test_list.remove_last().unwrap() == test_data_4,
            "fourth item doesn't match"
        );
        assert!(
            test_list.remove_last().unwrap() == test_data_5,
            "fifth item doesn't match"
        );
        assert!(
            test_list.remove_last().is_err(),
            "item was removed from empty list"
        );
    }
}
