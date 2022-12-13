use std::fmt::Display;
use std::iter::Peekable;
use std::str::Chars;

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day13/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

}

#[derive(Debug)]
enum Node {
    List(Vec<Box<Node>>),
    Number(u32)
}

impl Node {
    fn is_list(&self) -> bool {
        match self {
            Node::List(_) => true,
            Node::Number(_) => false
        }
    }

    fn is_number(&self) -> bool {
        match self {
            Node::List(_) => false,
            Node::Number(_) => true
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Number(num) => {
                write!(f, "{}", num)
            },
            Node::List(list) => {
                let mut needs_comma = false;
                write!(f, "[")?;
                for node in list {
                    if needs_comma {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", node)?;
                    needs_comma = true;
                }
                write!(f, "]")
            }
        }
    }
}

fn parse_node(s: &mut Peekable<Chars>) -> Node {
    if s.peek() == Some(&'[') {
        // Parse a list
        let mut list = Vec::new();
        s.next();       // Consume the open square bracket
        while s.peek() != Some(&']') {
            // Parse a Node
            let node = parse_node(s);
            list.push(Box::new(node));

            // Consume a comma, if any
            if s.peek() == Some(&',') {
                s.next();
            }
        }
        // Consume closing square backet
        assert_eq!(s.next(), Some(']'));
        Node::List(list)
    } else {
        // Pase a number
        let mut num = 0;
        while let Some(c) = s.peek() {
            if c.is_digit(10) {
                num = num * 10 + s.next().unwrap().to_digit(10).unwrap();
            } else {
                break;
            }
        }
        Node::Number(num)
    }
}

fn parse_packet(line: &str) -> Node {
    let mut chars = line.chars().peekable();
    let node = parse_node(&mut chars);
    assert!(node.is_list());
    assert_eq!(chars.peek(), None);
    node
}

#[test]
fn test_parse_list_of_five_numbers() {
    let line = "[1,2,3,4,5]";
    let node = parse_packet(line);
    match &node {
        Node::List(list) => {
            assert_eq!(list.len(), 5);
        },
        Node::Number(_) => panic!("expected list")
    }
    assert_eq!(node.to_string(), line);
}

#[test]
fn test_parse_nested_list_of_numbers() {
    let line = "[[1],[2,3,4]]";
    let node = parse_packet(line);
    match &node {
        Node::List(list) => {
            assert_eq!(list.len(), 2);
        },
        Node::Number(_) => panic!("expected list")
    }
    assert_eq!(node.to_string(), line);
}

#[test]
fn test_parse_nested_empty_lists() {
    let line = "[[[]],[]]";
    let node = parse_packet(line);
    assert_eq!(node.to_string(), line);
}
