use super::present::Present;

pub struct Presents {
    presents: Vec<Present>,
}

impl Presents {
    pub fn new(presents: Vec<Present>) -> Self {
        Self { presents }
    }

    pub fn wrapping_paper(&self) -> u32 {
        self.presents.iter().map(|p| p.wrapping_paper()).sum()
    }

    pub fn ribbon(&self) -> u32 {
        self.presents.iter().map(|p| p.ribbon()).sum()
    }
}
