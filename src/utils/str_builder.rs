#[derive(Debug)]
pub struct StrBuilder {
    str: String,
    rev_indices: Vec<usize>,
}

impl StrBuilder {
    pub fn new() -> Self {
        Self {
            str: String::new(),
            rev_indices: Vec::new(),
        }
    }

    pub fn push(&mut self, c: char) {
        self.str.push(c);
    }

    pub fn pop(&mut self) {
        self.str.pop();
    }

    pub fn update_rev(&mut self) {
        self.rev_indices.push(self.str.len());
    }

    pub fn revert_rev(&mut self) {
        self.rev_indices.pop();
    }

    pub fn build(&self) -> String {
        let rev_idx = self.rev_indices.last().map_or(0, |i| *i);
        self.str[0..rev_idx]
            .chars()
            .rev()
            .chain(self.str[rev_idx..].chars())
            .collect()
    }
}
