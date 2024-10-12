use netsim_embed::MachineFn;
use std::sync::OnceLock;

static DATA: OnceLock<String> = OnceLock::new();

#[netsim_embed_macros::machine]
fn foo() {
    DATA.get_or_init(|| "got it".to_string());
}

fn main() {
    foo::call(());
    let data = DATA.get_or_init(|| "others".to_string());
    assert_eq!(data, "got it");
}
