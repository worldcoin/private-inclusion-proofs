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
    println, slice,
};
use tree::{
    aligned_cmov::{typenum::U32, Aligned, GenericArray, A8},
    Tree,
};

#[no_mangle]
pub extern "C" fn inclusion_proof(
    tree_ptr: u64,
    leaf_node_ptr: *const u8,
    leaf_node_count: usize,
    proof_ptr: *mut u8,
    proof_count: usize,
) -> sgx_status_t {
    let leaf_slice = unsafe { std::slice::from_raw_parts(leaf_node_ptr, leaf_node_count) };
    let mut leaf = GenericArray::default();
    leaf.clone_from_slice(leaf_slice);
    let leaf = Aligned(leaf);

    let proof = unsafe {
        let proof = (tree_ptr as *mut Tree)
            .as_ref()
            .unwrap()
            .inclusion_proof(&leaf);
        proof
    };

    // println!();
    // println!("[T] Proof generated!");
    // proof.iter().for_each(|p| {
    //     println!("[T] Proof node: {:?}", p);
    // });
    // println!();

    let src_proof_ptr = proof.as_ptr() as *const u8;
    unsafe { std::ptr::copy(src_proof_ptr, proof_ptr, proof_count) };

    // println!("[T] Proof returning!");

    sgx_status_t::SGX_SUCCESS
}
