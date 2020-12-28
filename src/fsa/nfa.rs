use std::collections::{
    hash_map::Entry::{Occupied, Vacant},
    HashMap,
};

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

    delta: HashMap<(usize, Lable), Vec<usize>>,
}

impl Nfa {
    pub fn of_text(text: &str, offset: usize) -> Self {
        let start_id = offset;
        let finish_id = offset + text.len();
        let delta = HashMap::new();

        let chars: Vec<char> = text.chars().collect();

        let mut nfa = Nfa {
            start_id,
            finish_id,
            delta,
        };

        for index in 0..chars.len() {
            nfa.insert_transition(index + offset, chars[index].into(), index + offset + 1);
        }

        nfa
    }

    pub fn of_digit(digit: usize, offset: usize) -> Self {
        let start_id = offset;
        let finish_id = offset + 1;
        let delta = HashMap::new();

        let mut nfa = Nfa {
            start_id,
            finish_id,
            delta,
        };

        nfa.insert_transition(start_id, utoc(digit).into(), finish_id);

        nfa
    }

    pub fn of_char(c: char, offset: usize) -> Self {
        let start_id = offset;
        let finish_id = offset + 1;
        let delta = HashMap::new();

        let mut nfa = Nfa {
            start_id,
            finish_id,
            delta,
        };

        nfa.insert_transition(start_id, c.into(), finish_id);

        nfa
    }

    pub fn of_eps(offset: usize) -> Self {
        let start_id = offset;
        let finish_id = offset + 1;

        let delta = HashMap::new();

        let mut nfa = Nfa {
            start_id,
            finish_id,
            delta,
        };

        nfa.insert_transition(start_id, Lable::Eps, finish_id);

        nfa
    }

    pub fn of_cdash(c1: char, c2: char, offset: usize) -> Self {
        let start_id = offset;
        let finish_id = offset + 1;

        let delta = HashMap::new();

        let mut nfa = Nfa {
            start_id,
            finish_id,
            delta,
        };

        for c in c1..=c2 {
            nfa.insert_transition(start_id, c.into(), finish_id);
        }

        nfa
    }

    pub fn of_ndash(n1: usize, n2: usize, offset: usize) -> Self {
        let c1 = utoc(n1);
        let c2 = utoc(n2);

        Nfa::of_cdash(c1, c2, offset)
    }

    pub fn of_or(nfa1: Nfa, nfa2: Nfa, offset: usize) -> Self {
        let start_id = offset;
        let finish_id = offset + 1;

        let nfa1_sid = nfa1.sid();
        let nfa1_fid = nfa1.fid();
        let mut delta = nfa1.get_delta();

        let nfa2_sid = nfa2.sid();
        let nfa2_fid = nfa2.fid();
        let delta2 = nfa2.get_delta();

        delta.extend(delta2);

        let mut nfa = Nfa {
            start_id,
            finish_id,
            delta,
        };

        nfa.insert_transition(start_id, Lable::Eps, nfa1_sid);
        nfa.insert_transition(start_id, Lable::Eps, nfa2_sid);
        nfa.insert_transition(nfa1_fid, Lable::Eps, finish_id);
        nfa.insert_transition(nfa2_fid, Lable::Eps, finish_id);

        nfa
    }

    pub fn of_concat(nfa1: Nfa, nfa2: Nfa, _: usize) -> Self {
        let start_id = nfa1.sid();
        let finish_id = nfa2.fid();

        let nfa1_fid = nfa1.fid();
        let nfa2_sid = nfa2.sid();

        let mut delta = nfa1.get_delta();
        let delta2 = nfa2.get_delta();

        delta.extend(delta2);

        let mut nfa = Nfa {
            start_id,
            finish_id,
            delta,
        };

        nfa.insert_transition(nfa1_fid, Lable::Eps, nfa2_sid);

        nfa
    }

    pub fn of_star(nfa: Nfa, offset: usize) -> Self {
        let start_id = offset;
        let finish_id = offset + 1;

        let nfa_sid = nfa.sid();
        let nfa_fid = nfa.fid();

        let delta = nfa.get_delta();

        let mut nfa = Nfa {
            start_id,
            finish_id,
            delta,
        };

        nfa.insert_transition(nfa_fid, Lable::Eps, nfa_sid);
        nfa.insert_transition(start_id, Lable::Eps, nfa_sid);
        nfa.insert_transition(start_id, Lable::Eps, finish_id);
        nfa.insert_transition(nfa_fid, Lable::Eps, finish_id);

        nfa
    }

