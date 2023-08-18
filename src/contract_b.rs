pub static mut STORAGE_B: i32 = 0;

extern "C" {
    fn execute_contract_c() -> i32;
}

#[no_mangle]
pub extern "C" fn execute_contract_b() -> i32 {
    unsafe {
        let result = execute_contract_c();
        STORAGE_B += result;
        result
    }
}

#[no_mangle]
pub extern "C" fn get_storage_b() -> i32 {
    unsafe { STORAGE_B }
}
