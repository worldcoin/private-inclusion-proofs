use crate::{print_tree, Tree};
use aligned_cmov::{A8Bytes, Aligned, GenericArray, A8};
use rand::{thread_rng, RngCore};
use ruint::{aliases::U256, uint};
use typenum::U32;

// See <https://docs.rs/ark-bn254/latest/ark_bn254>
pub const MODULUS: U256 =
    uint!(21888242871839275222246405745257275088548364400416034343698204186575808495617_U256);

pub fn aligned_bytes_to_u256(aligned_bytes: &A8Bytes<U32>) -> U256 {
    let mut bytes = [0u8; 32];
    aligned_bytes
        .iter()
        .zip(bytes.iter_mut())
        .for_each(|(v0, v1)| {
            *v1 = *v0;
        });

    U256::from_be_bytes(bytes)
}

pub fn random_tree(depth: usize) -> Tree {
    let mut rng = thread_rng();

    let leaf_count = 1 << depth;
    let mut tree = Tree::new(depth);
    for leaf_index in 0..leaf_count {
        let mut rand_arr = [0u8; 32];
        rng.fill_bytes(&mut rand_arr);
        rand_arr[0] = 0;
        let rand_leaf = Aligned::<A8, _>(GenericArray::from(rand_arr));
        tree.update(leaf_index, rand_leaf);
    }
    tree
}

/// Credit: https://github.com/worldcoin/semaphore-rs
pub fn bytes_to_hex_str(bytes: &[u8]) -> String {
    // TODO: Replace `M` with a const expression once it's stable.
    debug_assert_eq!(66, 2 * bytes.len() + 2);
    let mut result = vec![0u8; 66];
    result[0] = b'0';
    result[1] = b'x';
    hex::encode_to_slice(&bytes[..], &mut result[2..]).expect("the buffer is correctly sized");
    String::from_utf8(result).unwrap()
}
