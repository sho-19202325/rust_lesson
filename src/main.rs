// mod ownership;
// mod stack_heap;
// mod vars;
// mod generics;
// mod lifetime;
// mod structs;
// mod enums;
// mod traits;
extern crate lib_demo;
mod debug;
mod error_handling;
mod unit_test;

fn main() {
    // println!("Hello, world!");
    // vars::run();
    // vars::sub_a::func_a();
    // vars::sub_b::func_b();
    // stack_heap::run()
    // ownership::run();
    // generics::run();
    // lifetime::run();
    // structs::run();
    // enums::run();
    // traits::run();
    error_handling::run();
    lib_demo::print_random_number();
    debug::run();
}
