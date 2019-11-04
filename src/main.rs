#![warn(elided_lifetimes_in_paths)]

fn main() {
    let list: ComponentList<f32> = unimplemented!();

    for this_member in list.iter() {}
}

struct ArrayEntry<T> {
    value: T,
}

// An array from GenerationalIndex to some Value T.
pub struct GenerationalIndexArray<T>(Vec<Option<ArrayEntry<T>>>);

impl<T> GenerationalIndexArray<T> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.0
            .iter_mut()
            .enumerate()
            .flat_map(|opt_ent| Some(&mut opt_ent.1.as_mut()?.value))
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|opt_ent| Some(&opt_ent.1.as_ref()?.value))
    }
}

pub type ComponentList<T> = GenerationalIndexArray<T>;
