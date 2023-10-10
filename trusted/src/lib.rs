#![crate_name = "enclave"]
#![crate_type = "staticlib"]
#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

use sgx_types::*;
use std::{
    boxed::Box,
    io::{self, Write},
    slice,
};
use tree::{Node, Tree};

#[no_mangle]
pub extern "C" fn inclusion_proof(
    tree_ptr: u64,
    leaf_node_ptr: *const u8,
    leaf_node_count: usize,
    proof_ptr: *mut u8,
    proof_count: usize,
) -> sgx_status_t {
    let tree = unsafe { Box::from_raw(tree_ptr as *mut Tree) };

    let leaf = unsafe { std::slice::from_raw_parts(leaf_node_ptr, leaf_node_count) };
    let leaf = Node::from_iter(leaf.iter());

    let proof = tree.inclusion_proof(&leaf);

    let proof_ret = unsafe { std::slice::from_raw_parts_mut(proof_ptr, proof_count) };
    proof_ret.copy_from_slice(proof.as_slice());

    sgx_status_t::SGX_SUCCESS
}
