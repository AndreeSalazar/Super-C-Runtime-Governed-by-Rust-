//! Build script for Super-C Runtime
//!
//! Links native C/C++ and CUDA libraries

fn main() {
    // Link native library
    println!("cargo:rustc-link-search=native=../native/build");
    println!("cargo:rustc-link-lib=static=super_c_native");

    // Link CUDA library (if feature enabled)
    #[cfg(feature = "cuda")]
    {
        println!("cargo:rustc-link-search=native=../cuda/build");
        println!("cargo:rustc-link-lib=static=super_c_cuda");
        println!("cargo:rustc-link-lib=cudart");
    }

    // Link ASM library (if feature enabled)
    #[cfg(feature = "asm")]
    {
        println!("cargo:rustc-link-search=native=../asm/build");
        println!("cargo:rustc-link-lib=static=super_c_asm");
    }

    // Rerun if native sources change
    println!("cargo:rerun-if-changed=../native/src");
    println!("cargo:rerun-if-changed=../cuda/src");
    println!("cargo:rerun-if-changed=../asm");
}
