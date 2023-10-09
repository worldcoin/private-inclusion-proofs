use crate::{print_tree, Tree};
use aligned_cmov::{A8Bytes, Aligned, GenericArray, A8};
use rand::{thread_rng, CryptoRng, RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;
use ruint::{aliases::U256, uint};
use typenum::U32;

/// Converts 8 byte aligned 32 bytes to U256
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

pub fn rand_leaf<R: CryptoRng + RngCore>(rng: &mut R) -> A8Bytes<U32> {
    let mut rand_arr = [0u8; 32];
    rng.fill_bytes(&mut rand_arr);
    rand_arr[0] = 0;
    Aligned::<A8, _>(GenericArray::from(rand_arr))
}

pub fn random_tree<R: CryptoRng + RngCore>(depth: usize, leaf_count: usize, rng: &mut R) -> Tree {
    let mut tree = Tree::new(depth);
    for leaf_index in 0..leaf_count {
        let rand_leaf = rand_leaf(rng);
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

pub fn bytes_from_hex(hex_str: &str) -> [u8; 32] {
    let str = trim_hex_prefix(hex_str);
    let mut result = [0_u8; 32];
    hex::decode_to_slice(str, &mut result).unwrap();
    result
}

fn trim_hex_prefix(str: &str) -> &str {
    if str.len() >= 2 && (&str[..2] == "0x" || &str[..2] == "0X") {
        &str[2..]
    } else {
        str
    }
}

pub fn seeded_rng() -> ChaCha8Rng {
    let rng = ChaCha8Rng::from_seed([0u8; 32]);
    rng
}

#[cfg(test)]
mod tests {
    use crate::PoseidonHash;

    use super::*;

    #[test]
    fn trial() {
        let left = Aligned(GenericArray::from(bytes_from_hex(
            "0x0000ef2f895f40d67f5bb8e81f09a5a12c840ec3ce9a7f3b181be188ef711a1e",
        )));
        let right = Aligned(GenericArray::from(bytes_from_hex(
            "0x004ce172b9216f419f445367456d5619314a42a3da86b001387bfdb80e0cfe42",
        )));
        let hash = bytes_to_hex_str(PoseidonHash::hash_node(&left, &right).as_slice());
        dbg!(hash);
    }
}
