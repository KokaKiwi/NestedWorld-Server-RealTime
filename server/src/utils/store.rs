use std::marker::PhantomData;
use std::mem;

#[derive(Debug, Clone)]
pub struct Store<T, I = usize> {
    entries: Vec<Slot<T>>,
    size: usize,
    next: usize,
    _index_marker: PhantomData<I>,
}

impl<T, I> Store<T, I> {
    pub fn new() -> Store<T, I> {
        Store {
            entries: Vec::new(),
            size: 0,
            next: 0,
            _index_marker: PhantomData,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T, I> Store<T, I> where I: From<usize> + Into<usize> {
    pub fn get(&self, index: I) -> Option<&T> {
        match self.entries.get(index.into()) {
            Some(&Slot::Occupied(ref value)) => Some(value),
            _ => None,
        }
    }

    pub fn get_mut(&mut self, index: I) -> Option<&mut T> {
        match self.entries.get_mut(index.into()) {
            Some(&mut Slot::Occupied(ref mut value)) => Some(value),
            _ => None,
        }
    }

    pub fn insert(&mut self, value: T) -> I {
        self.vacant_entry().insert(value).index()
    }

    pub fn entry(&mut self, index: I) -> Entry<T, I> {
        Entry {
            store: self,
            index: index.into(),
        }
    }

    pub fn vacant_entry(&mut self) -> VacantEntry<T, I> {
        let index = self.next;
        VacantEntry {
            store: self,
            index: index,
        }
    }

    pub fn reserve(&mut self, size: usize) {
        if size <= self.entries.len() { return; }

        let additional = size - self.entries.len();
        let next = self.entries.len() + 1;
        self.entries.extend((next..(next + additional)).map(Slot::Empty));
    }

    fn insert_at(&mut self, index: usize, value: T) {
        self.reserve(index + 1);

        // We know there's something at the specified index as we reserved enough slots.
        let slot = unsafe { self.entries.get_unchecked_mut(index) };
        self.next = match *slot {
            Slot::Empty(next) => next,
            Slot::Occupied(_) => panic!("Index already contains a value!"),
        };

        *slot = Slot::Occupied(value);
        self.size += 1;
    }

    fn replace(&mut self, index: usize, slot: Slot<T>) -> Option<T> {
        self.reserve(index + 1);

        let e = unsafe { self.entries.get_unchecked_mut(index) };
        if slot.is_empty() {
            self.next = index;

            if e.is_occupied() {
                self.size -= 1;
            }
        } else if e.is_occupied() {
            self.size += 1;
        }

        match mem::replace(e, slot) {
            Slot::Occupied(value) => Some(value),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
enum Slot<T> {
    Occupied(T),
    Empty(usize),
}

impl<T> Slot<T> {
    fn is_empty(&self) -> bool {
        match *self {
            Slot::Empty(_) => true,
            _ => false,
        }
    }

    fn is_occupied(&self) -> bool {
        !self.is_empty()
    }
}

pub struct Entry<'a, T: 'a, I: 'a> {
    store: &'a mut Store<T, I>,
    index: usize,
}

impl<'a, T, I> Entry<'a, T, I> where I: From<usize> + Into<usize> {
    pub fn replace(&mut self, value: T) -> Option<T> {
        self.store.replace(self.index, Slot::Occupied(value))
    }

    pub fn remove(self) -> Option<T> {
        let next = self.store.next;
        self.store.replace(self.index, Slot::Empty(next))
    }

    pub fn get(&self) -> &T {
        let index = self.index();
        self.store.get(index).expect("Filled slot in Entry")
    }

    pub fn get_mut(&mut self) -> &mut T {
        let index = self.index();
        self.store.get_mut(index).expect("Filled slot in Entry")
    }

    pub fn index(&self) -> I {
        I::from(self.index)
    }
}

pub struct VacantEntry<'a, T: 'a, I: 'a> {
    store: &'a mut Store<T, I>,
    index: usize,
}

impl<'a, T, I> VacantEntry<'a, T, I> where I: From<usize> + Into<usize> {
    pub fn insert(self, value: T) -> Entry<'a, T, I> {
        self.store.insert_at(self.index, value);

        Entry {
            store: self.store,
            index: self.index,
        }
    }

    pub fn index(&self) -> I {
        I::from(self.index)
    }
}

#[cfg(test)]
mod tests {
    use super::Store;

    #[test]
    fn test_store_push() {
        let mut store: Store<u8> = Store::new();

        let a = store.insert(10);
        let b = store.insert(10);
        let c = store.insert(10);

        assert_eq!(a, 0);
        assert_eq!(b, 1);
        assert_eq!(c, 2);

        store.entry(b).remove();

        let b = store.insert(10);

        assert_eq!(b, 1);
    }
}
