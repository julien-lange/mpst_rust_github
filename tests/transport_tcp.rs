#![allow(clippy::type_complexity)]

mod tcp;

#[test]
fn transport_tcp() {
    tcp::binary::main();
    tcp::binary_fail::main();
}
