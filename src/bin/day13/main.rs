use nom::{
    branch::alt, bytes::complete::tag, multi::separated_list0, sequence::delimited, IResult, Parser,
};
use std::fmt::Display;

fn main() {
    let path = std::env::args().nth(1)
        .unwrap_or_else(|| "src/bin/day13/input.txt".into());
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
    std::iter::zip(1.., pairs)
        .filter_map(|(i, pair)| {
            let (left, right) = pair.split_once('\n').unwrap();
            let left = parse_packet(left);
            let right = parse_packet(right);
            if left < right {
                Some(i)
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut packets = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_packet)
        .collect::<Vec<_>>();
    packets.push(parse_packet("[[2]]"));
    packets.push(parse_packet("[[6]]"));
    packets.sort();
    let packet2 = packets
        .iter()
        .position(|packet| packet == &parse_packet("[[2]]"))
        .unwrap();
    let packet6 = packets
        .iter()
        .position(|packet| packet == &parse_packet("[[6]]"))
        .unwrap();
    (packet2 + 1) * (packet6 + 1)
}

#[derive(Debug, PartialEq, Eq)]
enum Node {
    List(Vec<Node>),
    Number(u32),
}

impl Node {
    fn list(num: &u32) -> Self {
        Node::List(vec![Node::Number(*num)])
    }
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Number(num) => {
                write!(f, "{}", num)
            }
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
        use Node::*;
        match (self, other) {
            (Number(num), Number(other_num)) => num.cmp(other_num),
            (List(list), List(other_list)) => list.cmp(other_list),
            (Number(num), List(_)) => Node::list(num).cmp(other),
            (List(_), Number(other_num)) => self.cmp(&Node::list(other_num))
        }
    }
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    alt((
        delimited(tag("["), separated_list0(tag(","), parse_node), tag("]"))
            .map(Node::List),
        nom::character::complete::u32.map(Node::Number),
    ))(input)
}

fn parse_packet(input: &str) -> Node {
    let (remaining, node) = parse_node(input).unwrap();
    assert_eq!(remaining, "");
    node
}

#[cfg(test)]
mod tests {
    use super::*;
    use Node::*;

    #[test]
    fn test_parse_list_of_five_numbers() {
        let line = "[1,2,3,4,5]";
        let node = parse_packet(line);
        assert_eq!(
            node,
            List(vec![Number(1), Number(2), Number(3), Number(4), Number(5)])
        );
        assert_eq!(node.to_string(), line);
    }
    
    #[test]
    fn test_parse_nested_list_of_numbers() {
        let line = "[[1],[2,3,4]]";
        let node = parse_packet(line);
        assert_eq!(
            node,
            List(vec![
                List(vec![Number(1)]),
                List(vec![Number(2), Number(3), Number(4)])
            ])
        );
        assert_eq!(node.to_string(), line);
    }
    
    #[test]
    fn test_parse_nested_empty_lists() {
        use Node::*;
        let line = "[[[]],[]]";
        let node = parse_packet(line);
        assert_eq!(
            node,
            List(vec![
                List(vec![
                    List(vec![])
                ]),
                List(vec![])
            ])
        );
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
        assert!(
            parse_packet("[1,[2,[3,[4,[5,6,7]]]],8,9]") > parse_packet("[1,[2,[3,[4,[5,6,0]]]],8,9]")
        );
    }
    
    const TEST_INPUT: &str = "\
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
    
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 13);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 140);
    }
    
}
