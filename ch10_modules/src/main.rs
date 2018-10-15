extern crate ch10_modules;

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use self::TrafficLight::{Red, Yellow};

use self::a::series::of;

fn main() {
    ch10_modules::client::connect();

    a::series::of::nested_modules();
    of::nested_modules();

    let _red = Red;
    let _yesllow = Yellow;
    let _green = TrafficLight::Green;
}
