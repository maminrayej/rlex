mod repr;

pub use repr::Element::{self, *};

fn top_operator_of(stack: &Vec<char>) -> Option<Element> {
    stack.last().and_then(|c| Element::operator_of(*c))
}

fn next_occur_of<F>(text: &Vec<char>, start: usize, pat: F) -> Option<usize>
where
    F: Fn(&char) -> bool,
{
    text[start..]
        .iter()
        .position(|c| pat(c))
        .map(|index| index + start)
}

fn ctou(c: char) -> Option<usize> {
    c.to_digit(10).map(|n| n as usize)
}

pub fn to_postfix(infix: &str) -> Vec<Element> {
    let mut stack = vec![];
    let mut postfix = vec![];

    let mut char_index = 0;
    let chars: Vec<char> = infix.chars().collect();

    while char_index < infix.len() {
        let c = chars[char_index];

        if let Some(operator) = Element::operator_of(c) {
            while let Some(top_operator) = top_operator_of(&stack) {
                if operator.priority() <= top_operator.priority() {
                    postfix.push(Element::operator_of(stack.pop().unwrap()).unwrap());
                } else {
                    break;
                }
            }

            stack.push(c);

            char_index += 1;
        } else {
            match c {
                '(' => {
                    stack.push('(');
                    char_index += 1;
                }
                ')' => {
                    while let Some(c) = stack.pop() {
                        if c != '(' {
                            postfix.push(Element::operator_of(c).unwrap());
                        } else {
                            break;
                        }
                    }

                    char_index += 1;
                }
                '"' => {
                    if let Some(index) = next_occur_of(&chars, char_index + 1, |c| *c == '"') {
                        postfix.push(Text(&infix[char_index + 1..index]));

                        char_index = index + 1;
                    } else {
                        panic!("Could not find closing '\"'")
                    }
                }
                c if c.is_alphanumeric() || c == '_' => {
                    let index = next_occur_of(&chars, char_index + 1, |c| {
                        !c.is_alphanumeric() && *c != '_'
                    })
                    .unwrap_or(chars.len());

                    if infix[char_index..index].len() == 1 {
                        if chars[char_index].is_numeric() {
                            postfix.push(Number(ctou(chars[char_index]).unwrap()))
                        } else if chars[char_index].is_ascii_alphabetic()
                            || chars[char_index] == '_'
                        {
                            postfix.push(Char(chars[char_index]))
                        }
                    } else {
                        if &infix[char_index..index] == "eps" {
                            postfix.push(Eps)
                        } else {
                            postfix.push(NameOrText(&infix[char_index..index]));
                        }
                    }

                    char_index = index;
                }
                ' ' | '\t' | '\n' => char_index += 1,
                c => panic!("Unsupported character: {}", c),
            }
        }
    }

    while let Some(c) = stack.pop() {
        let op = Element::operator_of(c).expect("Expected operator but found parenthesis");

        postfix.push(op);
    }

    postfix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        let postfix = to_postfix(r#""#);

        assert!(postfix.is_empty());
    }

    #[test]
    fn one_text() {
        let postfix = to_postfix(r#" "if" "#);

        assert_eq!(postfix, vec![Text("if")]);
    }

    #[test]
    fn one_char() {
        let postfix = to_postfix(r#" a "#);

        assert_eq!(postfix, vec![Char('a')]);
    }

    #[test]
    fn one_number() {
        let postfix = to_postfix(r#" 1 "#);

        assert_eq!(postfix, vec![Number(1)]);
    }

    #[test]
    fn one_name_or_text() {
        let postfix = to_postfix(r#" if "#);

        assert_eq!(postfix, vec![NameOrText("if")]);
    }

    #[test]
    fn or() {
        let postfix = to_postfix(r#" if | else | while | for "#);

        assert_eq!(
            postfix,
            vec![
                NameOrText("if"),
                NameOrText("else"),
                Or,
                NameOrText("while"),
                Or,
                NameOrText("for"),
                Or,
            ]
        )
    }

    #[test]
    fn concat() {
        let postfix = to_postfix(r#" "r"."l"."e"."x" "#);

        assert_eq!(
            postfix,
            vec![
                Text("r"),
                Text("l"),
                Concat,
                Text("e"),
                Concat,
                Text("x"),
                Concat,
            ]
        )
    }

    #[test]
    fn star() {
        let postfix = to_postfix(r#" a* "#);

        assert_eq!(postfix, vec![Char('a'), Star]);
    }

    #[test]
    fn question() {
        let postfix = to_postfix(r#" a? "#);

        assert_eq!(postfix, vec![Char('a'), Question])
    }

    #[test]
    fn plus() {
        let postfix = to_postfix(r#" a+ "#);

        assert_eq!(postfix, vec![Char('a'), Plus]);
    }

    #[test]
    fn dash() {
        let postfix = to_postfix(r#" a-z "#);

        assert_eq!(postfix, vec![Char('a'), Char('z'), Dash])
    }

    #[test]
    fn parenthesis() {
        let postfix = to_postfix(r#" a . (b | c) . d "#);

        assert_eq!(
            postfix,
            vec![
                Char('a'),
                Char('b'),
                Char('c'),
                Or,
                Concat,
                Char('d'),
                Concat
            ]
        )
    }

    #[test]
    fn identifier_regex() {
        let postfix = to_postfix(r#" (a-z)+.(a-z | 0-9 | _ )* "#);

        assert_eq!(
            postfix,
            vec![
                Char('a'),
                Char('z'),
                Dash,
                Plus,
                Char('a'),
                Char('z'),
                Dash,
                Number(0),
                Number(9),
                Dash,
                Or,
                Char('_'),
                Or,
                Star,
                Concat
            ]
        )
    }

    #[test]
    fn floating_point_number_regex() {
        let postfix = to_postfix(r#" 0-9+.".".0-9+ "#);

        assert_eq!(
            postfix,
            vec![
                Number(0),
                Number(9),
                Dash,
                Plus,
                Text("."),
                Concat,
                Number(0),
                Number(9),
                Dash,
                Plus,
                Concat,
            ]
        )
    }

    #[test]
    fn eps() {
        let postfix = to_postfix(r#" a . (b | eps) . c "#);

        assert_eq!(
            postfix,
            vec![Char('a'), Char('b'), Eps, Or, Concat, Char('c'), Concat]
        )
    }
}
