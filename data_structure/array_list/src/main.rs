struct ArrayList<T: Clone + Default> {
    atual_size: usize,
    list_size: usize,
    list: Vec<T>,
}

impl<T: Clone + Default> ArrayList<T> {
    fn new(size: usize) -> Self {
        Self {
            atual_size: 0,
            list_size: size,
            list: vec![T::default(); size],
        }
    }

    fn is_empty(&self) -> bool {
        self.atual_size == 0
    }

    fn is_full(&self) -> bool {
        self.atual_size == self.list_size
    }

    fn size(&self) -> usize {
        self.atual_size
    }

    fn get(&self, pos: usize) -> Option<T>
    where
        T: Clone,
    {
        if pos > self.atual_size || pos == 0 {
            None
        } else {
            Some(self.list[pos - 1].clone())
        }
    }

    fn change(&mut self, data: T, pos: usize) -> Option<T>
    where
        T: Clone,
    {
        if self.is_full() || pos == 0 || pos > self.atual_size + 1 {
            None
        } else {
            let deleted = self.list[pos - 1].clone();
            self.list[pos - 1] = data;

            Some(deleted)
        }
    }

    fn insert(&mut self, data: T, pos: usize) -> bool {
        if self.is_full() || pos > self.atual_size + 1 || pos == 0 {
            false
        } else {
            for i in (pos..=self.atual_size).rev() {
                self.list[i] = self.list[i - 1].clone();
            }

            self.list[pos - 1] = data;
            self.atual_size += 1;
            true
        }
    }

    fn remove(&mut self, pos: usize) -> Option<T>
    where
        T: Clone,
    {
        if pos > self.atual_size || pos == 0 {
            None
        } else {
            let deleted = self.list[pos - 1].clone();

            for i in (pos - 1)..(self.atual_size - 1) {
                self.list[i] = self.list[i + 1].clone();
            }

            self.atual_size -= 1;

            Some(deleted)
        }
    }
}

fn main() {
    println!("Hello, world!");
}
