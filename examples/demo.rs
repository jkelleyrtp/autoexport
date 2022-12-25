#![allow(unused)]

use autoexport::export;

#[export]
pub mod things {
    pub mod inner_thing {
        pub mod super_inner_thing {
            pub fn inner_thing() {
                println!("Hello, world!");
            }

            fn inner_thing3() {
                println!("Hello, world!");
            }

            mod some_other_inner {
                pub fn inner_thing2() {
                    println!("Hello, world!");
                }
            }
        }
    }
}

fn main() {
    inner_thing();
}
