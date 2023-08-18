pub static mut STORAGE_C: i32 = 0;

#[no_mangle]
pub extern "C" fn execute_contract_c() -> i32 {
    // 42: The "Answer to the Ultimate Question of Life, the Universe, and Everything"
    unsafe {
        STORAGE_C += 42;
        STORAGE_C
    }
}

#[no_mangle]
pub extern "C" fn get_storage_c() -> i32 {
    unsafe { STORAGE_C }
}
