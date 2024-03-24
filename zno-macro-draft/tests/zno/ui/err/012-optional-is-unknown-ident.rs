use zno::zno;

#[zno(name = "a", some_unknown = true)]
fn f() {}

fn main() {
    f();
}
