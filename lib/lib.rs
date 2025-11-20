extern "C" {
    fn lib_function();
}

pub fn do_work() {
    unsafe {
        lib_function();
    }
    println!("Library doing work");
}
