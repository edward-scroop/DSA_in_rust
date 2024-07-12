trait Stack<T: Clone> {
    fn push(&mut self, data: T);

    fn pop(&mut self) -> Result<T, Error>;

    fn peek(&self) -> Result<T, Error>;

    fn count(&self) -> usize;

    fn is_empty(&self) -> bool;
}

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

pub struct VecStack<T> {
    stack: Vec<T>,
    count: usize,
}

impl<T> VecStack<T> {
    pub fn new() -> Self {
        VecStack {
            stack: Vec::<T>::new(),
            count: 0,
        }
    }
}

impl<T: Clone> Stack<T> for VecStack<T> {
    fn push(&mut self, data: T) {
        self.stack.insert(self.count, data);
        self.count += 1;
    }

    fn pop(&mut self) -> Result<T, Error> {
        if self.count == 0 {
            Err(Error::new("can not pop empty stack"))
        } else {
            self.count -= 1;
            Ok(self.stack.remove(self.count))
        }
    }

    fn peek(&self) -> Result<T, Error> {
        if self.count == 0 {
            Err(Error::new("stack is empty"))
        } else {
            Ok(self.stack[self.count - 1].clone())
        }
    }

    fn count(&self) -> usize {
        self.count
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }
}

pub struct ListStack<T> {
    root_node: Option<Box<ListStackNode<T>>>,
    count: usize,
}

struct ListStackNode<T> {
    data: T,
    next_node: Option<Box<ListStackNode<T>>>,
}

impl<T> ListStack<T> {
    pub fn new() -> Self {
        ListStack {
            root_node: None,
            count: 0,
        }
    }
}

impl<T> ListStackNode<T> {
    fn new(data: T, next_node: Option<Box<ListStackNode<T>>>) -> Self {
        ListStackNode { data, next_node }
    }
}

impl<T: Clone> Stack<T> for ListStack<T> {
    fn push(&mut self, data: T) {
        let new_node = ListStackNode::new(data, self.root_node.take());
        self.root_node = Some(Box::new(new_node));
        self.count += 1;
    }

    fn pop(&mut self) -> Result<T, Error> {
        if self.count == 0 {
            Err(Error::new("can not pop empty stack"))
        } else {
            let mut old_root_node = self.root_node.take();
            let new_node = old_root_node.as_mut().unwrap().next_node.take();
            self.root_node = new_node;
            self.count -= 1;
            Ok(old_root_node.unwrap().data)
        }
    }

    fn peek(&self) -> Result<T, Error> {
        if self.count == 0 {
            Err(Error::new("stack is empty"))
        } else {
            Ok(self.root_node.as_ref().unwrap().data.clone())
        }
    }

    fn count(&self) -> usize {
        self.count
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_stack_count() {
        let mut test_stack = VecStack::new();
        let (test_data_1, test_data_2, test_data_3) = (10, 42, 59492957);

        assert!(test_stack.count() == 0);
        test_stack.push(test_data_1.clone());
        assert!(test_stack.count() == 1);
        test_stack.push(test_data_2.clone());
        assert!(test_stack.count() == 2);
        test_stack.push(test_data_3.clone());
        assert!(test_stack.count() == 3);
    }

    #[test]
    fn vec_stack_push() {
        let mut test_stack = VecStack::new();
        let (test_data_1, test_data_2, test_data_3) = (10, 42, 59492957);

        test_stack.push(test_data_1.clone());
        test_stack.push(test_data_2.clone());
        test_stack.push(test_data_3.clone());
        assert!(test_stack.count() == 3);
        assert!(test_stack.is_empty() == false);
    }

    #[test]
    fn vec_stack_pop() {
        let mut test_stack = VecStack::new();
        let (test_data_1, test_data_2, test_data_3) = (10, 42, 59492957);

        test_stack.push(test_data_1.clone());
        test_stack.push(test_data_2.clone());
        test_stack.push(test_data_3.clone());

        assert!(test_stack.pop().unwrap() == test_data_3);
        assert!(test_stack.pop().unwrap() == test_data_2);
        assert!(test_stack.pop().unwrap() == test_data_1);

        assert!(test_stack.count() == 0);
        assert!(test_stack.is_empty());
        assert!(test_stack.pop().is_err());
    }

    #[test]
    fn vec_stack_peek() {
        let mut test_stack = VecStack::new();
        let (test_data_1, test_data_2, test_data_3) = (10, 42, 59492957);

        test_stack.push(test_data_1.clone());
        test_stack.push(test_data_2.clone());
        test_stack.push(test_data_3.clone());

        assert!(test_stack.peek().unwrap() == test_data_3);
        assert!(test_stack.pop().unwrap() == test_data_3);
        assert!(test_stack.peek().unwrap() == test_data_2);
        assert!(test_stack.pop().unwrap() == test_data_2);
        assert!(test_stack.peek().unwrap() == test_data_1);
        assert!(test_stack.pop().unwrap() == test_data_1);
        assert!(test_stack.is_empty());
        assert!(test_stack.peek().is_err());
    }

    #[test]
    fn list_stack_count() {
        let mut test_stack = ListStack::new();
        let (test_data_1, test_data_2, test_data_3) = (10, 42, 59492957);

        assert!(test_stack.count() == 0);
        test_stack.push(test_data_1.clone());
        assert!(test_stack.count() == 1);
        test_stack.push(test_data_2.clone());
        assert!(test_stack.count() == 2);
        test_stack.push(test_data_3.clone());
        assert!(test_stack.count() == 3);
    }

    #[test]
    fn list_stack_push() {
        let mut test_stack = ListStack::new();
        let (test_data_1, test_data_2, test_data_3) = (10, 42, 59492957);

        test_stack.push(test_data_1.clone());
        test_stack.push(test_data_2.clone());
        test_stack.push(test_data_3.clone());
        assert!(test_stack.count() == 3);
        assert!(test_stack.is_empty() == false);
    }

    #[test]
    fn list_stack_pop() {
        let mut test_stack = ListStack::new();
        let (test_data_1, test_data_2, test_data_3) = (10, 42, 59492957);

        test_stack.push(test_data_1.clone());
        test_stack.push(test_data_2.clone());
        test_stack.push(test_data_3.clone());

        assert!(test_stack.pop().unwrap() == test_data_3);
        assert!(test_stack.pop().unwrap() == test_data_2);
        assert!(test_stack.pop().unwrap() == test_data_1);

        assert!(test_stack.count() == 0);
        assert!(test_stack.is_empty());
        assert!(test_stack.pop().is_err());
    }

    #[test]
    fn list_stack_peek() {
        let mut test_stack = ListStack::new();
        let (test_data_1, test_data_2, test_data_3) = (10, 42, 59492957);

        test_stack.push(test_data_1.clone());
        test_stack.push(test_data_2.clone());
        test_stack.push(test_data_3.clone());

        assert!(test_stack.peek().unwrap() == test_data_3);
        assert!(test_stack.pop().unwrap() == test_data_3);
        assert!(test_stack.peek().unwrap() == test_data_2);
        assert!(test_stack.pop().unwrap() == test_data_2);
        assert!(test_stack.peek().unwrap() == test_data_1);
        assert!(test_stack.pop().unwrap() == test_data_1);
        assert!(test_stack.is_empty());
        assert!(test_stack.peek().is_err());
    }
}
