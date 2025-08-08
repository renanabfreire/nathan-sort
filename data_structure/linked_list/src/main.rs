struct Node<T: Clone> {
    contend: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Clone> Node<T> {
    fn get(&self) -> T {
        self.contend.clone()
    }
}

struct LinkedList<T: Clone> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: Clone> LinkedList<T> {
    fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }

    fn get(&self, pos: usize) -> Option<T>
    where
        T: Clone,
    {
        if self.is_empty() || pos == 0 || pos > self.size {
            None
        } else {
            let mut cont = 1;
            let mut cur = self.head.as_ref();

            while cont < pos {
                cur = cur?.next.as_ref();
                cont += 1;
            }

            cur.map(|node| node.get())
        }
    }

    fn get_pos(&self, data: T) -> Option<usize>
    where
        T: Clone + PartialEq,
    {
        let mut cur = self.head.as_ref();
        let mut cont: usize = 1;

        while let Some(node) = cur {
            if node.contend == data {
                return Some(cont);
            }

            cur = node.next.as_ref();
            cont += 1;
        }

        None
    }

    fn insert_beg(&mut self, data: T) -> bool {
        let new = Box::new(Node {
            contend: data,
            next: self.head.take(),
        });

        self.head = Some(new);

        self.size += 1;

        true
    }

    fn insert_end(&mut self, data: T) -> bool {
        let new = Box::new(Node {
            contend: data,
            next: None,
        });

        let mut cur = self.head.as_mut();

        while let Some(node) = cur {
            if node.next.is_none() {
                node.next = Some(new);
                break;
            }

            cur = node.next.as_mut();
        }

        self.size += 1;

        true
    }

    fn insert_middle(&mut self, pos: usize, data: T) -> bool {
        if pos < 1 || pos > self.size + 1 {
            false
        } else {
            let mut cur = self.head.as_mut();

            let mut cont = 1usize;

            while cont < pos - 1 {
                if let Some(node) = cur {
                    cur = node.next.as_mut();
                    cont += 1;
                }
            }

            if let Some(node) = cur {
                let new = Box::new(Node {
                    contend: data,
                    next: node.next.take(),
                });

                node.next = Some(new);
            }

            self.size += 1;

            true
        }
    }

    fn insert(&mut self, pos: usize, data: T) -> bool {
        if self.is_empty() && pos != 1 {
            false
        } else if pos == 1 {
            self.insert_beg(data)
        } else if pos == self.size + 1 {
            self.insert_end(data)
        } else {
            self.insert_middle(pos, data)
        }
    }

    fn remove_beg(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        } else {
            let pivot = self.head.take()?;

            self.head = pivot.next;

            self.size -= 1;

            Some(pivot.contend)
        }
    }

    fn remove_pos(&mut self, pos: usize) -> Option<T> {
        if pos < 1 || pos > self.size() {
            None
        } else {
            let mut cur = self.head.as_mut()?;
            let mut cont = 1;

            while cont < pos - 1 {
                cur = cur.next.as_mut()?;
                cont += 1;
            }

            let mut removed = cur.next.take()?;

            cur.next = removed.next.take();

            self.size -= 1;

            Some(removed.contend)
        }
    }

    fn remove(&mut self, pos: usize) -> Option<T> {
        if self.is_empty() || pos < 1 || pos > self.size() {
            None
        } else if pos == 1 {
            self.remove_beg()
        } else {
            self.remove_pos(pos)
        }
    }
}

fn main() {
    let mut lista = LinkedList::new();

    println!(
        "List {} empty",
        if lista.is_empty() { "is" } else { "is not" }
    );
    println!("Size: {}", lista.size());

    println!("\nInsert 10 at position 1 (begning)...");
    if lista.insert(1, 10) {
        println!("Insertion well done!");
    } else {
        println!("Fail to insert");
    }

    println!("Position 1: {:?}", lista.get(1));
    println!("Element 10: {:?}", lista.get_pos(10));

    println!("Size: {}", lista.size());

    println!("\nInserting 30 at position 2...");
    if lista.insert(2, 30) {
        println!("Insertion well done");
    } else {
        println!("Failed on insertion");
    }
    println!("2nd positon element: {:?}", lista.get(2));
    println!("30 is in position:   {:?}", lista.get_pos(30));
    println!("Size: {}", lista.size());

    println!("\nInserting 20 on the middle...");
    if lista.insert(2, 20) {
        println!("insertion well done");
    } else {
        println!("Failed to insert");
    }
    println!("2nd positon element: {:?}", lista.get(2));
    println!("20 is in position:   {:?}", lista.get_pos(20));
    println!("Last positon element: {:?}", lista.get(lista.size()));
    println!("Size: {}", lista.size());

    println!("\nTrying to access unexistent position (5)...");
    println!("{:?}\n", lista.get(5));

    println!("\nRemoving first element");
    let removed = lista.remove(1);
    if let Some(element) = removed {
        println!("Element removed: {}", element);
    } else {
        println!("Failed to remove");
    }
    println!("\nNew size: {}", lista.size());
    println!("New first element: {:?}", lista.get(1));

    println!("\nRemoving last element");
    let removed = lista.remove(2);
    if let Some(element) = removed {
        println!("Element removed: {}", element);
    } else {
        println!("Failed to remove");
    }
    println!("\nNew size after removing: {}", lista.size());

    println!("\nTrying to remove ivalid element (position 5)");
    println!("{:?}", lista.remove(5));

    println!("\nThe list at this point:");
    for i in 1..=lista.size() {
        println!("Position {i}: {:?}", lista.get(i));
    }
}
