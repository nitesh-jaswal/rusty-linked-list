use std::alloc::{dealloc, Layout};
use std::fmt::{Debug, Display};
use std::marker::Copy;
use std::ptr::NonNull;

// TODO: Create wrapper struct to manage Linked List instead of  directly interacting with Node
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
    T: Display + Debug + Copy,
{
    pub fn new(value: T) -> Self {
        Self {
             value, 
             next: None 
        }
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

    pub fn add_at_index(&mut self, index: usize, value: T) {
        unsafe {
            // Adding at first position
            if index == 0 {
                let old_head = Box::into_raw(Box::new(Node::new(self.value)));
                (*old_head).next = self.next; 
                self.value = value;
                self.next = Some(NonNull::new(old_head)).unwrap();
                return
            }

            // Add after index
            let mut head_ptr: *mut Node<T> = self;
            let mut count: usize = 1;
            while (*head_ptr).next.is_some() {
                if count == index {
                    let mut newnode = Box::new(Node::new(value));
                    newnode.next = (*head_ptr).next;
                    let newnode = Box::into_raw(newnode);
                    (*head_ptr).next = Some(NonNull::new(newnode)).unwrap();
                    return
                }
                head_ptr = (*head_ptr).next.unwrap().as_ptr();
                count += 1;
            }

            // Check if adding after last index
            if index == count {
                let newnode = Box::into_raw(Box::new(Node::new(value)));
                (*head_ptr).next = Some(NonNull::new(newnode)).unwrap();
                return
            }
            panic!("The Linked List is of length {}. Cannot add at index {}", count, index);
        }
    }
    // Because we are directly interacting with nodes, there can never be an 
    // empty LinkedList. The minimum no. of members will be 1 i.e. head
    // which would need to be dropped to clear it
    pub fn remove_at_index(&mut self, index: usize) -> Option<T> {
        unsafe {
            let count = self.count();
            if count == 0 || index + 1 > count {
                return None
            }
            // Last index is a special case
            let mut head_ptr: *mut Node<T> = self;
            let mut prev_node_ptr: *mut Node<T> = head_ptr;
            for _ in 0..index {
                prev_node_ptr = head_ptr;
                head_ptr = (*head_ptr).next.unwrap().as_ptr();
            }

            if index + 1 == count {
                // Retval will be the value in the node that has been currently iterated to i.e. head_ptr
                // Simply point the prev_node to None
                // Drop head_ptr
                let retval = Some((*head_ptr).value);
                (*prev_node_ptr).next = None;
                drop(head_ptr);
                retval
            }
            else {
                // Store the node after the current node as nextnode
                // Copy contents of nextnode into self
                // Drop nextnode
                let retval = Some((*head_ptr).value);
                let nextnode = (*head_ptr).next.unwrap().as_ptr();
                (*head_ptr).next = (*nextnode).next;
                (*head_ptr).value = (*nextnode).value;
                drop(nextnode);
                retval
            }
        }
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.remove_at_index(self.count() - 1)
    }

    // You can also use unsafe in fn signature. This would just mean that the 
    // user has to explicitly open an unsafe block whenever xey are using this function.
    // Useful to make them aware that the code is "unsafe" in Rust terms.
    pub fn print(&self) {
        unsafe {
            let mut head_ptr: *const Node<T> = self;
            let mut str_repr = String::new();
            loop {
                str_repr += format!("{}", (*head_ptr).value).as_str();
                if (*head_ptr).next.is_none() {
                    break
                }
                head_ptr = (*head_ptr).next.unwrap().as_ptr();
                str_repr += " -> ";

            }
            println!("{}", str_repr);
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
        list.add_at_index(2, 1111);
        list.add_at_index(2, 1234);
        list.append(666);
        list.remove_at_index(2);
        assert!(list.count() == 4);
    }

    #[test]
    fn check_removed_element() {
        let mut list = Node::new(1);
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);
        assert!(list.remove_at_index(0) == Some(1));
        assert!(list.pop() == Some(5));
        assert!(list.remove_at_index(1) == Some(3));
        assert!(list.remove_at_index(10) == None);
    }
}
