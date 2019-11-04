#[derive(Default, Debug)]
struct ArrayEntry<T> {
    value: T,
    generation: u64,
}

// An array from GenerationalIndex to some Value T.
#[derive(Debug, Default)]
pub struct GenerationalIndexArray<T>(Vec<Option<ArrayEntry<T>>>);

impl<T> GenerationalIndexArray<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

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

pub type ComponentList<T> = GenerationalIndexArray<super::Component<T>>;
