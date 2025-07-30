use std::cmp::PartialOrd;

fn max_heapify<T: PartialOrd + Clone>(arr: &mut [T], i : usize, tamanho : usize) {
    let l = 2 * i + 1;
    let r = 2 * i + 2;

    let mut maior = i;

    if (l < tamanho) && arr[l] > arr[maior] {
        maior = l;
    }

    if (r < tamanho) && arr[r] > arr[maior] {
        maior = r;
    }

    if maior != i {
        let temp = arr[i].clone();
        arr[i] = arr[maior].clone();
        arr[maior] = temp;

        max_heapify(arr, maior, tamanho);
    }
}

fn build_max_heap<T: PartialOrd + Clone>(arr: &mut [T]) {
    for i in (0..(arr.len()/2)).rev() {
        max_heapify(arr, i, arr.len());
    }
}

fn heap_sort<T: PartialOrd + Clone> (arr: &mut [T]) {
    let mut heap_size = arr.len();
    
    build_max_heap(arr);

    for i in (0..arr.len()).rev() {
        let temp = arr[0].clone();
        arr[0] = arr[i].clone();
        arr[i] = temp;
        
        heap_size -= 1;

        max_heapify(arr, 0, heap_size);
    }
}

fn main() {
    let mut a = [4, 2, 3, 7, 1, 8, 9, 4, 3, 1];

    heap_sort(&mut a);

    for i in 0..a.len() {
        println!("{}", a[i]);
    }
}
