/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/


#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	q1:Queue<T>,
	q2:Queue<T>
}

impl<T> myStack<T> {
	pub fn new() -> Self {
		Self {
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
		}
	}

	pub fn push(&mut self, elem: T) {
		// 将新元素加入非空的队列（或q1如果两个都是空的）
		if self.q1.is_empty() {
			self.q2.enqueue(elem);
		} else {
			self.q1.enqueue(elem);
		}
	}

	pub fn pop(&mut self) -> Result<T, &str> {
		if self.is_empty() {
			return Err("Stack is empty");
		}

		// 确定哪个队列非空
		let (from, to) = if self.q1.is_empty() {
			(&mut self.q2, &mut self.q1)
		} else {
			(&mut self.q1, &mut self.q2)
		};

		// 将元素从一个队列移到另一个队列，直到只剩一个元素
		while from.size() > 1 {
			if let Ok(item) = from.dequeue() {
				to.enqueue(item);
			}
		}

		// 弹出最后一个元素（这就是栈的顶部元素）
		from.dequeue().map_err(|_| "Stack is empty")
	}

	pub fn is_empty(&self) -> bool {
		self.q1.is_empty() && self.q2.is_empty()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}
