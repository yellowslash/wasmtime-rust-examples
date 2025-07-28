wit_bindgen::generate!({
    world: "component-a",
});

use exports::component::component_a::component_a_to_ext::Guest;
use component::component_a::component_b_to_ext;

struct Component;

impl Guest for Component {
    fn main() {
        let r = component_b_to_ext::say_hi("Hi");
        println!("component-a -> {}", r.message);
    }
}

export!(Component);