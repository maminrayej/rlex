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
}

impl Nfa {
    pub fn of_text(text: &str, offset: usize) -> Self {
        let start_id = offset;
        let finish_id = offset + text.len() + 1;
        let mut delta = HashMap::new();

        let chars: Vec<char> = text.chars().collect();

        for id in start_id..finish_id {
            delta.insert((id, chars[id].into()), id + 1);
        }

        Nfa {
            start_id,
            finish_id,
            delta,
        }
    }

    pub fn of_digit(digit: usize, offset: usize) -> Self {
        let start_id = offset;
        let finish_id = offset + 1;
        let mut delta = HashMap::new();

        delta.insert((start_id, utoc(digit).into()), finish_id);

        Nfa {
            start_id,
            finish_id,
            delta,
        }
    }

    pub fn of_char(c: char, offset: usize) -> Self {
        let start_id = offset;
        let finish_id = offset + 1;
        let mut delta = HashMap::new();

        delta.insert((start_id, c.into()), finish_id);

        Nfa {
            start_id,
            finish_id,
            delta,
        }
    }

    pub fn from_eps(offset: usize) -> Self {
        let start_id = offset;
        let finish_id = offset + 1;

        let mut delta = HashMap::new();

        delta.insert((start_id, Lable::Eps), finish_id);

        Nfa {
            start_id,
            finish_id,
            delta,
        }
    }

    pub fn from_cdash(c1: char, c2: char, offset: usize) -> Self {
        let start_id = offset;
        let finish_id = offset + 1;

        let mut delta = HashMap::new();

        for c in c1..c2 {
            delta.insert((start_id, c.into()), finish_id);
        }

        Nfa {
            start_id,
            finish_id,
            delta,
        }
    }

    pub fn from_ndash(n1: usize, n2: usize, offset: usize) -> Self {
        let c1 = utoc(n1);
        let c2 = utoc(n2);

        Nfa::from_cdash(c1, c2, offset)
    }

    pub fn from_or(nfa1: Nfa, nfa2: Nfa, offset: usize) -> Self {
        let start_id = offset;
        let finish_id = offset + 1;

        let nfa1_sid = nfa1.sid();
        let nfa1_fid = nfa1.fid();
        let mut delta = nfa1.get_delta();

        let nfa2_sid = nfa2.sid();
        let nfa2_fid = nfa2.fid();
        let delta2 = nfa2.get_delta();

        delta.extend(delta2);

        delta.insert((start_id, Lable::Eps), nfa1_sid);
        delta.insert((start_id, Lable::Eps), nfa2_sid);
        delta.insert((nfa1_fid, Lable::Eps), finish_id);
        delta.insert((nfa2_fid, Lable::Eps), finish_id);

        Nfa {
            start_id,
            finish_id,
            delta,
        }
    }

    pub fn from_concat(nfa1: Nfa, nfa2: Nfa, _: usize) -> Self {
        let start_id = nfa1.sid();
        let finish_id = nfa2.fid();

        let nfa1_fid = nfa1.fid();
        let nfa2_sid = nfa2.sid();

        let mut delta = nfa1.get_delta();
        let delta2 = nfa2.get_delta();

        delta.extend(delta2);

        delta.insert((nfa1_fid, Lable::Eps), nfa2_sid);

        Nfa {
            start_id,
            finish_id,
            delta,
        }
    }

    pub fn from_star(nfa: Nfa, offset: usize) -> Self {
        let start_id = offset;
        let finish_id = offset + 1;

        let nfa_sid = nfa.sid();
        let nfa_fid = nfa.fid();

        let mut delta = nfa.get_delta();

        delta.insert((nfa_fid, Lable::Eps), nfa_sid);
        delta.insert((start_id, Lable::Eps), nfa_sid);
        delta.insert((start_id, Lable::Eps), finish_id);
        delta.insert((nfa_fid, Lable::Eps), finish_id);

        Nfa {
            start_id,
            finish_id,
            delta,
        }
    }

    pub fn from_plus(nfa: Nfa, offset: usize) -> Self {
        let start_id = offset;
        let finish_id = offset + 1;

        let nfa_sid = nfa.sid();
        let nfa_fid = nfa.fid();

        let mut delta = nfa.get_delta();

        delta.insert((nfa_fid, Lable::Eps), nfa_sid);
        delta.insert((start_id, Lable::Eps), nfa_sid);
        delta.insert((nfa_fid, Lable::Eps), finish_id);

        Nfa {
            start_id,
            finish_id,
            delta,
        }
    }

    pub fn from_question(nfa: Nfa, offset: usize) -> Self {
        let start_id = offset;
        let finish_id = offset + 1;

        let nfa_sid = nfa.sid();
        let nfa_fid = nfa.fid();

        let mut delta = nfa.get_delta();

        delta.insert((start_id, Lable::Eps), nfa_sid);
        delta.insert((start_id, Lable::Eps), finish_id);
        delta.insert((nfa_fid, Lable::Eps), finish_id);

        Nfa {
            start_id,
            finish_id,
            delta,
        }
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

    pub fn get_delta(self) -> HashMap<(usize, Lable), usize> {
        self.delta
    }

    pub fn delta(&self, (state_id, lable): (usize, Lable)) -> Option<usize> {
        self.delta.get(&(state_id, lable)).copied()
    }
}
