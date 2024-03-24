use zno::zno;

#[zno(name = "test_span", enter_on_poll = true)]
async fn f(a: u32) -> u32 {
    a
}

#[tokio::main]
async fn main() {
    f(1).await;
}
