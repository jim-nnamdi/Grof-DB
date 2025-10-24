
pub struct Node {
    val: i32,
    next: Option<Box<Node>>
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node { val, next: None }
    }
}

pub struct List {
    head: Option<Box<Node>>
}

impl List {
    pub fn new(&self, val: i32) -> List {
        let new_node = Box::new(Node::new(val));
        List { head:  Some(new_node)}
    }

    pub fn push_front(&mut self, val: i32) {
         /* create a new node */
         /* let the value of next for new node be*/
         /* the current value of self.head*/
         /* then make self.head the new node*/
        let mut node = Box::new(Node::new(val));
        node.next = self.head.take();
        self.head = Some(node);
    }

    pub fn pop_front(&mut self) -> Option<i32> {
        /* take the value out of head */
         /* iterate through it and get the internal node*/
         /* return the current value of the node*/
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }
}