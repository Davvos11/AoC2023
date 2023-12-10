use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

pub fn day08(input: &str) -> (isize, isize) {
    let mut lines = input.lines();
    // Get LR instructions
    let instructions = lines.next().unwrap();
    // Skip empty line
    lines.next();

    // Parse nodes
    let mut nodes: HashMap<&str, Rc<RefCell<Node>>> = HashMap::new();
    let mut start = None;

    for line in lines {
        let (title, left_title, right_title) = parse_line(line);
        // Get left and right nodes if already constructed, otherwise create new
        let left = Rc::clone(nodes.entry(left_title)
            .or_insert(Rc::new(RefCell::new(Node::new(left_title)))));
        let right = Rc::clone(nodes.entry(right_title)
            .or_insert(Rc::new(RefCell::new(Node::new(right_title)))));
        // Get current node if already constructed, otherwise create new
        let node = Rc::clone(nodes.entry(title)
            .or_insert(Rc::new(RefCell::new(Node::new(title)))));
        node.borrow_mut().set_children(&left, &right);
        // Check if this is our start
        if start.is_none() && title == "AAA" {
            start = Some(node);
        }
    }

    let mut node = start.unwrap();
    let mut result1 = 0;

    // Loop over instructions
    for (i, instruction) in instructions.chars().cycle().enumerate() {
        let current = Rc::clone(&node);
        // let mut next = None;
        match instruction {
            'R' => { node = current.borrow().right.clone().unwrap() }
            'L' => { node = current.borrow().left.clone().unwrap() }
            _ => panic!("Parse error"),
        };
        if node.borrow().title == "ZZZ" {
            result1 = i + 1;
            break
        }
    }


    let result2 = 0;

    (result1 as i32, result2)
}

fn parse_line(line: &str) -> (&str, &str, &str) {
    let mut split = line.split_ascii_whitespace();
    let title = split.next().unwrap();
    split.next(); // ignore '='
    let left = &split.next().unwrap()[1..=3];
    let right = &split.next().unwrap()[0..=2];

    (title, left, right)
}

struct Node {
    title: String,
    // TODO &str?
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(title: &str) -> Self {
        Self { title: title.to_string(), left: None, right: None }
    }
    pub fn set_children(&mut self, left: &Rc<RefCell<Node>>, right: &Rc<RefCell<Node>>) {
        self.left = Some(Rc::clone(left));
        self.right = Some(Rc::clone(right));
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}, {:?}", self.title,
               self.left.clone().map(|x| Rc::clone(&x).borrow().title.clone()),
               self.right.clone().map(|x| Rc::clone(&x).borrow().title.clone()),
        )
    }
}

