use linkedlist::Node;

fn main() {
    let mut head = Node::new("Start");
    head.add_at_index(0, "89");
    head.add_at_index(1, "90");
    head.add_at_index(3, "91");
    head.add_at_index(0, "88");
    head.add_at_index(5, "92");
    // head.add_at_index(100, "I will Panic");
    // Expected Output: 88-89-90-Start-91-92
    head.print();
    println!("This is LinkedList before removing elements...");
    println!("Removed {:?}", head.remove_at_index(0));
    println!("Removed {:?}", head.remove_at_index(2));
    println!("Removed {:?}", head.remove_at_index(3));
    println!("LinkedList after removing elements...");
    head.print();
}