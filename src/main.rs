/*
This is a proof-of-concept for PLC code online change in Rust
Written by Rainer Kordmaa, MIT licence, do whatever you want with it.

demonstrates only function pointers,

for variable access, rust should probably also copy memory contents to it's own memory to persist data across online change
and handle cases if variables are added/removed between verisons
*/

use libloading::{Library, Symbol};

fn call_main(got_ptr: *mut *const [*const u8; 2]) -> i32 {
    unsafe {
        let got_base = **got_ptr;
        let main_slot = got_base[0];
        let main_raw = main_slot;
        let main_fn: extern "C" fn() -> i32 = std::mem::transmute(main_raw);
        main_fn()
    }
}

fn main() {
    // Load project1 library
    let lib1 = unsafe { Library::new("./project1.so").unwrap() };

    // Get symbols from project1
    let custom_got_pointer: Symbol<'_, *mut *const [*const u8; 2]> = unsafe { lib1.get(b"__custom_got_pointer").unwrap() };
    let main_sym: Symbol<'_, extern "C" fn() -> i32> = unsafe { lib1.get(b"main").unwrap() };
    let foo_sym: Symbol<'_, extern "C" fn() -> i32> = unsafe { lib1.get(b"foo").unwrap() };

    // Call main by addressing it through __custom_got_pointer
    let result1 = call_main(*custom_got_pointer);
    println!("First call result before got handover to rust: {}", result1);

    // Create an identical GOT table in Rust
    let mut rust_got: [*const u8; 2] = [
        *main_sym as *const u8,
        *foo_sym as *const u8,
    ];

    // Update __custom_got_pointer in project1 with location of the new GOT
    unsafe { **custom_got_pointer = &rust_got; }

    // Call main again the same way
    let result2 = call_main(*custom_got_pointer);
    println!("Second call result after got handover to rust: {}", result2);

    // Load project2 library
    let lib2 = unsafe { Library::new("./project2.so").unwrap() };

    // Load same symbols from project2
    let main_sym2: Symbol<'_, extern "C" fn() -> i32> = unsafe { lib2.get(b"main").unwrap() };
    let foo_sym2: Symbol<'_, extern "C" fn() -> i32> = unsafe { lib2.get(b"foo").unwrap() };

    // Update GOT table in Rust project with loaded symbols from project2
    rust_got[0] = *main_sym2 as *const u8;
    rust_got[1] = *foo_sym2 as *const u8;

    // Update __custom_got_pointer in project2
    let custom_got_pointer2: Symbol<'_, *mut *const [*const u8; 2]> = unsafe { lib2.get(b"__custom_got_pointer").unwrap() };
    unsafe { **custom_got_pointer2 = &rust_got; };

    // Call main again the same way
    let result3 = call_main(*custom_got_pointer);
    println!("Third call result after loading new version of library: {}", result3);
}
