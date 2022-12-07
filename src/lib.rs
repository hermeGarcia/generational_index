#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct Index {
    value: usize,
    generation: usize,
}
impl Index {
    fn new(value: usize, generation: usize) -> Index {
        Index { value, generation }
    }
}

#[derive(Clone, Debug)]
struct AllocatorEntry {
    is_live: bool,
    generation: usize,
}
impl AllocatorEntry {
    fn new() -> AllocatorEntry {
        AllocatorEntry {
            is_live: true,
            generation: 0,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Allocator {
    entries: Vec<AllocatorEntry>,
    free: Vec<usize>,
}
impl Allocator {
    pub fn new() -> Allocator {
        Allocator::default()
    }
    pub fn allocate(&mut self) -> Index {
        match self.free.pop() {
            Some(i) => {
                self.entries[i].generation += 1;
                self.entries[i].is_live = true;
                Index::new(i, self.entries[i].generation)
            }
            None => {
                let index = self.entries.len();
                self.entries.push(AllocatorEntry::new());
                Index::new(index, self.entries[index].generation)
            }
        }
    }
    pub fn deallocate(&mut self, index: Index) {
        if self.is_live(index) {
            self.entries[index.value].is_live = false;
            self.free.push(index.value);
        }
    }
    pub fn is_live(&self, index: Index) -> bool {
        self.entries
            .get(index.value)
            .map(|v| v.is_live)
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
struct ArrayEntry<E> {
    value: E,
    generation: usize,
}
impl<E> ArrayEntry<E> {
    fn value(&self, index: Index) -> Option<&E> {
        if self.generation == index.generation {
            Some(&self.value)
        } else {
            None
        }
    }
    fn value_mut(&mut self, index: Index) -> Option<&mut E> {
        if self.generation == index.generation {
            Some(&mut self.value)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Arena<E> {
    values: Vec<Option<ArrayEntry<E>>>,
}
impl<E> std::ops::Index<Index> for Arena<E> {
    type Output = E;
    fn index(&self, index: Index) -> &Self::Output {
        self.get(index).unwrap()
    }
}
impl<E> std::ops::IndexMut<Index> for Arena<E> {
    fn index_mut(&mut self, index: Index) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
impl<E: Default> Default for Arena<E> {
    fn default() -> Self {
        Arena::new()
    }
}
impl<E> Arena<E> {
    pub fn new() -> Arena<E> {
        Arena { values: vec![] }
    }
    pub fn allocate(&mut self, index: Index, elem: E) {
        while self.len() <= index.value {
            self.values.push(None);
        }
        if self.values[index.value].is_none() {
            self.values[index.value] = Some(ArrayEntry {
                value: elem,
                generation: index.generation,
            })
        }
    }
    pub fn deallocate(&mut self, index: Index) {
        if self.get(index).is_some() {
            self.values[index.value] = None;
        }
    }
    pub fn set(&mut self, index: Index, elem: E) {
        if let Some(v) = self.get_mut(index) {
            *v = elem
        }
    }
    pub fn get(&self, index: Index) -> Option<&E> {
        self.values
            .get(index.value)
            .map(|v| v.as_ref())
            .flatten()
            .and_then(|v| v.value(index))
    }
    pub fn get_mut(&mut self, index: Index) -> Option<&mut E> {
        self.values
            .get_mut(index.value)
            .map(|v| v.as_mut())
            .flatten()
            .and_then(|v| v.value_mut(index))
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
}
