#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<*mut Node<T>>
}

// If this is not implemeted then memory allocated to Heap will not be deallocated
// resulting in a memory leak. This happens because we are using unsafe rust 
// and raw pointers in Box. This means the compiler itself has no information as 
// to when to deallocate memory. This custom implementation of Drop trait makes sure
// the compiler knows how to cleanup this data structure when the program ends. 
impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        unsafe {
            if !self.next.is_none() {
                let mut ptr: *mut Node<T> = self.next.unwrap();
                while !(*ptr).next.is_none() {
                    let temp = ptr;
                    ptr = (*ptr).next.unwrap();
                    std::alloc::dealloc(temp as *mut u8, std::alloc::Layout::new::<Node<T>>());
                }
                std::alloc::dealloc(ptr as *mut u8, std::alloc::Layout::new::<Node<T>>());
            }
        }
    }
}

impl<T> Node<T> where T: std::fmt::Display + std::fmt::Debug + std::marker::Copy {
    fn new(value: T) -> Self {
        Self {
            value,
            next: None
        }
    }

    fn append(&mut self, value: T) {
        unsafe {
            let mut head_ptr: *mut Node<T> = self;
            while !(*head_ptr).next.is_none() {
                head_ptr = (*head_ptr).next.unwrap();
            }
            let newnode = Box::new(Node::new(value));

            let newnode = Box::into_raw(newnode);
            (*head_ptr).next = Some(newnode);
        }
    }

    fn add_at_index(&mut self, index: usize, value: T) {
        unsafe {
            // Adding at first position
            if index == 0 {
                // let mut new_head = Node::new(value);
                let old_head = Box::new(Node::new(self.value));
                let old_head = Box::into_raw(old_head);
                (*old_head).next = self.next; 
                self.value = value;
                self.next = Some(old_head);
                return
            }

            // Add after index
            let mut head_ptr: *mut Node<T> = self;
            let mut count: usize = 1;
            while !(*head_ptr).next.is_none() {
                if count == index {
                    let mut newnode = Box::new(Node::new(value));
                    newnode.next = (*head_ptr).next;
                    let newnode = Box::into_raw(newnode);
                    (*head_ptr).next = Some(newnode);
                    return
                }
                head_ptr = (*head_ptr).next.unwrap();
                count += 1;
            }

            // Check if adding after last index
            if index == count {
                let newnode = Box::new(Node::new(value));
                let newnode = Box::into_raw(newnode);
                (*head_ptr).next = Some(newnode);
                return
            }
            panic!("The Linked List is of length {}. Cannot add at index {}", count, index);
        }
    }

    // You can also use unsafe in fn signature. This would just mean that the 
    // user has to explicitly open an unsafe block whenever xey are using this function.
    // Useful to make them aware that the code is "unsafe" in Rust terms.
    fn print(&self) {
        unsafe {
            let mut head_ptr: *const Node<T> = self;

            while !(*head_ptr).next.is_none() {
                println!("{}", (*head_ptr).value);
                head_ptr = (*head_ptr).next.unwrap();
            }
            println!("{}", (*head_ptr).value);
        }
    }

}

fn main() {
    let mut head = Node::new("Start");
    head.add_at_index(0, "89");
    head.add_at_index(1, "90");
    head.add_at_index(3, "91");
    head.add_at_index(0, "88");
    head.add_at_index(5, "92");
    // head.add_at_index(100, "I will Panic");
    head.print();
}