trait Queue<T> {
    fn queue(&mut self, data: T);

    fn dequeue(&mut self) -> Result<T, Error>;

    fn peek(&self) -> Result<T, Error>;

    fn count(&self) -> usize;

    fn is_empty(&self) -> bool;
}

pub struct ShuffleQueue<T> {
    queue: Vec<T>,
    count: usize,
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

impl<T> ShuffleQueue<T> {
    pub fn new() -> Self {
        ShuffleQueue {
            queue: Vec::<T>::new(),
            count: 0,
        }
    }
}

impl<T: Clone> Queue<T> for ShuffleQueue<T> {
    fn queue(&mut self, data: T) {
        self.queue.insert(self.count, data);
        self.count += 1;
    }

    fn dequeue(&mut self) -> Result<T, Error> {
        if self.count == 0 {
            Err(Error::new("can not dequeue from empty queue"))
        } else {
            self.count -= 1;
            Ok(self.queue.remove(0))
        }
    }

    fn peek(&self) -> Result<T, Error> {
        if self.count == 0 {
            Err(Error::new("can not peek empty queue"))
        } else {
            Ok(self.queue[0].clone())
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
    fn shuffle_queue() {
        let mut test_queue = ShuffleQueue::new();
        let (test_data_1, test_data_2, test_data_3) = (10, 42, 59492957);

        test_queue.queue(test_data_1.clone());
        test_queue.queue(test_data_2.clone());
        test_queue.queue(test_data_3.clone());

        assert!(test_queue.dequeue().unwrap() == test_data_1);
        assert!(test_queue.peek().unwrap() == test_data_2);
        assert!(test_queue.dequeue().unwrap() == test_data_2);
        assert!(test_queue.dequeue().unwrap() == test_data_3);
        assert!(test_queue.dequeue().is_err());
    }
}
