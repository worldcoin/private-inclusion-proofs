use sgx_types::*;
use sgx_urts::SgxEnclave;
use tree::{Tree, utils::random_tree};

static ENCLAVE_FILE: &'static str = "enclave.signed.so";
static TREE_PTR: *mut Tree = std::ptr::null_mut();
static DEPTH: usize = 16;

extern "C" {
    fn set_ptr(eid: sgx_enclave_id_t, retval: *mut sgx_status_t, ptr: *mut i32) -> sgx_status_t;
    fn print_val(eid: sgx_enclave_id_t, retval: *mut sgx_status_t) -> sgx_status_t;
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

fn main() {
    // Create tree
    let mut rng = thread_rng();
    let tree = Box::new(random_tree(DEPTH, 10,&mut rng));


    let encalve = match init_enclave() { 
        Ok(r) => {
            println!("[U] Init enclave success {}!", r.geteid());
            r
        }
        Err(r) => { 
            println!("[U] Init enclave failed with {}!", r);
            return;
        }
    }

}
