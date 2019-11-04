#![warn(elided_lifetimes_in_paths)]

mod component;
mod entities;

pub use component::Component;
pub use entities::*;

fn main() {
    let list: ComponentList<f32> = unimplemented!();
    
    for this_member in list.iter() {

    }
}
