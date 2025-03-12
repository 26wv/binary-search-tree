use std::cmp::Ordering;
use std::fmt;

// Define custom error types
#[derive(Debug)]
enum BSTError {
    DuplicateValue,
    ValueNotFound,
}

impl fmt::Display for BSTError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BSTError::DuplicateValue => write!(f, "Duplicate value: cannot insert the same value twice"),
            BSTError::ValueNotFound => write!(f, "Value not found: cannot delete a non-existent value"),
        }
    }
}

// Define the structure of a node in the BST
#[derive(Debug)]
struct Node<T: Ord + Clone> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

// Implement methods for the Node
impl<T: Ord + Clone> Node<T> {
    // Create a new node
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) -> Result<(), BSTError> {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(ref mut left) = self.left {
                    left.insert(value)
                } else {
                    self.left = Some(Box::new(Node::new(value)));
                    Ok(())
                }
            }
            Ordering::Greater => {
                if let Some(ref mut right) = self.right {
                    right.insert(value)
                } else {
                    self.right = Some(Box::new(Node::new(value)));
                    Ok(())
                }
            }
            Ordering::Equal => Err(BSTError::DuplicateValue),
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Less => self.left.as_ref().map_or(false, |left| left.search(value)),
            Ordering::Greater => self.right.as_ref().map_or(false, |right| right.search(value)),
            Ordering::Equal => true,
        }
    }

    // Find the minimum value in the BST
    fn find_min(&self) -> &T {
        self.left.as_ref().map_or(&self.value, |left| left.find_min())
    }

    // Find the maximum value in the BST
    fn find_max(&self) -> &T {
        self.right.as_ref().map_or(&self.value, |right| right.find_max())
    }

    // Delete a value from the BST
    fn delete(&mut self, value: T) -> Result<Option<Box<Node<T>>>, BSTError> {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(ref mut left) = self.left {
                    self.left = left.delete(value)?;
                } else {
                    return Err(BSTError::ValueNotFound);
                }
            }
            Ordering::Greater => {
                if let Some(ref mut right) = self.right {
                    self.right = right.delete(value)?;
                } else {
                    return Err(BSTError::ValueNotFound);
                }
            }
            Ordering::Equal => {
                if self.left.is_none() {
                    return Ok(self.right.take());
                } else if self.right.is_none() {
                    return Ok(self.left.take());
                } else {
                    // Node has two children, find the in-order successor (minimum in the right subtree)
                    let min_value = self.right.as_ref().unwrap().find_min().clone();
                    self.value = min_value;
                    self.right = self.right.as_mut().unwrap().delete(self.value.clone())?;
                }
            }
        }
        Ok(Some(Box::new(Node {
            value: self.value.clone(),
            left: self.left.take(),
            right: self.right.take(),
        })))
    }

    // In-order traversal (left, root, right)
    fn in_order_traversal(&self, result: &mut Vec<T>) {
        if let Some(ref left) = self.left {
            left.in_order_traversal(result);
        }
        result.push(self.value.clone());
        if let Some(ref right) = self.right {
            right.in_order_traversal(result);
        }
    }

    // Pre-order traversal (root, left, right)
    fn pre_order_traversal(&self, result: &mut Vec<T>) {
        result.push(self.value.clone());
        if let Some(ref left) = self.left {
            left.pre_order_traversal(result);
        }
        if let Some(ref right) = self.right {
            right.pre_order_traversal(result);
        }
    }

    // Post-order traversal (left, right, root)
    fn post_order_traversal(&self, result: &mut Vec<T>) {
        if let Some(ref left) = self.left {
            left.post_order_traversal(result);
        }
        if let Some(ref right) = self.right {
            right.post_order_traversal(result);
        }
        result.push(self.value.clone());
    }

    // Count the number of nodes in the BST
    fn count_nodes(&self) -> usize {
        let mut count = 1;
        if let Some(ref left) = self.left {
            count += left.count_nodes();
        }
        if let Some(ref right) = self.right {
            count += right.count_nodes();
        }
        count
    }

    // Check if the BST is balanced
    fn is_balanced(&self) -> bool {
        let left_height = self.left.as_ref().map_or(0, |left| left.height());
        let right_height = self.right.as_ref().map_or(0, |right| right.height());
        (left_height as i32 - right_height as i32).abs() <= 1
    }

    // Calculate the height of the BST
    fn height(&self) -> usize {
        let left_height = self.left.as_ref().map_or(0, |left| left.height());
        let right_height = self.right.as_ref().map_or(0, |right| right.height());
        1 + left_height.max(right_height)
    }
}

// Define the structure of the BST
#[derive(Debug)]
struct BinarySearchTree<T: Ord + Clone> {
    root: Option<Box<Node<T>>>,
}

