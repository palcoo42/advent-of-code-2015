use super::{aunts::Aunts, sue::Sue, tape::Tape};

pub struct Computer<'a> {
    aunts: &'a Aunts,
}

impl<'a> Computer<'a> {
    pub fn new(aunts: &'a Aunts) -> Self {
        Self { aunts }
    }

    pub fn find_sue(&self, tape: &Tape) -> Option<&Sue> {
        let sues = self
            .aunts
            .iter()
            .filter(|&sue| {
                sue.iter().all(|(item, count)| match tape.get(item) {
                    Some(c) => c == *count,
                    None => false,
                })
            })
            .collect::<Vec<_>>();

        if sues.len() > 1 {
            panic!("Too many sues found '{}'", sues.len());
        }

        Some(sues[0])
    }
}
