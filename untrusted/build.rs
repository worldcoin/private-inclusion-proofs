use std::env;

fn main() {
    let sdk_dir = env::var("SGX_SDK").unwrap_or_else(|_| "/opt/sgxsdk".to_string());
    let is_sim = env::var("SGX_MODE").unwrap_or_else(|_| "HW".to_string());

    println!("cargo:rustc-link-search=native=../lib");
    println!("cargo:rustc-link-lib=static=Enclave_u");

    // https://github.com/intel/linux-sgx/issues/652
    // In simulation mode build scripts fails to search for libsgx_uae_service_sim.a (a dependency of libsgx_urts_sim.a)
    // in $SGX_SDK/lib64. For compiler to be able to fine the path, either source `$SGX_SDK/environment` before run or
    // source it in ~/.bashrc.
    println!("cargo:rustc-link-search=native={}/lib64", sdk_dir);
    match is_sim.as_ref() {
        "SW" => println!("cargo:rustc-link-lib=dylib=sgx_urts_sim"),
        "HW" => println!("cargo:rustc-link-lib=dylib=sgx_urts"),
        _ => println!("cargo:rustc-link-lib=dylib=sgx_urts"), // Treat undefined as HW
    }
}
