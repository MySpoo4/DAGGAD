use crate::utils::StrBuilder;

use super::{TWDNode, node::ALPHABET_SIZE};

#[derive(Debug)]
pub struct TWDawg {
    nodes: Vec<TWDNode>,
}

impl TWDawg {
    pub fn new(nodes: Vec<TWDNode>) -> Self {
        Self { nodes }
    }

    pub fn root(&self) -> Option<&TWDNode> {
        self.nodes.get(0)
    }

    pub fn get_node(&self, id: usize) -> &TWDNode {
        &self.nodes[id]
    }

    pub fn contains(&self, word: &str) -> bool {
        // root node
        let mut cur_node = self.get_node(0);

        for c in word.to_ascii_uppercase().chars().rev() {
            cur_node = match cur_node.get(c as u8) {
                Some(id) => self.get_node(id),
                None => return false,
            }
        }

        cur_node
            .get('[' as u8)
            .map_or(false, |n| self.get_node(n).is_terminal)
    }

    pub fn get_all_words(&self) -> Vec<String> {
        let mut words = Vec::new();
        let mut builder = StrBuilder::new();
        self.get_words_helper(&mut words, &mut builder, 0);

        words
    }

    fn get_words_helper(&self, words: &mut Vec<String>, builder: &mut StrBuilder, node_id: usize) {
        let node = self.get_node(node_id);

        if node.is_terminal {
            words.push(builder.build());
        }

        node.edges
            .iter()
            .take(ALPHABET_SIZE)
            .enumerate()
            .for_each(|(i, &id)| {
                if let Some(node_id) = id {
                    builder.push(char::from_u32((i + 65) as u32).unwrap());
                    self.get_words_helper(words, builder, node_id);
                    builder.pop();
                }
            });

        if let Some(node_id) = node.edges[ALPHABET_SIZE] {
            builder.update_rev();
            self.get_words_helper(words, builder, node_id);
            builder.revert_rev();
        }
    }
}
