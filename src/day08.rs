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
    let mut start_1 = None;
    let mut start_2 = Vec::new();

    for line in lines {
        let (title, left_title, right_title) = parse_line(line);
        // TODO make this less ugly
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
        if start_1.is_none() && title == "AAA" {
            start_1 = Some(Rc::clone(&node));
        }
        if title.ends_with('A') {
            start_2.push(node);
        }
    }

    let instructions = instructions.chars().cycle();

    // Part 1:
    let mut result1 = None;
    let mut node = start_1.clone().unwrap();
    // Loop over instructions
    for (i, instruction) in instructions.clone().enumerate() {
        node = get_next(instruction, &node);

        if node.borrow().title == "ZZZ" {
            result1 = Some(i + 1);
            break;
        }
    }


    // Part 2:
    // Follow instructions until you find an end node
    // from looking at the example and real data, the end locations will loop
    // i.e. if the first end is at 12, the second will be at 24, then 36, etc.
    let intervals: Vec<_> = start_2.iter().map(|node| {
        let mut node = Rc::clone(node);
        let mut end_interval = 0;
        for (i, instruction) in instructions.clone().enumerate() {
            node = get_next(instruction, &node);
            if node.borrow().is_end {
                end_interval = i+1;
                break;
            }
        }
        end_interval
    }).collect();

    let result2 = find_lcm(&intervals).unwrap();

    (result1.unwrap_or(0) as i32 as isize, result2 as isize)
}

fn get_next(instruction: char, node: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    let node = Rc::clone(node);
    match instruction {
        'R' => { node.borrow().right.clone().unwrap() }
        'L' => { node.borrow().left.clone().unwrap() }
        _ => panic!("Parse error"),
    }
}


fn find_lcm(numbers: &[usize]) -> Option<usize> {
    if numbers.is_empty() {
        return None;
    }
    let mut lcm = numbers[0];
    for &num in &numbers[1..] {
        lcm = lcm * num / gcd(lcm, num);
    }
    Some(lcm)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
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
    is_end: bool,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(title: &str) -> Self {
        Self { title: title.to_string(), is_end: title.ends_with('Z'), left: None, right: None }
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

