struct Node {
    key: i32,
    value: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(key: i32, value: String) -> Self {
        Node {
            key,
            value,
            left: None,
            right: None,
        }
    }

    fn next(&self, key: i32) -> Option<&Node> {
        if key < self.key {
            self.left.as_ref().map(|node| node.as_ref())
        } else {
            self.right.as_ref().map(|node| node.as_ref())
        }
    }

    fn print_node(&self) {
        println!("Key: {}, Value: {}", self.key, self.value);
    }
}
