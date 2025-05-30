fn main() {
    test();

    let mut dummy = 0;
    test_ice(&mut dummy);
}

#[ice_proc_macro::slow_warning()]
pub fn test() {}

#[ice_proc_macro::slow_warning()]
pub fn test_ice(dummy: &mut i32) {
    *dummy += 1;
}
