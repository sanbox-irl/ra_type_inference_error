use super::GenerationalIndex;

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

    /// Set the value for some generational index.  May overwrite past generation
    /// values.
    pub fn set(&mut self, index: &GenerationalIndex, value: T) {
        self.0[index.index] = Some(ArrayEntry {
            value,
            generation: index.generation,
        });
    }

    /// Adds a new component for a new entity to the end of the List.
    pub fn expand_list(&mut self) {
        self.0.push(None);
    }

    /// Unsets the value for some generational index. Returns true if succesfully
    /// unset.
    #[allow(dead_code)]
    pub fn unset(&mut self, index: &GenerationalIndex) -> bool {
        let ret = &self.0[index.index];
        if let Some(ret) = ret {
            if ret.generation == index.generation {
                self.0[index.index] = None;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    // Gets the value for some generational index, the generation must match.
    #[allow(dead_code)]
    pub fn get(&self, index: &GenerationalIndex) -> Option<&T> {
        let ret = &self.0[index.index];
        if let Some(ret) = ret {
            if ret.generation == index.generation {
                Some(&ret.value)
            } else {
                None
            }
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, index: &GenerationalIndex) -> Option<&mut T> {
        let ret = &mut self.0[index.index];
        if let Some(ret) = ret {
            if ret.generation == index.generation {
                Some(&mut ret.value)
            } else {
                None
            }
        } else {
            None
        }
    }
}
