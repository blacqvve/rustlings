// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)


fn main() {
    call_me(10);
}

fn call_me(num:i32) {
    for i in 5..num {
        println!("Ring! Call number {}", i + 1);
    }
}
