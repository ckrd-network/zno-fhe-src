use zno::zno;

#[zno]
fn f(a: u64) {
    std::thread::sleep(std::time::Duration::from_millis(a));
}

fn main() {}
