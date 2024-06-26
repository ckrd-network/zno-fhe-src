use zno::zno;
use test_utilities::*;

// With no block expression the span "a-span" is silently omitted.
// Reference:
// - https://github.com/tikv/minitrace-rust/issues/125
// - https://github.com/tikv/minitrace-rust/issues/126
#[zno( name = "a-span")]
fn f(a: u32) -> u32 {
    a
}

fn main() {
    let (root, collector) = zno::Span::root("root");
    //{
    let _child_span = root.set_local_parent();
    f(1);
    //}
    //drop(child_span); //This is required when not using `{ ... }`
    drop(root);
    let records: Vec<zno::collector::SpanRecord> =
        futures::executor::block_on(collector.collect());
    let expected = r#"[
    SpanRecord {
        id: 1,
        parent_id: 0,
        begin_unix_time_ns: \d+,
        duration_ns: \d+,
        event: "root",
        properties: [],
    },
]"#;
    let actual = normalize_spans(records);
    assert_eq_text!(expected, &actual);
}
