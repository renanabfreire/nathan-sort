struct Node<T: Clone> {
    contend: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Clone> Node<T> {
    fn new(data: T) -> Self {
        Self {
            contend: data,
            next: None,
        }
    }

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
}

fn main() {}
