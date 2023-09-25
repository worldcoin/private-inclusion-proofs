use crate::utils::{aligned_bytes_to_u256, bytes_to_hex_str};
use aligned_cmov::{typenum::U8, A8Bytes, Aligned, GenericArray, A8};
use semaphore::poseidon;
use typenum::U32;

pub mod utils;

pub struct PoseidonHash;

impl PoseidonHash {
    fn hash_node(left: &A8Bytes<U32>, right: &A8Bytes<U32>) -> A8Bytes<U32> {
        let left = aligned_bytes_to_u256(left);
        let right = aligned_bytes_to_u256(right);
        let hash: [u8; 32] = poseidon::hash2(left, right).to_be_bytes();
        A8Bytes::<U32>::from(Aligned(GenericArray::from(hash)))
    }
}

pub struct Level {
    data: Vec<A8Bytes<U32>>,
}
impl Level {
    fn update_index(&mut self, index: usize, value: &A8Bytes<U32>) {
        self.data[index] = value.clone();
    }

    fn read_index(&self, index: usize) -> &A8Bytes<U32> {
        &self.data[index]
    }

    fn scan_and_load_sibling(&self, node: &A8Bytes<U32>) -> (usize, A8Bytes<U32>) {
        // Load 4Kb using ocall; do some stuff and store.
        let mut sibling = Default::default();
        let mut i = 0;
        let mut node_index = 0;
        while i < self.data.len() {
            // TODO: make oblivious
            if &self.data[i] == node {
                sibling = self.data[i + 1].clone();
                node_index = i;
            }
            if &self.data[i + 1] == node {
                sibling = self.data[i].clone();
                node_index = i + 1;
            }
            i += 2;
        }
        (node_index, sibling)
    }

    fn scan_sibling_node(&self, node_index: usize) -> (A8Bytes<U32>) {
        let sibling_index = sibling_index_ct(node_index);
        let mut sibling = Default::default();
        let mut i = 0;
        while i < self.data.len() {
            // TODO: make oblivious
            if sibling_index == i {
                sibling = self.data[i].clone();
            }
            i += 1;
        }

        sibling
    }
}

pub struct Tree {
    levels: Vec<Level>,
    root: A8Bytes<U32>,
    depth: usize,
}

impl Tree {
    pub fn new(depth: usize) -> Tree {
        let mut levels = Vec::new();
        for i in 1..depth + 1 {
            let level = Level {
                data: vec![Default::default(); 1 << i],
            };
            levels.push(level);
        }

        Tree {
            levels,
            root: Default::default(),
            depth,
        }
    }

    pub fn update(&mut self, mut index: usize, mut value: A8Bytes<U32>) {
        let mut curr_depth = self.depth;

        while curr_depth > 0 {
            let level = &mut self.levels[curr_depth - 1];

            level.update_index(index, &value);

            // read sibling node
            let sibling_index = sibling_index(index);
            let sibling_node = level.read_index(sibling_index);

            // println!(
            //     "index: {index}; node: {} ; sibling {}",
            //     bytes_to_hex_str(value.as_slice()),
            //     bytes_to_hex_str(sibling_node.as_slice())
            // );

            // if node's index is off then it's the right child, otherwise left.
            let (left, right) = if index & 1 == 1 {
                (sibling_node, &value)
            } else {
                (&value, sibling_node)
            };
            value = PoseidonHash::hash_node(left, right);

            curr_depth -= 1;
            index >>= 1;

            if curr_depth == 0 {
                self.root = value.clone();
            }
        }
    }

    pub fn inclusion_proof(&self, leaf: &A8Bytes<U32>) -> Vec<A8Bytes<U32>> {
        let mut inclusion_proof = vec![Default::default(); self.depth];

        // find leaf in level `depth`
        let (node_index, sibling_node) = self.levels[self.depth - 1].scan_and_load_sibling(leaf);
        inclusion_proof[0] = sibling_node;

        let mut inclusion_proof_index = 1;
        let mut curr_depth = self.depth - 1;

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

    pub fn root(&self) -> &A8Bytes<U32> {
        &self.root
    }

    pub fn leaf(&self, index: usize) -> &A8Bytes<U32> {
        &self.levels[self.depth - 1].data[index]
    }
}

pub fn sibling_index(node_index: usize) -> usize {
    if node_index & 1 == 1 {
        node_index - 1
    } else {
        node_index + 1
    }
}

pub fn sibling_index_ct(node_index: usize) -> usize {
    // TODO: make constant time
    if node_index & 1 == 1 {
        node_index - 1
    } else {
        node_index + 1
    }
}
pub fn print_tree(tree: &Tree) {
    // print root and the print rest of the vcalues
    println!("{:?}", bytes_to_hex_str(tree.root().as_slice()));

    let depth = tree.depth;
    for l in 0..depth {
        let mut l_nodes = vec![];
        for node in tree.levels[l].data.iter() {
            l_nodes.push(bytes_to_hex_str(node.as_slice()));
        }
        println!("Level {}: {:?}", l + 1, l_nodes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{rand_leaf, random_tree, seeded_rng};
    use aligned_cmov::{typenum::U8, A8Bytes, Aligned, GenericArray, A8};
    use rand::thread_rng;

    #[test]
    fn inclusion_proof_works() {
        let mut rng = seeded_rng();
        let mut tree = random_tree(16, 1 << 16, &mut rng);

        let mut leaf_index = 2;
        let leaf = tree.leaf(leaf_index);
        let proof = tree.inclusion_proof(leaf);

        let proof_hex = proof
            .iter()
            .map(|node| bytes_to_hex_str(node.as_slice()))
            .collect::<Vec<String>>();

        // check proof
        let mut curr_index = leaf_index;
        let mut curr_node = A8Bytes::from(leaf.clone());
        for i in 0..tree.depth {
            let (left, right): (&A8Bytes<U32>, &A8Bytes<U32>) = if curr_index & 1 == 1 {
                (&proof[i], &curr_node)
            } else {
                (&curr_node, &proof[i])
            };

            curr_node = PoseidonHash::hash_node(left, right);
            curr_index >>= 1;
        }

        assert_eq!(&curr_node, tree.root());
    }
}
