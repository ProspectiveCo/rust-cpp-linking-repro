mod bridge;

#[no_mangle]
pub extern "C" fn foo() -> i32 {
    bridge::AllocateAndPrint();
    69 // lol
}
