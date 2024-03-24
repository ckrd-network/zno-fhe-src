use zno::zno;
fn re_a<'minitrace>(
    n: i32,
) -> impl ::core::future::Future<Output = Result<impl Iterator<Item = i32>, ()>> + 'minitrace {
    zno::future::FutureExt::in_span(
        async move {
            {
                let n = n;
                Ok((0..10).filter(move |x| *x < n))
            }
        },
        zno::Span::enter_with_local_parent("re_a"),
    )
}
fn re_b<'minitrace>(
    n: i32,
) -> impl ::core::future::Future<Output = Result<impl Iterator<Item = i32>, &'static str>> + 'minitrace
{
    zno::future::FutureExt::in_span(
        async move {
            {
                Ok((0..10).filter(move |x| *x < n))
            }
        },
        zno::Span::enter_with_local_parent("err"),
    )
}
