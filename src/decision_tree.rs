use common::{Classifier, Digit, Picture, is_not_black};
use rand::{sample, thread_rng};
use std::collections::HashMap;

pub type DecisionTree = Vec<(usize, Digit)>;

impl Classifier for DecisionTree {
    fn classify(&self, p: &Picture) -> Option<Digit> {
        for &(i, digit) in self.iter() {
            if is_not_black(p[i]) {
                return Some(digit);
            }
        }

        None
    }
}

pub fn new_decision_tree(modes: &HashMap<usize, Digit>) -> DecisionTree {
    let num_pixels = 25;
    let mut decision_tree = Vec::with_capacity(num_pixels);
    let mut rng = thread_rng();

    let sampled_modes = sample(&mut rng, modes.iter(), num_pixels);

    for &(&i, &digit) in sampled_modes.iter() {
        decision_tree.push((i, digit));
    }

    decision_tree
}
