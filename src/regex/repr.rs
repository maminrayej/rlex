#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Element<'a> {
    // Operators
    Dash,
    Plus,
    Star,
    Or,
    Concat,
    Question,

    // Operands
    Text(&'a str),
    Number(usize),
    Char(char),
    NameOrText(&'a str),
    Eps,
}

impl<'a> Element<'a> {
    pub fn operator_of(c: char) -> Option<Element<'a>> {
        match c {
            '-' => Some(Element::Dash),
            '+' => Some(Element::Plus),
            '*' => Some(Element::Star),
            '|' => Some(Element::Or),
            '.' => Some(Element::Concat),
            '?' => Some(Element::Question),
            _ => None,
        }
    }

    pub fn is_operator(&self) -> bool {
        match self {
            Element::Dash
            | Element::Plus
            | Element::Star
            | Element::Or
            | Element::Concat
            | Element::Question => true,

            Element::Text(_)
            | Element::Number(_)
            | Element::Char(_)
            | Element::NameOrText(_)
            | Element::Eps => false,
        }
    }

    pub fn is_operand(&self) -> bool {
        !self.is_operator()
    }

    pub fn priority(&self) -> Option<usize> {
        match self {
            Element::Dash => Some(3),
            Element::Plus | Element::Star | Element::Question => Some(2),
            Element::Concat => Some(1),
            Element::Or => Some(0),

            Element::Text(_)
            | Element::Number(_)
            | Element::Char(_)
            | Element::NameOrText(_)
            | Element::Eps => None,
        }
    }
}