// Implement methods for the BST
impl<T: Ord + Clone> BinarySearchTree<T> {
    // Create a new empty BST
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) -> Result<(), BSTError> {
        if let Some(ref mut root) = self.root {
            root.insert(value)
        } else {
            self.root = Some(Box::new(Node::new(value)));
            Ok(())
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        self.root.as_ref().map_or(false, |root| root.search(value))
    }

    // Delete a value from the BST
    fn delete(&mut self, value: T) -> Result<(), BSTError> {
        if let Some(ref mut root) = self.root {
            self.root = root.delete(value)?;
            Ok(())
        } else {
            Err(BSTError::ValueNotFound)
        }
    }

    // Find the minimum value in the BST
    fn find_min(&self) -> Option<&T> {
        self.root.as_ref().map(|root| root.find_min())
    }

    // Find the maximum value in the BST
    fn find_max(&self) -> Option<&T> {
        self.root.as_ref().map(|root| root.find_max())
    }

    // In-order traversal
    fn in_order_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            root.in_order_traversal(&mut result);
        }
        result
    }

    // Pre-order traversal
    fn pre_order_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            root.pre_order_traversal(&mut result);
        }
        result
    }

    // Post-order traversal
    fn post_order_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            root.post_order_traversal(&mut result);
        }
        result
    }

    // Count the number of nodes in the BST
    fn count_nodes(&self) -> usize {
        self.root.as_ref().map_or(0, |root| root.count_nodes())
    }

    // Check if the BST is balanced
    fn is_balanced(&self) -> bool {
        self.root.as_ref().map_or(true, |root| root.is_balanced())
    }

    // Calculate the height of the BST
    fn height(&self) -> usize {
        self.root.as_ref().map_or(0, |root| root.height())
    }
}

fn main() {
    let mut bst = BinarySearchTree::new();

    // Insert some values into the BST
    match bst.insert(10) {
        Ok(_) => println!("Inserted 10"),
        Err(e) => println!("Error: {}", e),
    }
    match bst.insert(5) {
        Ok(_) => println!("Inserted 5"),
        Err(e) => println!("Error: {}", e),
    }
    match bst.insert(15) {
        Ok(_) => println!("Inserted 15"),
        Err(e) => println!("Error: {}", e),
    }
    match bst.insert(3) {
        Ok(_) => println!("Inserted 3"),
        Err(e) => println!("Error: {}", e),
    }
    match bst.insert(7) {
        Ok(_) => println!("Inserted 7"),
        Err(e) => println!("Error: {}", e),
    }
    match bst.insert(12) {
        Ok(_) => println!("Inserted 12"),
        Err(e) => println!("Error: {}", e),
    }
    match bst.insert(18) {
        Ok(_) => println!("Inserted 18"),
        Err(e) => println!("Error: {}", e),
    }

    // Try inserting a duplicate value
    match bst.insert(10) {
        Ok(_) => println!("Inserted 10"),
        Err(e) => println!("Error: {}", e), // Should print: Error: Duplicate value
    }

    // Search for values in the BST
    println!("Search for 7: {}", bst.search(7)); // Should print: true
    println!("Search for 12: {}", bst.search(12)); // Should print: true
    println!("Search for 20: {}", bst.search(20)); // Should print: false

    // Find minimum and maximum values
    println!("Minimum value: {:?}", bst.find_min()); // Should print: Some(3)
    println!("Maximum value: {:?}", bst.find_max()); // Should print: Some(18)

    // Perform traversals
    println!("In-order traversal: {:?}", bst.in_order_traversal()); // Should print: [3, 5, 7, 10, 12, 15, 18]
    println!("Pre-order traversal: {:?}", bst.pre_order_traversal()); // Should print: [10, 5, 3, 7, 15, 12, 18]
    println!("Post-order traversal: {:?}", bst.post_order_traversal()); // Should print: [3, 7, 5, 12, 18, 15, 10]

    // Count the number of nodes
    println!("Number of nodes: {}", bst.count_nodes()); // Should print: 7

    // Check if the tree is balanced
    println!("Is balanced: {}", bst.is_balanced()); // Should print: true

    // Delete a node
    match bst.delete(15) {
        Ok(_) => println!("Deleted 15"),
        Err(e) => println!("Error: {}", e),
    }
    println!("In-order traversal after deleting 15: {:?}", bst.in_order_traversal()); // Should print: [3, 5, 7, 10, 12, 18]

    // Try deleting a non-existent value
    match bst.delete(20) {
        Ok(_) => println!("Deleted 20"),
        Err(e) => println!("Error: {}", e), // Should print: Error: Value not found
    }

    // Check height of the tree
    println!("Height of the tree: {}", bst.height()); // Should print: 3
}