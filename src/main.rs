#[macro_use]
extern crate num_derive;
extern crate num_traits;

use num_traits::FromPrimitive;

mod other_file;

fn main() {
    assert_eq!(other_file::TestEnum::from_u32(1).unwrap(), other_file::TestEnum::B);
}
