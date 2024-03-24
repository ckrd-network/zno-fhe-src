use zno::zno;

#[allow(unused_braces)]
#[zno]
#[warn(unused_braces)]
fn f(a: u64) {
    std::thread::sleep(std::time::Duration::from_millis(a));
}

fn main() {}
