extern "C" {
    fn execute_contract_b() -> i32;
}

#[no_mangle]
pub extern "C" fn execute_chained_calls() -> i32 {
    unsafe { execute_contract_b() }
}
