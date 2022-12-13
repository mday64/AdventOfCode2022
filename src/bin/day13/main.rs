use std::fmt::Display;
use std::iter::Peekable;
use std::str::Chars;

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day13/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    let result1 = part1(&input);
    println!("Part 1: {}", result1);
    assert_eq!(result1, 5252);

    let result2 = part2(&input);
    println!("Part 2: {}", result2);
    assert_eq!(result2, 20592);
}

fn part1(input: &str) -> usize {
    let pairs = input.trim_end().split("\n\n");
    std::iter::zip(1.., pairs).filter_map(|(i,pair)| {
        let (left, right) = pair.split_once("\n").unwrap();
        let left = parse_packet(left);
        let right = parse_packet(right);
        if left < right { Some(i) } else { None }
    }).sum()
}

fn part2(input: &str) -> usize {
    let mut packets = input.lines()
        .filter(|line| line.len() > 0)
        .map(|line| parse_packet(line))
        .collect::<Vec<_>>();
    packets.push(parse_packet("[[2]]"));
    packets.push(parse_packet("[[6]]"));
    packets.sort();
    let packet2 = packets.iter().position(|packet| packet == &parse_packet("[[2]]")).unwrap();
    let packet6 = packets.iter().position(|packet| packet == &parse_packet("[[6]]")).unwrap();
    (packet2 + 1) * (packet6 + 1)
}

#[derive(Debug, PartialEq, Eq)]
enum Node {
    List(Vec<Node>),
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

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Node::Number(num) => {
                match other {
                    Node::Number(other_num) => {
                        num.cmp(other_num)
                    },
                    Node::List(_) => {
                        Node::List(vec![Node::Number(*num)]).cmp(other)
                    }
                }
            },
            Node::List(list) => {
                match other {
                    Node::List(other_list) => {
                        list.cmp(other_list)
                    },
                    Node::Number(other_num) => {
                        self.cmp(&Node::List(vec![Node::Number(*other_num)]))
                    }
                }
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
            list.push(node);

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
            assert_eq!(list[0], Node::Number(1));
            assert_eq!(list[1], Node::Number(2));
            assert_eq!(list[2], Node::Number(3));
            assert_eq!(list[3], Node::Number(4));
            assert_eq!(list[4], Node::Number(5));
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

#[test]
fn test_cmp_numbers() {
    assert!(Node::Number(3) < Node::Number(4));
    assert!(Node::Number(4) == Node::Number(4));
    assert!(Node::Number(5) > Node::Number(4));
}

#[test]
fn test_cmp_lists() {
    assert!(parse_packet("[1,1,3,1,1]") < parse_packet("[1,1,5,1,1]"));
    assert!(parse_packet("[[1],[2,3,4]]") < parse_packet("[[1],4]"));
    assert!(parse_packet("[9]") > parse_packet("[[8,7,6]]"));
    assert!(parse_packet("[[4,4],4,4]") < parse_packet("[[4,4],4,4,4]"));
    assert!(parse_packet("[7,7,7,7]") > parse_packet("[7,7,7]"));
    assert!(parse_packet("[]") < parse_packet("[3]"));
    assert!(parse_packet("[[[]]]") > parse_packet("[[]]"));
    assert!(parse_packet("[1,[2,[3,[4,[5,6,7]]]],8,9]") > parse_packet("[1,[2,[3,[4,[5,6,0]]]],8,9]"));
}

#[test]
fn test_part1() {
    let input = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
    assert_eq!(part1(input), 13);
}


#[test]
fn test_part2() {
    let input = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
    assert_eq!(part2(input), 140);
}
