use aligned_cmov::{A8Bytes, Aligned, GenericArray, A8};
use private_inclusion_proofs::{print_tree, utils::random_tree, Tree};

fn main() {
    let tree = random_tree(4);
    print_tree(&tree);
}
