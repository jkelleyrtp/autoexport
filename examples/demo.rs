use autoexport::inline_props;

#[inline_props]
pub mod things {
    pub mod inner_thing {
        pub mod super_inner_thing {
            pub fn inner_thing() {
                println!("Hello, world!");
            }
        }
    }
}

fn main() {
    inner_thing()
}
