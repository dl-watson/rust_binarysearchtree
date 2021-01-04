use std::cmp::Ordering;

// class Node {
//     value;
//     left;
//     right;

// I'm going to define a tree that works for any type. This is harder in Rust because types of
// different sizes are handled differently memory-wise, so we'll be doing more memory management
// code that won't be relevant to your JavaScript code. This would be easier in Rust if I just said
// that this code only works for 32 bit integers.
#[derive(Debug, PartialEq, Eq)]
struct Node<T: Ord> {
    value: T,
    // Option is Rust's version of: this value *could* be null. It's explicit. null == None in Rust.
    // Box is memory management stuff; just ignore it.
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    //     constructor(value) {
    //       this.value = value;
    //       this.left = null;
    //       this.right = null;
    //     }
    //   }

    // Self means that it returns the type in context: Node<T>.
    // Self { } constructs the type in context: Node<T>.
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    //     #insert(node, at) {
    //       if (node.value < at.value) {
    //         if (!at.left) at.left = node;
    //         else this.#insert(node, at.left);
    //       } else {
    //         if (!at.right) at.right = node;
    //         else this.#insert(node, at.right);
    //       }
    //     }

    // &mut Self means that this method requires that the Node we give to insert at
    // be *mutable*, because we're going to change its data.
    fn insert_at(self, at: &mut Self) {
        if self.value < at.value {
            // VALUE IS LESS THAN, GO LEFT
            // Let's see if there is already a node to the left
            match at.left {
                // If there isn't, we just insert there and are done. We use Some to say
                // it's not null, and Box::new to put it behind a pointer for mem reasons.
                None => at.left = Some(Box::new(self)),
                // If there is, we recurse. This "ref mut" business just says that we're
                // using a mutable reference to a Node; un-needed in JS
                Some(ref mut left_node) => self.insert_at(left_node),
            }
        } else {
            // VALUE IS GREATER THAN OR EQUAL, GO RIGHT
            // Same procedure as before, but just to the right this time.
            match at.right {
                None => at.right = Some(Box::new(self)),
                Some(ref mut right_node) => self.insert_at(right_node),
            }
        }
    }

    // if at is the node you are looking for return it
    // otherwise if at.value is greater than the value you are looking for continue your search to the left
    // otherwise continue your search to the right
    // **NOTE** if at is ever null that means you've traversed the tree and never found your node return null in this case

    //     #find(val, at) {
    //       if (!at) return null;
    //       else if (val === at.value) return at;
    //       else if (val < at.value) return this.#find(val, at.left);
    //       else if (val > at.value) return this.#find(val, at.right);
    //     }

    pub fn find(&self, val: T) -> Option<&Self> {
        match val.cmp(&self.value) {
            Ordering::Equal => Some(&self),
            Ordering::Less => self.left.as_ref().and_then(|node| node.find(val)),
            Ordering::Greater => self.right.as_ref().and_then(|node| node.find(val)),
        }
    }

    /// Goes left if possible. Because this is for testing, errors if not possible.
    fn left(&self) -> &Self {
        self.left.as_ref().unwrap()
    }

    /// Goes right if possible. Because this is for testing, errors if not possible.
    fn right(&self) -> &Self {
        self.right.as_ref().unwrap()
    }

    /// Shows value of the current Node.
    fn value(&self) -> &T {
        &self.value
    }
}

//   module.exports = class BinarySearchTree {
//     root;
struct BST<T: Ord> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> BST<T> {
    //     constructor() {
    //       this.root = null;
    //     }

    // Self means that it returns the type in context: BST<T>.
    // Self { } constructs the type in context: BST<T>.
    pub fn new() -> Self {
        Self { root: None }
    }

    //     insert(value) {
    //       const node = new Node(value);
    //       if (!this.root) this.root = node;
    //       else return this.#insert(node, this.root);
    //       return node;
    //     }

    pub fn insert(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.root {
            None => self.root = Some(Box::new(new_node)),
            Some(ref mut existing_node) => new_node.insert_at(existing_node),
        }
    }

    //     find(val) {
    //       return this.#find(val, this.root);
    //     }
    //   };
    pub fn find(&self, val: T) -> Option<&Node<T>> {
        self.root.as_ref().and_then(|node| node.find(val))
    }

    /// Goes left if possible. Because this is for testing, errors if not possible.
    fn left(&self) -> &Node<T> {
        self.root.as_ref().map(|node| node.left()).unwrap()
    }

    /// Goes right if possible. Because this is for testing, errors if not possible.
    fn right(&self) -> &Node<T> {
        self.root.as_ref().map(|node| node.right()).unwrap()
    }

    /// Shows the top value of the current tree.
    /// Used for testing, so we just throw an error if there is no root.
    fn value(&self) -> &T {
        self.root.as_ref().map(|node| node.value()).unwrap()
    }
}

// TEST

// const BinarySearchTree = require("./BinarySearchTree");

// describe("BinarySearchTrea", () => {

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_insert_a_new_node() {
        let mut bst = BST::new();

        bst.insert(10);
        assert_eq!(*bst.value(), 10);

        bst.insert(5);
        assert_eq!(*bst.left().value(), 5);

        bst.insert(15);
        assert_eq!(*bst.right().value(), 15);

        bst.insert(20);
        assert_eq!(*bst.right().right().value(), 20);

        bst.insert(12);
        assert_eq!(*bst.right().left().value(), 12);

        bst.insert(7);
        assert_eq!(*bst.left().right().value(), 7);

        bst.insert(2);
        assert_eq!(*bst.left().left().value(), 2);
    }

    //   it("finds a node", () => {
    //     const bst = new BinarySearchTree();
    //     bst.insert(10);
    //     bst.insert(5);
    //     const twenty = bst.insert(20);
    //     bst.insert(25);
    //     const fifteen = bst.insert(15);
    //     const one = bst.insert(1);
    //     bst.insert(7);

    //     bst.postOrderPrint();

    //     expect(bst.find(20)).toBe(twenty);
    //     expect(bst.find(15)).toBe(fifteen);
    //     expect(bst.find(1)).toBe(one);
    //     expect(bst.find(100)).toBeNull();
    //   });
    // });
    fn finds_a_node() {
        let mut bst = BST::new();
        bst.insert(10);
        bst.insert(5);
        bst.insert(20);
        bst.insert(25);
        bst.insert(15);
        bst.insert(1);
        bst.insert(7);

        assert_eq!(*bst.find(20).unwrap().value(), 20);
        assert_eq!(*bst.find(15).unwrap().value(), 20);
        assert_eq!(*bst.find(1).unwrap().value(), 20);
        assert_eq!(bst.find(100), None);
    }
}
