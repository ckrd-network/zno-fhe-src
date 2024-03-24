use zno::zno;

#[zno(enter_on_poll=true)]
fn f(a: u32) -> u32 {
    a
}

fn main() {
    f(1);
}
