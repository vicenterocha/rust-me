use std::sync::RwLock;

fn main() {
    let optional = RwLock::new(Some(123));
    let lock = optional.read().unwrap();
    let lock2 = optional.write().unwrap();
    eprintln!("Finished!");
}