    pub fn of_plus(nfa: Nfa, offset: usize) -> Self {
        let start_id = offset;
        let finish_id = offset + 1;

        let nfa_sid = nfa.sid();
        let nfa_fid = nfa.fid();

        let delta = nfa.get_delta();

        let mut nfa = Nfa {
            start_id,
            finish_id,
            delta,
        };

        nfa.insert_transition(nfa_fid, Lable::Eps, nfa_sid);
        nfa.insert_transition(start_id, Lable::Eps, nfa_sid);
        nfa.insert_transition(nfa_fid, Lable::Eps, finish_id);

        nfa
    }

    pub fn of_question(nfa: Nfa, offset: usize) -> Self {
        let start_id = offset;
        let finish_id = offset + 1;

        let nfa_sid = nfa.sid();
        let nfa_fid = nfa.fid();

        let delta = nfa.get_delta();

        let mut nfa = Nfa {
            start_id,
            finish_id,
            delta,
        };

        nfa.insert_transition(start_id, Lable::Eps, nfa_sid);
        nfa.insert_transition(start_id, Lable::Eps, finish_id);
        nfa.insert_transition(nfa_fid, Lable::Eps, finish_id);

        nfa
    }

    pub fn insert_transition(&mut self, src_id: usize, lable: Lable, dst_id: usize) {
        match self.delta.entry((src_id, lable)) {
            Occupied(mut occ) => {
                occ.get_mut().push(dst_id);
            }
            Vacant(vac) => {
                vac.insert(vec![dst_id]);
            }
        }
    }

    pub fn sid(&self) -> usize {
        self.start_id
    }

    pub fn fid(&self) -> usize {
        self.finish_id
    }

    pub fn get_delta(self) -> HashMap<(usize, Lable), Vec<usize>> {
        self.delta
    }

