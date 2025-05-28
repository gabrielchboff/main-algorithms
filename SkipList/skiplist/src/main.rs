mod skiplist;
mod linkedlist;

fn main() {
    // Create a new linked list
    let mut list = linkedlist::LinkedList::new();

    // Test push_front and push_back
    println!("Adding elements to the list...");
    list.push_front(2);
    list.push_front(1);
    list.push_back(3);
    list.push_back(4);

    // Display the list
    println!("List: {}", list);
    println!("List length: {}", list.len());

    // Test contains
    println!("Contains 2: {}", list.contains(2));
    println!("Contains 5: {}", list.contains(5));

    // Test peek_front
    if let Some(value) = list.peek_front() {
        println!("Front element: {}", value);
    }

    // Test pop_front
    println!("Removing elements from the front...");
    while let Some(value) = list.pop_front() {
        println!("Popped: {}", value);
    }

    // Check if the list is empty
    println!("Is the list empty? {}", list.is_empty());

    // Test iterator
    println!("Adding elements and iterating...");
    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    print!("Iterating over list: ");
    for item in list.iter() {
        print!("{} ", item);
    }
    println!();

    // Test clear
    println!("Clearing the list...");
    list.clear();
    println!("Is the list empty? {}", list.is_empty());
}
