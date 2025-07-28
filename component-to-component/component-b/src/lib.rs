wit_bindgen::generate!({
    path: "wit",
    world: "component-b",
});

use exports::component::component_a::component_b_to_ext::{Guest, HiMessage};
struct Component;

impl Guest for Component {
    /// Say hello!
    fn say_hi(name: String) -> HiMessage {
        let msg = format!("Hi there, {name}");
        let m = HiMessage {
            message: msg,
            is_it_a_hi: true
        };
        return m
    }
}

export!(Component);