    pub fn delta(&self, (state_id, lable): (usize, Lable)) -> Option<&Vec<usize>> {
        self.delta.get(&(state_id, lable))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn of_text() {
        let nfa = Nfa::of_text("abc", 0);

        assert_eq!(nfa.sid(), 0);
        assert_eq!(nfa.delta((0, 'a'.into())).unwrap()[0], 1);
        assert_eq!(nfa.delta((1, 'b'.into())).unwrap()[0], 2);
        assert_eq!(nfa.delta((2, 'c'.into())).unwrap()[0], 3);
        assert_eq!(nfa.fid(), 3);
    }

    #[test]
    fn of_text_with_offset() {
        let nfa = Nfa::of_text("abc", 5);

        assert_eq!(nfa.sid(), 5);
        assert_eq!(nfa.delta((5, 'a'.into())).unwrap()[0], 6);
        assert_eq!(nfa.delta((6, 'b'.into())).unwrap()[0], 7);
        assert_eq!(nfa.delta((7, 'c'.into())).unwrap()[0], 8);
        assert_eq!(nfa.fid(), 8);
    }

    #[test]
    fn of_digit() {
        let nfa = Nfa::of_digit(1, 0);

        assert_eq!(nfa.sid(), 0);
        assert_eq!(nfa.delta((0, '1'.into())).unwrap()[0], 1);
        assert_eq!(nfa.fid(), 1);
    }

    #[test]
    fn of_digit_with_offset() {
        let nfa = Nfa::of_digit(1, 5);

        assert_eq!(nfa.sid(), 5);
        assert_eq!(nfa.delta((5, '1'.into())).unwrap()[0], 6);
        assert_eq!(nfa.fid(), 6);
    }

    #[test]
    fn of_char() {
        let nfa = Nfa::of_char('a', 0);

        assert_eq!(nfa.sid(), 0);
        assert_eq!(nfa.delta((0, 'a'.into())).unwrap()[0], 1);
        assert_eq!(nfa.fid(), 1);
    }

    #[test]
    fn of_char_with_offset() {
        let nfa = Nfa::of_char('a', 5);

        assert_eq!(nfa.sid(), 5);
        assert_eq!(nfa.delta((5, 'a'.into())).unwrap()[0], 6);
        assert_eq!(nfa.fid(), 6);
    }

    #[test]
    fn of_eps() {
        let nfa = Nfa::of_eps(0);

        assert_eq!(nfa.sid(), 0);
        assert_eq!(nfa.delta((0, Lable::Eps)).unwrap()[0], 1);
        assert_eq!(nfa.fid(), 1);
    }

    #[test]
    fn of_eps_with_offset() {
        let nfa = Nfa::of_eps(5);

        assert_eq!(nfa.sid(), 5);
        assert_eq!(nfa.delta((5, Lable::Eps)).unwrap()[0], 6);
        assert_eq!(nfa.fid(), 6);
    }

    #[test]
    fn of_cdash() {
        let nfa = Nfa::of_cdash('a', 'c', 0);

        assert_eq!(nfa.sid(), 0);
        assert_eq!(nfa.delta((0, 'a'.into())).unwrap()[0], 1);
        assert_eq!(nfa.delta((0, 'b'.into())).unwrap()[0], 1);
        assert_eq!(nfa.delta((0, 'c'.into())).unwrap()[0], 1);
        assert_eq!(nfa.fid(), 1);
    }

    #[test]
    fn of_cdash_with_offset() {
        let nfa = Nfa::of_cdash('a', 'c', 5);

        assert_eq!(nfa.sid(), 5);
        assert_eq!(nfa.delta((5, 'a'.into())).unwrap()[0], 6);
        assert_eq!(nfa.delta((5, 'b'.into())).unwrap()[0], 6);
        assert_eq!(nfa.delta((5, 'c'.into())).unwrap()[0], 6);
        assert_eq!(nfa.fid(), 6);
    }

    #[test]
    fn of_ndash() {
        let nfa = Nfa::of_ndash(2, 4, 0);

        assert_eq!(nfa.sid(), 0);
        assert_eq!(nfa.delta((0, '2'.into())).unwrap()[0], 1);
        assert_eq!(nfa.delta((0, '3'.into())).unwrap()[0], 1);
        assert_eq!(nfa.delta((0, '4'.into())).unwrap()[0], 1);
        assert_eq!(nfa.fid(), 1);
    }

    #[test]
    fn of_ndash_with_offset() {
        let nfa = Nfa::of_ndash(2, 4, 5);

        assert_eq!(nfa.sid(), 5);
        assert_eq!(nfa.delta((5, '2'.into())).unwrap()[0], 6);
        assert_eq!(nfa.delta((5, '3'.into())).unwrap()[0], 6);
        assert_eq!(nfa.delta((5, '4'.into())).unwrap()[0], 6);
        assert_eq!(nfa.fid(), 6);
    }

    #[test]
    fn of_or() {
        let to_be = Nfa::of_text("To be", 0);
        let not_to_be = Nfa::of_text("Not to be", to_be.fid() + 1);

        let offset = not_to_be.fid() + 1;

        let to_be_or_not_to_be = Nfa::of_or(to_be, not_to_be, offset);

        assert_eq!(to_be_or_not_to_be.sid(), 16);
        assert_eq!(
            to_be_or_not_to_be.delta((16, Lable::Eps)).unwrap(),
            &vec![0, 6]
        );

        for (index, c) in "To be".chars().enumerate() {
            assert_eq!(
                to_be_or_not_to_be.delta((index, c.into())).unwrap()[0],
                index + 1
            )
        }

        for (index, c) in "Not to be".chars().enumerate() {
            assert_eq!(
                to_be_or_not_to_be.delta((index + 6, c.into())).unwrap()[0],
                index + 6 + 1
            );
        }

        assert_eq!(
            to_be_or_not_to_be.delta((5, Lable::Eps)).unwrap()[0],
            to_be_or_not_to_be.fid()
        );
        assert_eq!(
            to_be_or_not_to_be.delta((15, Lable::Eps)).unwrap()[0],
            to_be_or_not_to_be.fid()
        );
    }

    #[test]
    fn of_concat() {
        let to_be = Nfa::of_text("To be", 0);
        let or = Nfa::of_text(" or ", to_be.fid() + 1);
        let not_to_be = Nfa::of_text("Not to be", or.fid() + 1);

        let to_be_or = Nfa::of_concat(to_be, or, not_to_be.fid() + 1);

        let offset = to_be_or.fid() + 1;

        let to_be_or_not_to_be = Nfa::of_concat(to_be_or, not_to_be, offset + 1);

        assert_eq!(to_be_or_not_to_be.sid(), 0);
        assert_eq!(to_be_or_not_to_be.delta((0, 'T'.into())).unwrap()[0], 1);
        assert_eq!(to_be_or_not_to_be.delta((1, 'o'.into())).unwrap()[0], 2);
        assert_eq!(to_be_or_not_to_be.delta((2, ' '.into())).unwrap()[0], 3);
        assert_eq!(to_be_or_not_to_be.delta((3, 'b'.into())).unwrap()[0], 4);
        assert_eq!(to_be_or_not_to_be.delta((4, 'e'.into())).unwrap()[0], 5);

        assert_eq!(to_be_or_not_to_be.delta((5, Lable::Eps)).unwrap()[0], 6);

        assert_eq!(to_be_or_not_to_be.delta((6, ' '.into())).unwrap()[0], 7);
        assert_eq!(to_be_or_not_to_be.delta((7, 'o'.into())).unwrap()[0], 8);
        assert_eq!(to_be_or_not_to_be.delta((8, 'r'.into())).unwrap()[0], 9);
        assert_eq!(to_be_or_not_to_be.delta((9, ' '.into())).unwrap()[0], 10);

        assert_eq!(to_be_or_not_to_be.delta((10, Lable::Eps)).unwrap()[0], 11);

        assert_eq!(to_be_or_not_to_be.delta((11, 'N'.into())).unwrap()[0], 12);
        assert_eq!(to_be_or_not_to_be.delta((12, 'o'.into())).unwrap()[0], 13);
        assert_eq!(to_be_or_not_to_be.delta((13, 't'.into())).unwrap()[0], 14);
        assert_eq!(to_be_or_not_to_be.delta((14, ' '.into())).unwrap()[0], 15);
        assert_eq!(to_be_or_not_to_be.delta((15, 't'.into())).unwrap()[0], 16);
        assert_eq!(to_be_or_not_to_be.delta((16, 'o'.into())).unwrap()[0], 17);
        assert_eq!(to_be_or_not_to_be.delta((17, ' '.into())).unwrap()[0], 18);
        assert_eq!(to_be_or_not_to_be.delta((18, 'b'.into())).unwrap()[0], 19);
        assert_eq!(to_be_or_not_to_be.delta((19, 'e'.into())).unwrap()[0], 20);
        assert_eq!(to_be_or_not_to_be.fid(), 20);
    }

    #[test]
    fn of_star() {
        let a_to_z = Nfa::of_cdash('a', 'z', 0);
        let offset = a_to_z.fid() + 1;
        let a_to_z_star = Nfa::of_star(a_to_z, offset);

        assert_eq!(a_to_z_star.sid(), 2);
        assert!(a_to_z_star.delta((2, Lable::Eps)).unwrap().contains(&0));
        assert!(a_to_z_star.delta((2, Lable::Eps)).unwrap().contains(&3));
        assert!(a_to_z_star.delta((1, Lable::Eps)).unwrap().contains(&0));
        assert!(a_to_z_star.delta((1, Lable::Eps)).unwrap().contains(&3));
    }

    #[test]
    fn of_plus() {
        let a_to_z = Nfa::of_cdash('a', 'z', 0);
        let offset = a_to_z.fid() + 1;
        let a_to_z_star = Nfa::of_plus(a_to_z, offset);

        assert_eq!(a_to_z_star.sid(), 2);
        assert!(a_to_z_star.delta((2, Lable::Eps)).unwrap().contains(&0));
        assert!(!a_to_z_star.delta((2, Lable::Eps)).unwrap().contains(&3));
        assert!(a_to_z_star.delta((1, Lable::Eps)).unwrap().contains(&0));
        assert!(a_to_z_star.delta((1, Lable::Eps)).unwrap().contains(&3));
    }

    #[test]
    fn of_question() {
        let a_to_z = Nfa::of_cdash('a', 'z', 0);
        let offset = a_to_z.fid() + 1;
        let a_to_z_star = Nfa::of_question(a_to_z, offset);

        assert_eq!(a_to_z_star.sid(), 2);
        assert!(a_to_z_star.delta((2, Lable::Eps)).unwrap().contains(&0));
        assert!(a_to_z_star.delta((2, Lable::Eps)).unwrap().contains(&3));
        assert!(!a_to_z_star.delta((1, Lable::Eps)).unwrap().contains(&0));
        assert!(a_to_z_star.delta((1, Lable::Eps)).unwrap().contains(&3));
    }
}
