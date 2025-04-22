use std::collections::HashMap;

use crate::utils::RevWord;

use super::{Daggad, DaggadNode};

#[derive(Debug)]
pub struct DaggadBuilder<'words> {
    nodes: Vec<DaggadNode>,
    minimized: HashMap<DaggadNode, usize>,
    unchecked: Vec<(usize, u8, usize)>,
    previous_word: RevWord<'words>,
}

impl<'word> DaggadBuilder<'word> {
    fn new() -> Self {
        let mut nodes = Vec::new();
        nodes.push(DaggadNode::new(0));

        Self {
            nodes,
            minimized: HashMap::new(),
            unchecked: Vec::new(),
            previous_word: RevWord::default(),
        }
    }

    pub fn build_one_way<'words, I: IntoIterator<Item = &'words str>>(words: I) -> Daggad {
        // efficient ways to store the reverse word iterations
        let mut rev_words: Vec<RevWord> = words
            .into_iter()
            .map(|w| RevWord {
                word: w,
                rev_idx: 1,
            })
            .collect();

        rev_words.sort();

        let mut builder = Self::new();
        rev_words.into_iter().for_each(|w| builder.insert(w));

        builder.finalize()
    }

    pub fn build<'words, I: IntoIterator<Item = &'words str>>(words: I) -> Daggad {
        // efficient ways to store the reverse word iterations
        let mut rev_words: Vec<RevWord> = words
            .into_iter()
            .flat_map(|w| RevWord::build_all(w))
            .collect();

        rev_words.sort();

        let mut builder = Self::new();
        rev_words.into_iter().for_each(|w| builder.insert(w));

        builder.finalize()
    }

    fn insert(&mut self, word: RevWord<'word>) {
        // Find longest common prefix with previous word
        let prefix_len = self.common_prefix(&word);

        // Minimize the remaining suffix of the previous word
        self.minimize(prefix_len);

        word.iter_all()
            .skip(prefix_len)
            .map(|c| c as u8)
            .for_each(|c| {
                let node_id = self.unchecked.last().map_or(0, |(_, _, n)| *n);

                // Extend the last node with the remaining unmatched letters
                let next_id = self.build_node();
                self.nodes.get_mut(node_id).unwrap().set(c, next_id);

                self.unchecked.push((node_id, c, next_id));
            });

        self.nodes
            .get_mut(self.unchecked.last().unwrap().2)
            .unwrap()
            .is_terminal = true;

        self.previous_word = word;
    }

    fn minimize(&mut self, prefix_len: usize) {
        while self.unchecked.len() > prefix_len {
            let (parent_id, c, child_id) = self.unchecked.pop().unwrap();
            let child = self.nodes.get(child_id).unwrap();

            if let Some(&new_child_id) = self.minimized.get(child) {
                // Reduces node count (prob???)
                self.nodes.get_mut(new_child_id).unwrap().is_terminal |= child.is_terminal;
                self.nodes.remove(child_id);

                self.nodes.get_mut(parent_id).unwrap().set(c, new_child_id);
            } else {
                self.minimized
                    .insert(self.nodes.get(child_id).unwrap().clone(), child_id);
            }
        }
    }

    fn finalize(mut self) -> Daggad {
        self.minimize(0);
        // Prob not necessary
        self.minimized.clear();
        self.unchecked.clear();

        Daggad::new(self.nodes)
    }

    fn build_node(&mut self) -> usize {
        let indice = self.nodes.len();
        self.nodes.push(DaggadNode::new(indice));
        indice
    }

    // gets length of common prefix
    fn common_prefix(&self, word: &RevWord) -> usize {
        self.previous_word
            .iter_all()
            .zip(word.iter_all())
            .take_while(|(a, b)| a == b)
            .count()
    }
}
