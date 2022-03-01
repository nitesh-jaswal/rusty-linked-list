use std::alloc::{dealloc, Layout};
use std::fmt::{Debug, Display};
use std::ptr::NonNull;

pub struct Node<T> {
    value: T,
    next: Option<NonNull<Node<T>>>,
}

// If this is not implemeted then memory allocated to Heap will not be deallocated
// resulting in a memory leak. This happens because we are using unsafe rust
// and raw pointers in Box. This means the compiler itself has no information as
// to when to deallocate memory. This custom implementation of Drop trait makes sure
// the compiler knows how to cleanup this data structure when the program ends.
impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        unsafe {
            if self.next.is_some() {
                let mut ptr: *mut Node<T> = self.next.unwrap().as_ptr();
                while (*ptr).next.is_some() {
                    let temp = ptr;
                    ptr = (*ptr).next.unwrap().as_ptr();
                    dealloc(temp as *mut u8, Layout::new::<Node<T>>());
                }
                dealloc(ptr as *mut u8, Layout::new::<Node<T>>());
            }
        }
    }
}

impl<T> Node<T>
where
    T: Display + Debug,
{
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }

    pub fn append(&mut self, value: T) {
        unsafe {
            let mut head_ptr: *mut Node<T> = self;
            while (*head_ptr).next.is_some() {
                head_ptr = (*head_ptr).next.unwrap().as_ptr();
            }
            let newnode = Box::into_raw(Box::new(Node::new(value)));
            (*head_ptr).next = Some(NonNull::new(newnode)).unwrap();
        }
    }

    // You can also use unsafe in fn signature. This would just mean that the
    // user has to explicitly open an unsafe block whenever xey are using this function.
    // Useful to make them aware that the code is "unsafe" in Rust terms.
    pub fn print(&self) {
        unsafe {
            let mut head_ptr: *const Node<T> = self;
            while (*head_ptr).next.is_some() {
                println!("{}", (*head_ptr).value);
                head_ptr = (*head_ptr).next.unwrap().as_ptr();
            }
            println!("{}", (*head_ptr).value);
        }
    }

    // Count the number of Nodes in the Linked List
    pub fn count(&self) -> usize {
        let mut count = 1;
        unsafe {
            let mut head_ptr: *const Node<T> = self;
            while (*head_ptr).next.is_some() {
                head_ptr = (*head_ptr).next.unwrap().as_ptr();
                count += 1;
            }
        }
        count
    }


}

#[cfg(test)]
mod tests {
    use crate::Node;

    #[test]
    fn basic_test() {
        let mut list = Node::new(69);
        list.append(420);
        assert!(list.count() == 2);
    }
}
