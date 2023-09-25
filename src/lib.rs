use aligned_cmov::{typenum::U8, A8Bytes, Aligned, A8};

struct Level {
    data: Vec<A8Bytes<32>>,
}
impl Level {
    fn update_index(&mut self, index: usize, value: &A8Bytes<32>) {
        self.data[index] = value.clone();
    }

    fn read_index(&self, index: usize) -> &A8Bytes<32> {
        &self.data[index]
    }

    fn scan_and_load_sibling(&self, node: &A8Bytes<32>) -> (usize, A8Bytes<32>) {
        // Load 4Kb using ocall; do some stuff and store.
        let mut sibling = Default::default();
        let mut i = 0;
        let mut node_index = 0;
        while i < self.data.len() {
            // TODO: make oblivious
            if self.data[i] == node {
                sibling = self.data[i + 1].clone();
                node_index = i;
            }
            if self.data[i + 1] == node {
                sibling = self.data[i].clone();
                node_index = i + 1;
            }
            i += 2;
        }
        (node_index, sibling)
    }

    fn scan_sibling_node(&self, node_index: usize) -> (A8Bytes<32>) {
        let sibling_index = sibling_index_ct(node_index);
        let mut sibling = Default::default();
        while i < self.data.len() {
            // TODO: make oblivious
            if sibling_index == i {
                sibling = self.data[i].clone;
            }
            i += 1;
        }

        sibling
    }
}

struct Tree {
    levels: Vec<Level>,
    root:   A8Bytes<32>,
    depth:  usize,
}

impl Tree {
    pub fn new(depth: usize) -> Tree {
        let mut levels = Vec::new();
        for i in 1..depth + 1 {
            let level = Level {
                data: vec![Default::default(); 2 << i],
            };
            levels.push(level);
        }

        Tree {
            levels,
            root: Default::default(),
            depth,
        }
    }

    pub fn update(&mut self, mut index: usize, mut value: A8Bytes<32>) {
        let mut curr_depth = self.depth;

        while curr_depth > 0 {
            let level = &mut self.levels[curr_depth - 1];

            level.update_index(index, &value);

            // read sibling node
            let sibling_index = sibling_index(index);
            let sibling_node = level.read_index(sibling_index);

            // TODO: hash current value and sibling node value
            value = Default::default();

            curr_depth -= 1;

            if curr_depth == 0 {
                self.root = value;
            }
        }
    }

    pub fn inclusion_proof(&self, leaf: &A8Bytes<32>) -> Vec<A8Bytes<32>> {
        let inclusion_proof = vec![Default::default(), self.depth];

        // find leaf in level `depth`
        let (node_index, sibling_node) = self.levels[depth - 1].scan_and_load_sibling(leaf);
        inclusion_proof[0] = sibling_node;

        let inclusion_proof_index = 1;
        let mut curr_depth = self.depth - 1 - 1;

        // parent and parent's sibling index at level `depth-1`
        let mut parent_index = node_index >> 1;

        while curr_depth > 0 {
            inclusion_proof[inclusion_proof_index] =
                self.levels[curr_depth - 1].scan_sibling_node(parent_index);
            inclusion_proof_index += 1;
            curr_depth -= 1;

            parent_index >>= 1;
        }

        inclusion_proof
    }

    pub fn map_index_to_parent_and_sibling(index: usize) -> (usize, usize) {
        let parent_index = index / 2;
        // TODO: must be swapped out with a constant time function
        let parent_sibling_index = if parent_index & 1 == 1 {
            parent_index - 1
        } else {
            parent_index + 1
        };
    }
}

pub fn sibling_index(node_index: usize) -> usize {
    if index & 1 == 1 {
        index - 1
    } else {
        index + 1
    }
}

pub fn sibling_index_ct(node_index: usize) -> usize {
    // TODO: make constant time
    if index & 1 == 1 {
        index - 1
    } else {
        index + 1
    }
}

pub fn print_tree(tree: &Tree) {
    // print root and the print rest of the vcalues
}
