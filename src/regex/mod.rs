use std::iter;

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    Dash,
    Plus,
    Star,
    Or,
    Concatenation,
    Question,
}

impl Operator {
    pub fn num_of_args(&self) -> usize {
        match self {
            Operator::Or | Operator::Concatenation | Operator::Dash => 2,
            Operator::Plus | Operator::Question | Operator::Star => 1,
        }
    }

    fn of(value: char) -> Option<Operator> {
        match value {
            '-' => Some(Operator::Dash),
            '+' => Some(Operator::Plus),
            '*' => Some(Operator::Star),
            '|' => Some(Operator::Or),
            '.' => Some(Operator::Concatenation),
            '?' => Some(Operator::Question),
            _ => None,
        }
    }

    pub fn priority(&self) -> usize {
        match self {
            Operator::Plus | Operator::Star | Operator::Question => 3,
            Operator::Dash => 2,
            Operator::Concatenation => 1,
            Operator::Or => 0,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Operator::Dash => "-",
            Operator::Plus => "+",
            Operator::Star => "*",
            Operator::Or => "|",
            Operator::Concatenation => ".",
            Operator::Question => "?",
        }
    }
}

#[derive(Debug)]
pub enum Operand<'a> {
    Text(&'a str),
    Number(usize),
    Char(char),
    Name(&'a str),
}

#[derive(Clone, Copy)]
enum StackElement {
    Operator(Operator),
    LeftParen,
    RightParen,
}
#[derive(Debug)]
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
    stack
        .iter()
        .find(|e| matches!(e, StackElement::Operator(_)))
        .and_then(|e| e.inner_op())
        .copied()
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

pub fn to_postfix(infix: &str) -> Vec<Element> {
    let name_chars: Vec<char> = iter::once('_').chain('a'..'z').chain('A'..'Z').collect();

    let mut stack = Vec::<StackElement>::new();
    let mut postfix = Vec::<Element>::new();

    let mut char_index = 0;
    let chars: Vec<char> = infix.chars().collect();

    while char_index < infix.len() {
        let c_char = chars[char_index];

        if let Some(operator) = Operator::of(c_char) {
            while let Some(top_operator) = top_operator_of(&stack) {
                if operator.priority() >= top_operator.priority() {
                    postfix.push(stack.pop().unwrap().into())
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
                    while let Some(element) = stack.pop() {
                        if !matches!(element, StackElement::LeftParen) {
                            postfix.push(element.into());
                        } else {
                            break;
                        }
                    }
                }
                '"' => {
                    println!("Found qoute at: {}", char_index);
                    if let Some(index) = next_occur_of(&chars, char_index + 1, |c| *c == '"') {
                        println!("Closing qoute will be at: {}", index);
                        postfix.push(Element::Operand(Operand::Text(
                            &infix[char_index + 1..index],
                        )));

                        char_index = index + 1;
                        println!("Now char_index at: {}", char_index);
                    } else {
                        panic!("Could not find closing '\"'")
                    }
                }
                c if name_chars.contains(&c) => {
                    let index = next_occur_of(&chars, char_index + 1, |c| !name_chars.contains(c))
                        .unwrap_or(chars.len());

                    postfix.push(Element::Operand(Operand::Name(&infix[char_index..index])));

                    char_index = index + 1;
                }
                ' ' | '\t' | '\n' => char_index += 1,
                c => panic!(format!("Unsupported character: {}", c)),
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
    use super::*;

    #[test]
    fn test_order() {
        let postfix = to_postfix("\"a\" . \"b\"");

        println!("{:?}", postfix);
    }
}
