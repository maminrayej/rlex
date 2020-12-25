use std::collections::HashMap;

pub fn utoc(digit: usize) -> char {
    match digit {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        _ => panic!("Number is not a digit: {}", digit),
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum Lable {
    Char(char),
    Eps,
}

impl From<char> for Lable {
    fn from(c: char) -> Self {
        Lable::Char(c)
    }
}

pub struct Nfa {
    start_id: usize,
    finish_id: usize,

    delta: HashMap<(usize, Lable), usize>,

    sub_nfa1: Option<Box<Nfa>>,
    sub_nfa2: Option<Box<Nfa>>,
}

impl Nfa {
    pub fn of_text(text: &str) -> Self {
        let start_id = 0;
        let finish_id = text.len() + 1;
        let mut delta = HashMap::new();

        let chars: Vec<char> = text.chars().collect();

        for id in start_id..finish_id {
            delta.insert((id, chars[id].into()), id + 1);
        }

        Nfa {
            start_id,
            finish_id,
            delta,
            sub_nfa1: None,
            sub_nfa2: None,
        }
    }

    pub fn of_digit(digit: usize) -> Self {
        let start_id = 0;
        let finish_id = 1;
        let mut delta = HashMap::new();

        delta.insert((start_id, utoc(digit).into()), finish_id);

        Nfa {
            start_id,
            finish_id,
            delta,
            sub_nfa1: None,
            sub_nfa2: None,
        }
    }

    pub fn of_char(c: char) -> Self {
        let start_id = 0;
        let finish_id = 1;
        let mut delta = HashMap::new();

        delta.insert((start_id, c.into()), finish_id);

        Nfa {
            start_id,
            finish_id,
            delta,
            sub_nfa1: None,
            sub_nfa2: None,
        }
    }

    pub fn from_eps() -> Self {
        let start_id = 0;
        let finish_id = 1;

        let mut delta = HashMap::new();

        delta.insert((start_id, Lable::Eps), finish_id);

        Nfa {
            start_id,
            finish_id,
            delta,
            sub_nfa1: None,
            sub_nfa2: None,
        }
    }

    pub fn from_cdash(c1: char, c2: char) -> Self {
        let start_id = 0;
        let finish_id = 1;

        let mut delta = HashMap::new();

        for c in c1..c2 {
            delta.insert((start_id, c.into()), finish_id);
        }

        Nfa {
            start_id,
            finish_id,
            delta,
            sub_nfa1: None,
            sub_nfa2: None,
        }
    }

    pub fn from_ndash(n1: usize, n2: usize) -> Self {
        let c1 = utoc(n1);
        let c2 = utoc(n2);

        Nfa::from_cdash(c1, c2)
    }

    pub fn insert_transition(&mut self, src_id: usize, lable: Lable, dst_id: usize) {
        self.delta.insert((src_id, lable), dst_id);
    }

    pub fn sid(&self) -> usize {
        self.start_id
    }

    pub fn fid(&self) -> usize {
        self.finish_id
    }
}
