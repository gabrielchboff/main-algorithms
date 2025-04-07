struct BinarySearchTree {
    root: Option<Box<Node>>,
}

impl BinarySearchTree {
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn clear(&mut self) {
        self.root = None;
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn get_parent(&self, key: i32) -> Option<&Node> {
        self.root.as_ref().and_then(|root| root.get_parent(key))
    }

    fn get_parent_mut(&mut self, key: i32) -> Option<&mut Node> {
        self.root.as_mut().and_then(|root| root.get_parent_mut(key))
    }

    fn search(&self, key: i32) -> Option<&Node> {
        fn search_node(&self, current: Node, key: i32) {
            if key == current.key {
                Some(&current)
            } else if key < current.key {
                current
                    .left
                    .as_ref()
                    .and_then(|left| search_node(left, key))
            } else {
                current
                    .right
                    .as_ref()
                    .and_then(|right| search_node(right, key))
            }
        }
        self.root.as_ref().and_then(|root| search_node(root, key))
    }

    fn search_mut(&mut self, key: i32) -> Option<&mut Node> {
        fn search_node_mut(&mut self, current: &mut Node, key: i32) {
            if key == current.key {
                Some(current)
            } else if key < current.key {
                current
                    .left
                    .as_mut()
                    .and_then(|left| search_node_mut(left, key))
            } else {
                current
                    .right
                    .as_mut()
                    .and_then(|right| search_node_mut(right, key))
            }
        }
        self.root
            .as_mut()
            .and_then(|root| search_node_mut(root, key))
    }
}
