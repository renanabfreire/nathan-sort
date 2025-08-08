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
        if pos == 0 || pos > self.atual_size + 1 {
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
    let mut exemplo = ArrayList::new(100);

    if exemplo.is_empty() {
        println!("It's empty");
    } else {
        println!("It isn't empty");
    }

    for i in 1..101 {
        exemplo.insert(i, i);
    }

    if exemplo.is_full() {
        println!("Is full");
    } else {
        println!("Not full");
    }

    println!("{}", exemplo.size());

    if let Some(v) = exemplo.get(50) {
        println!("{:#?}", v);
    } else {
        println!("None");
    }

    exemplo.change(101, 50);

    if let Some(v) = exemplo.get(50) {
        println!("{:#?}", v);
    } else {
        println!("None");
    }

    for _ in 0..100 {
        exemplo.remove(1);
    }

    if exemplo.is_empty() {
        println!("It's empty");
    } else {
        println!("It isn't empty");
    }
}
