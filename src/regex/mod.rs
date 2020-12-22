#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    Dash,
    Plus,
    Star,
    Or,
    Concat,
    Question,
}

impl Operator {
    pub fn num_of_args(&self) -> usize {
        match self {
            Operator::Or | Operator::Concat | Operator::Dash => 2,
            Operator::Plus | Operator::Question | Operator::Star => 1,
        }
    }

    fn of(value: char) -> Option<Operator> {
        match value {
            '-' => Some(Operator::Dash),
            '+' => Some(Operator::Plus),
            '*' => Some(Operator::Star),
            '|' => Some(Operator::Or),
            '.' => Some(Operator::Concat),
            '?' => Some(Operator::Question),
            _ => None,
        }
    }

    pub fn priority(&self) -> usize {
        match self {
            Operator::Dash => 3,
            Operator::Plus | Operator::Star | Operator::Question => 2,
            Operator::Concat => 1,
            Operator::Or => 0,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Operator::Dash => "-",
            Operator::Plus => "+",
            Operator::Star => "*",
            Operator::Or => "|",
            Operator::Concat => ".",
            Operator::Question => "?",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Operand<'a> {
    Text(&'a str),
    Number(usize),
    Char(char),
    NameOrText(&'a str),
}

#[derive(Clone, Copy, Debug)]
enum StackElement {
    Operator(Operator),
    LeftParen,
    RightParen,
}
#[derive(Debug, PartialEq)]
pub enum Element<'a> {
    Operand(Operand<'a>),
    Operator(Operator),
}

impl<'a> From<StackElement> for Element<'a> {
    fn from(se: StackElement) -> Self {
        match se {
            StackElement::Operator(op) => Element::Operator(op),
            _ => panic!("Can not build Element from non-operator variants of StackElement"),
        }
    }
}

impl StackElement {
    fn inner_op(&self) -> Option<&Operator> {
        match self {
            StackElement::Operator(operator) => Some(operator),
            StackElement::LeftParen | StackElement::RightParen => None,
        }
    }
}

fn top_operator_of(stack: &Vec<StackElement>) -> Option<Operator> {
    stack.last().and_then(|e| e.inner_op().copied())
}

fn next_occur_of<F>(text: &Vec<char>, start: usize, pat: F) -> Option<usize>
where
    F: Fn(&char) -> bool,
{
    text[start..]
        .iter()
        .position(|c| pat(c))
        .and_then(|index| Some(index + start))
}

fn ctou(c: char) -> Option<usize> {
    c.to_digit(10).map(|n| n as usize)
}

pub fn to_postfix(infix: &str) -> Vec<Element> {
    use crate::regex::Operand::{Char, NameOrText, Number, Text};
    use Element::Operand;

    let mut stack = Vec::<StackElement>::new();
    let mut postfix = Vec::<Element>::new();

    let mut char_index = 0;
    let chars: Vec<char> = infix.chars().collect();

    while char_index < infix.len() {
        let c_char = chars[char_index];

        if let Some(operator) = Operator::of(c_char) {
            println!("Found operator: {}", c_char);
            println!("Stack: {:?}", stack);

            while let Some(top_operator) = top_operator_of(&stack) {
                println!("Found top operator: {:?}", top_operator);

                if operator.priority() <= top_operator.priority() {
                    postfix.push(stack.pop().unwrap().into())
                } else {
                    break;
                }
            }

            stack.push(StackElement::Operator(operator));

            char_index += 1;
        } else {
            match c_char {
                '(' => {
                    stack.push(StackElement::LeftParen);
                    char_index += 1;
                }
                ')' => {
                    println!("Found )");
                    println!("Stack: {:?}", stack);
                    while let Some(element) = stack.pop() {
                        if !matches!(element, StackElement::LeftParen) {
                            postfix.push(element.into());
                        } else {
                            break;
                        }
                    }

                    char_index += 1;
                }
                '"' => {
                    println!("Found qoute at: {}", char_index);
                    if let Some(index) = next_occur_of(&chars, char_index + 1, |c| *c == '"') {
                        println!("Closing qoute will be at: {}", index);
                        postfix.push(Operand(Text(&infix[char_index + 1..index])));

                        char_index = index + 1;
                        println!("Now char_index at: {}", char_index);
                    } else {
                        panic!("Could not find closing '\"'")
                    }
                }
                c if c.is_alphanumeric() || c == '_' => {
                    println!("Found alphanumeric: {} at {}", c, char_index);
                    let index = next_occur_of(&chars, char_index + 1, |c| {
                        !c.is_alphanumeric() && *c != '_'
                    })
                    .unwrap_or(chars.len());

                    if infix[char_index..index].len() == 1 {
                        if chars[char_index].is_numeric() {
                            postfix.push(Operand(Number(ctou(chars[char_index]).unwrap())))
                        } else if chars[char_index].is_ascii_alphabetic() || chars[char_index] == '_' {
                            postfix.push(Operand(Char(chars[char_index])))
                        } else {
                            panic!("Unsupported character: {}", chars[char_index]);
                        }
                    } else {
                        postfix.push(Operand(NameOrText(&infix[char_index..index])));
                    }

                    char_index = index;
                    println!("Now char_index at: {}", char_index);
                }
                ' ' | '\t' | '\n' => char_index += 1,
                c => panic!("Unsupported character: {}", c),
            }
        }
    }

    while let Some(operator) = stack.pop() {
        let op = operator.inner_op().copied().unwrap();

        let op = Element::Operator(op);

        postfix.push(op);
    }

    postfix
}

#[cfg(test)]
mod tests {
    use super::{to_postfix, Element::*, Operand::*, Operator::*};

    #[test]
    fn empty_string() {
        let postfix = to_postfix(r#""#);

        assert!(postfix.is_empty());
    }

    #[test]
    fn one_text() {
        let postfix = to_postfix(r#" "if" "#);

        assert_eq!(postfix, vec![Operand(Text("if"))]);
    }

    #[test]
    fn one_char() {
        let postfix = to_postfix(r#" a "#);

        assert_eq!(postfix, vec![Operand(Char('a'))]);
    }

    #[test]
    fn one_number() {
        let postfix = to_postfix(r#" 1 "#);

        assert_eq!(postfix, vec![Operand(Number(1))]);
    }

    #[test]
    fn one_name_or_text() {
        let postfix = to_postfix(r#" if "#);

        assert_eq!(postfix, vec![Operand(NameOrText("if"))]);
    }

    #[test]
    fn or() {
        let postfix = to_postfix(r#" if | else | while | for "#);

        assert_eq!(
            postfix,
            vec![
                Operand(NameOrText("if")),
                Operand(NameOrText("else")),
                Operator(Or),
                Operand(NameOrText("while")),
                Operator(Or),
                Operand(NameOrText("for")),
                Operator(Or),
            ]
        )
    }

    #[test]
    fn concat() {
        let postfix = to_postfix(r#" "r"."l"."e"."x" "#);

        assert_eq!(
            postfix,
            vec![
                Operand(Text("r")),
                Operand(Text("l")),
                Operator(Concat),
                Operand(Text("e")),
                Operator(Concat),
                Operand(Text("x")),
                Operator(Concat),
            ]
        )
    }

    #[test]
    fn star() {
        let postfix = to_postfix(r#" a* "#);

        assert_eq!(postfix, vec![Operand(Char('a')), Operator(Star)]);
    }

    #[test]
    fn question() {
        let postfix = to_postfix(r#" a? "#);

        assert_eq!(postfix, vec![Operand(Char('a')), Operator(Question)])
    }

    #[test]
    fn plus() {
        let postfix = to_postfix(r#" a+ "#);

        assert_eq!(postfix, vec![Operand(Char('a')), Operator(Plus)]);
    }

    #[test]
    fn dash() {
        let postfix = to_postfix(r#" a-z "#);

        assert_eq!(
            postfix,
            vec![Operand(Char('a')), Operand(Char('z')), Operator(Dash)]
        )
    }

    #[test]
    fn parenthesis() {
        let postfix = to_postfix(r#" a . (b | c) . d "#);

        assert_eq!(
            postfix,
            vec![
                Operand(Char('a')),
                Operand(Char('b')),
                Operand(Char('c')),
                Operator(Or),
                Operator(Concat),
                Operand(Char('d')),
                Operator(Concat)
            ]
        )
    }

    #[test]
    fn identifier_regex() {
        let postfix = to_postfix(r#" (a-z)+.(a-z | 0-9 | _ )* "#);

        assert_eq!(
            postfix,
            vec![
                Operand(Char('a')),
                Operand(Char('z')),
                Operator(Dash),
                Operator(Plus),
                Operand(Char('a')),
                Operand(Char('z')),
                Operator(Dash),
                Operand(Number(0)),
                Operand(Number(9)),
                Operator(Dash),
                Operator(Or),
                Operand(Char('_')),
                Operator(Or),
                Operator(Star),
                Operator(Concat)
            ]
        )
    }

    #[test]
    fn floating_point_number_regex() {
        let postfix = to_postfix(r#" 0-9+.".".0-9+ "#);

        assert_eq!(
            postfix,
            vec![
                Operand(Number(0)),
                Operand(Number(9)),
                Operator(Dash),
                Operator(Plus),
                Operand(Text(".")),
                Operator(Concat),
                Operand(Number(0)),
                Operand(Number(9)),
                Operator(Dash),
                Operator(Plus),
                Operator(Concat),
            ]
        )
    }
}
