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

    fn insert(&self, key: i32) {
        fn insert_node(&self, current: Node, key: i32) {
            if current == null {
                return Node;
            }
            if key == current.key {
                Some(&current);
            }
            if key < current.key {
                current
                    .left
                    .as_ref()
                    .and_then(|left| insert_node(left, key))
            }
            else {
                current
                    .right
                    .as_ref()
                    .and_then(|right| insert_node(right, key))
            }
            return current;
        }
        self.root.as_ref().and_then(|root| insert_node(root, key));
    }
    insert_mut(&mut self, key: i32) {
        fn insert_node_mut(&mut self, current: &mut Node, key: i32) {
            if key == current.key {
                Some(current);
            }
            if key < current.key {
                current
                    .left
                    .as_mut()
                    .and_then(|left| insert_node_mut(left, key))
            }
            else {
                current
                    .right
                    .as_mut()
                    .and_then(|right| insert_node_mut(right, key))
            }
            return current;
        }
        self.root.as_mut().and_then(|root| insert_node_mut(root, key));
    }

    fn show_tree(&self) {
        fn show_tree_helper(node: &Node, depth: usize) {
            if let Some(right) = &node.right {
                show_tree_helper(right, depth + 1);
            }
            println!("{:width$}{}", "", node.key, width = depth * 4);
            if let Some(left) = &node.left {
                show_tree_helper(left, depth + 1);
            }
        }
        if let Some(root) = &self.root {
            show_tree_helper(root, 0);
        }
    }

    fn delete_by_merging(&mut self, key: i32) {
        let parent: Node;
        let current: Node;
        parernt = self.get_parent(key);
        current = self.get_parent(key);
        if current.is_none() {
            return None;
        }

        else if !current.left.is_none() && !current.right.is_none() {
            let mut at_the_right: Node = current.right.take().unwrap();
            while at_the_right.right.is_some() {
                at_the_right = at_the_right.right.take().unwrap();
            }
            at_the_right.left = current.left.take();
            if parent.is_none() {
                self.root = at_the_right;
            }
            else if parrent.left == current {
                parent.left = current.left;
            }
            else {
                parent.right = current.left;
            }

        }

        else {
            let next_node: Node = current.left || current.right;
            if current == self.root {
                self.root = next_node;
            }
            else if parent.left == current {z
                parent.left = next_node;
            }
            else {
                parent.right = next_node;
            }
        }
        return Some(current)
    }

    fn delete(&self, key: i32){
        return self.delete_by_merging(key);
    }

    fn pre_order_transversal(&self) {
        fn _pre_order_transversal(current: &Node) {
            if current.is_some() {
                println!("{}", current.value);
                pre_order_transversal(current.left.as_ref());
                pre_order_transversal(current.right.as_ref());
            }
        }
        _pre_order_transversal(&self.root);
    }

    fn in_order_transversal(&self) {
        fn _in_order_transversal(current: &Node) {
            if current.is_some() {
                _in_order_transversal(current.left.as_ref());
                println!("{}", current.value);
                _in_order_transversal(current.right.as_ref());
            }
        }
        _in_order_transversal(&self.root);
    }

    fn post_order_transversal(&self) {
        fn _post_order_transversal(current: &Node) {
            if current.is_some() {
                _post_order_transversal(current.left.as_ref());
                _post_order_transversal(current.right.as_ref());
                println!("{}", current.value);
            }
        }
        _post_order_transversal(&self.root);
    }

    fn count_internal(&self) -> usize {
        fn _count_internal(current: &Node) -> usize {
            if current.is_none() {
                return 0;
            }
            count = 0;
            if !current.left.is_none() {
                if !current.left.left.is_none() || !current.left.right.is_none() {
                    count += 1;
                }
                count += _count_internal(current.left.as_ref());
            }

            if !current.right.is_none() {
                if !current.right.left.is_none() || !current.right.right.is_none() {
                    count += 1;
                }
                count += _count_internal(current.right.as_ref());
            }
            return count;
        }
        if self.root.is_none() {
            return 0;
        }
        _count_internal(&self.root)
    }

    fn degree(&self, key: i32) -> i32 {
        let level = self.level(key);
        if level == -1 {
            return -1;
        }
        return level + 1;
    }

    fn height(&self, key: i32) -> i32 {
        fn _height(current: &Node, key: i32) -> i32 {
            if current.is_none() {
                return -1;
            }
            if current.left.is_none() && current.right.is_none() {
                return 0;
            }
            let left_height = _height(current.left.as_ref(), key);
            let right_height = _height(current.right.as_ref(), key);
            return 1 + std::cmp::max(left_height, right_height);
        }
        let current = self.root.as_ref();
        while current.is_some() {
            if current.as_ref().unwrap().key == key {
                return _height(current.as_ref(), key);
            }
            current = current.next(key);
        }
        return -1;
    }

    fn level(&self, key: i32) {
        fn _level(current: &Node, key: i32, level: i32) {
            !todo();
        }
    }

}
