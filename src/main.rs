use aligned_cmov::{A8Bytes, Aligned, GenericArray, A8};
use private_inclusion_proofs::{
    print_tree,
    utils::{bytes_to_hex_str, random_tree},
    Tree,
};

fn main() {
    // let tree = random_tree(4, 1);
    // print_tree(&tree);
    // let leaf = tree.leaf(0);
    // let proof = tree.inclusion_proof(leaf);
    // let proof_hex = proof
    //     .iter()
    //     .map(|node| bytes_to_hex_str(node.as_slice()))
    //     .collect::<Vec<String>>();
    // dbg!(proof_hex);
}
