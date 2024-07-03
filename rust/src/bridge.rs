#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("helloworld.h");
        
        type Internal;

        fn AllocateAndPrint() -> *mut Internal;
    }
}

pub use ffi::AllocateAndPrint;