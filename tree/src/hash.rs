use crate::utils::aligned_bytes_to_u256;
use aligned_cmov::{typenum::U8, A8Bytes, Aligned, ArrayLength, CMov, GenericArray, A8};
use semaphore::poseidon;
use typenum::U32;

pub struct PoseidonHash;

impl PoseidonHash {
    pub fn hash_node(left: &A8Bytes<U32>, right: &A8Bytes<U32>) -> A8Bytes<U32> {
        let left = aligned_bytes_to_u256(left);
        let right = aligned_bytes_to_u256(right);
        let hash: [u8; 32] = poseidon::hash2(left, right).to_be_bytes();
        A8Bytes::<U32>::from(Aligned(GenericArray::from(hash)))
    }
}
