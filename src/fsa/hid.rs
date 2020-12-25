pub struct Hid {
    ids: Vec<usize>,

    // Current Id PoinTeR: pointer to the highest level id in hierarchy
    cid_ptr: usize,
}

impl Hid {
    pub fn init(id: usize) -> Self {
        Hid {
            ids: vec![id],
            cid_ptr: 0,
        }
    }

    pub fn consume(&mut self) {
        if self.cid_ptr > 0 {
            self.cid_ptr -= 1;
        }
    }

    pub fn append(&mut self, id: usize) {
        self.ids.push(id);
        self.cid_ptr += 1;
    }

    pub fn cid(&self) -> usize {
        self.ids[self.cid_ptr]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_level_hierarchy() {
        let hid = Hid::init(0);

        assert_eq!(hid.cid(), 0);
    }

    #[test]
    fn two_level_hierarchy() {
        let mut hid = Hid::init(0);

        hid.append(2);

        assert_eq!(hid.cid(), 2);
    }

    #[test]
    fn consume_id() {
        let mut hid = Hid::init(0);

        hid.append(2);
        hid.consume();

        assert_eq!(hid.cid(), 0);
    }

    #[test]
    fn multiple_consume() {
        let mut hid = Hid::init(0);

        hid.append(2);
        hid.consume();
        hid.consume(); // consume gets called more than append

        assert_eq!(hid.cid(), 0);
    }
}
