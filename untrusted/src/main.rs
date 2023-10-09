extern crate sgx_types;
extern crate sgx_urts;

use rand::thread_rng;
use sgx_types::*;
use sgx_urts::SgxEnclave;
use tree::{utils::random_tree, Node, Tree};

static ENCLAVE_FILE: &'static str = "enclave.signed.so";
static mut TREE_PTR: *mut Tree = std::ptr::null_mut();
static DEPTH: usize = 16;

extern "C" {
    fn inclusion_proof(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        tree_ptr: u64,
        leaf_node_ptr: *const u8,
        leaf_node_count: usize,
        proof_ptr: *const u8,
        proof_count: usize,
    ) -> sgx_status_t;
}

fn init_enclave() -> SgxResult<SgxEnclave> {
    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    // call sgx_create_enclave to initialize an enclave instance
    // Debug Support: set 2nd parameter to 1
    let debug = 1;
    let mut misc_attr = sgx_misc_attribute_t {
        secs_attr: sgx_attributes_t { flags: 0, xfrm: 0 },
        misc_select: 0,
    };
    SgxEnclave::create(
        ENCLAVE_FILE,
        debug,
        &mut launch_token,
        &mut launch_token_updated,
        &mut misc_attr,
    )
}

pub fn update_tree(index: usize, value: Node) {
    unsafe {
        (TREE_PTR.as_mut().unwrap()).update(index, value);
    }
}

pub fn generate_inclusion_proof(eid: sgx_enclave_id_t, leaf: Node) {
    let leaf_node_ptr = leaf.as_ptr();
    let leaf_node_count = 32;

    let mut proof = vec![Node::default(); DEPTH];
    let proof_ptr = proof.as_mut_ptr() as *mut u8;
    let proof_count = 32 * DEPTH;

    let mut ret_val = sgx_status_t::SGX_SUCCESS;
    let ret_status = unsafe {
        inclusion_proof(
            eid,
            &mut ret_val,
            TREE_PTR as u64,
            leaf_node_ptr,
            leaf_node_count,
            proof_ptr,
            proof_count,
        )
    };

    // check status

    // return proof
}

fn main() {
    // Create tree
    let mut rng = thread_rng();
    let tree = Box::new(random_tree(DEPTH, 10, &mut rng));
    let tree_ptr = Box::into_raw(tree);
    unsafe { TREE_PTR = tree_ptr };

    let enclave = match init_enclave() {
        Ok(r) => {
            println!("[U] Init enclave success {}!", r.geteid());
            r
        }
        Err(r) => {
            println!("[U] Init enclave failed with {}!", r);
            return;
        }
    };

    let _get_dropped = unsafe { Box::from_raw(TREE_PTR) };

    enclave.destroy();
}
