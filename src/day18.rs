#[derive(Debug)]
pub enum Op {
    Mult,
    Plus,
}

#[derive(Debug)]
pub enum Node {
    Value(usize),
    SubNode(Box<Node>),
    Binary(Op, Box<Node>, Box<Node>),
}

#[derive(Debug, PartialEq)]
pub enum Part {
    One,
    Two,
}

/// parse a string into a node
pub fn parse(txt: &str, part: &Part) -> Option<Node> {
    let chars = txt.chars().filter(|c| *c != ' ').collect();
    parse_expression(&chars, 0, part).map(|(_, n)| n)
}

/// parse an expression into a node, keeping track of the position in the character vector
fn parse_expression(chars: &Vec<char>, pos: usize, part: &Part) -> Option<(usize, Node)> {
    match parse_start(chars, pos, part) {
        Some((new_pos, first)) => match parse_operator(chars, new_pos) {
            Some((new_pos2, op)) => {
                if let Some((new_pos3, second)) = parse_expression(chars, new_pos2, part) {
                    Some((new_pos3, combine(op, first, second, part)))
                } else {
                    None
                }
            }
            None => Some((new_pos, first)),
        },
        None => None,
    }
}

fn combine(op: Op, first: Node, second: Node, part: &Part) -> Node {
    match second {
        Node::Binary(op2, v21, v22) => {
            if precedence(&op, part) >= precedence(&op2, part) {
                Node::Binary(op2, Box::new(combine(op, first, *v21, part)), v22)
            } else {
                Node::Binary(op, Box::new(first), Box::new(Node::Binary(op2, v21, v22)))
            }
        }
        _ => Node::Binary(op, Box::new(first), Box::new(second)),
    }
}

/// a precedence rank for operators
fn precedence(op: &Op, part: &Part) -> usize {
    if part == &Part::One {
        1
    } else {
        match op {
            Op::Plus => 2,
            Op::Mult => 1,
        }
    }
}

/// try to parse from the start of an expression (either a parenthesis or a value)
fn parse_start(chars: &Vec<char>, pos: usize, part: &Part) -> Option<(usize, Node)> {
    match start_parenthesis(chars, pos) {
        Some(new_pos) => {
            let r = parse_expression(chars, new_pos, part);
            end_parenthesis(chars, r)
        }
        None => parse_value(chars, pos),
    }
}

/// match a starting parentheseis
fn start_parenthesis(chars: &Vec<char>, pos: usize) -> Option<usize> {
    if pos < chars.len() && chars[pos] == '(' {
        Some(pos + 1)
    } else {
        None
    }
}

/// match an end parenthesis, if successful will create a sub node contained the wrapped expression
fn end_parenthesis(chars: &Vec<char>, wrapped: Option<(usize, Node)>) -> Option<(usize, Node)> {
    match wrapped {
        Some((pos, node)) => {
            if pos < chars.len() && chars[pos] == ')' {
                Some((pos + 1, Node::SubNode(Box::new(node))))
            } else {
                None
            }
        }
        None => None,
    }
}

fn parse_value(chars: &Vec<char>, pos: usize) -> Option<(usize, Node)> {
    let mut new_pos = pos;
    while new_pos < chars.len() && (chars[new_pos] >= '0' && chars[new_pos] <= '9') {
        new_pos = new_pos + 1;
    }
    if new_pos > pos {
        if let Ok(v) = chars[pos..new_pos].iter().collect::<String>().parse() {
            Some((new_pos, Node::Value(v)))
        } else {
            None
        }
    } else {
        None
    }
}

/// parse an operator
fn parse_operator(chars: &Vec<char>, pos: usize) -> Option<(usize, Op)> {
    if pos < chars.len() {
        let ops_with_char = vec![('+', Op::Plus), ('*', Op::Mult)];
        for (ch, op) in ops_with_char {
            if chars[pos] == ch {
                return Some((pos + 1, op));
            }
        }
    }
    None
}

/// eval a string
pub fn eval(txt: &str, part: Part) -> usize {
    match parse(txt, &part) {
        Some(t) => eval_term(&t),

        None => panic!("Cannot parse {}", txt),
    }
}

/// eval a term, recursively
fn eval_term(t: &Node) -> usize {
    match t {
        Node::Value(v) => *v,
        Node::SubNode(t) => eval_term(t),
        Node::Binary(Op::Plus, t1, t2) => eval_term(t1) + eval_term(t2),
        Node::Binary(Op::Mult, t1, t2) => eval_term(t1) * eval_term(t2),
    }
}

pub fn part1(inp: String) {
    println!(
        "{}",
        inp.lines().map(|line| eval(line, Part::One)).sum::<usize>()
    );
}

pub fn part2(inp: String) {
    println!(
        "{}",
        inp.lines().map(|line| eval(line, Part::Two)).sum::<usize>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_part1() {
        assert_eq!(eval("1 + 2 * 3 + 4 * 5 + 6", Part::One), 71);
        assert_eq!(eval("2 * 3 + (4 * 5)", Part::One), 26);
        assert_eq!(eval("5 + (8 * 3 + 9 + 3 * 4 * 3)", Part::One), 437);
        assert_eq!(
            eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", Part::One),
            12240
        );
    }

    #[test]
    fn test_eval_part2() {
        assert_eq!(eval("1 + (2 * 3) + (4 * (5 + 6))", Part::Two), 51);
        assert_eq!(eval("2 * 3 + (4 * 5)", Part::Two), 46);
        assert_eq!(eval("5 + (8 * 3 + 9 + 3 * 4 * 3)", Part::Two), 1445);
        assert_eq!(
            eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", Part::Two),
            669060
        );
    }
}
