use zno::zno;

#[zno(enter_on_poll = true, name = "b")]
async fn f(a: u32) -> u32 {
    a
}

#[tokio::main]
async fn main() {
    f(1).await;
}
