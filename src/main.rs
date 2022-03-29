
#[link(name="lemitter.so")]
extern {
    fn test();
}

fn main() {
    unsafe {
        test();
    }
}
