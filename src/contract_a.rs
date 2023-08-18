pub static mut STORAGE_A: i32 = 0;

extern "C" {
    fn execute_contract_b() -> i32;
}

#[no_mangle]
pub extern "C" fn execute_chained_calls() -> i32 {
    unsafe {
        let result = execute_contract_b();
        STORAGE_A += result;
        result
    }
}

#[no_mangle]
pub extern "C" fn get_storage_a() -> i32 {
    unsafe { STORAGE_A }
}
