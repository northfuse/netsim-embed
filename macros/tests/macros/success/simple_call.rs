use netsim_embed::MachineFn;
use std::sync::OnceLock;

static DATA: OnceLock<String> = OnceLock::new();

#[netsim_embed_macros::machine]
fn foo(bar: &'static str) {
    DATA.get_or_init(|| format!("got {bar}"));
}

fn main() {
    foo::call("hello world");
    let data = DATA.get_or_init(|| "others".to_string());
    assert_eq!(data, "got hello world");
}
