use zno::zno;

#[zno(name = "a", enter_on_poll = y)]
fn f() {}

fn main() {
    f();
}
