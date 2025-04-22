use std::fmt;

#[derive(Eq, PartialEq, Default)]
pub struct RevWord<'w> {
    pub word: &'w str,
    pub rev_idx: usize,
}

impl<'w> RevWord<'w> {
    // make private later
    pub fn build(word: &'w str, rev_idx: usize) -> Self {
        Self { word, rev_idx }
    }

    pub fn build_all(word: &'w str) -> Vec<Self> {
        (1..word.len() + 1)
            .into_iter()
            .map(|i| Self::build(word, i))
            .collect()
    }

    pub fn iter_prefix(&self) -> impl Iterator<Item = char> {
        self.word[0..self.rev_idx]
            .chars()
            .map(|c| c.to_ascii_uppercase())
            .rev()
    }
    pub fn iter_suffix(&self) -> impl Iterator<Item = char> {
        self.word[self.rev_idx..]
            .chars()
            .map(|c| c.to_ascii_uppercase())
    }
    pub fn iter_all(&self) -> impl Iterator<Item = char> {
        self.iter_prefix()
            .chain(std::iter::once('['))
            .chain(self.iter_suffix())
    }
}

impl Ord for RevWord<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.iter_all().cmp(other.iter_all())
    }
}

impl PartialOrd for RevWord<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for RevWord<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rev: String = self.iter_all().collect();

        f.debug_struct("RevWord")
            .field("word", &self.word)
            .field("rev", &rev)
            .field("rev_idx", &self.rev_idx)
            .finish()
    }
}
