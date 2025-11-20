extern "C" {
    fn app_function();
}

fn main() {
    unsafe {
        app_function();
    }
    println!("App starting");
    mylib::do_work();

    println!("If you see this, the collision was avoided!");
}
