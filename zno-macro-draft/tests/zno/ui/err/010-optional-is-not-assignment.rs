use zno::zno;

#[zno(name = "a", enter_on_poll)]
fn f() {}

fn main() {
    f();
}
