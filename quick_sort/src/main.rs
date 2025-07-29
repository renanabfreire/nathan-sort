use std::cmp::PartialOrd;

fn hoare_partition<T: PartialOrd + Clone>(arr: &mut [T], start: usize, end: usize) -> usize {
    let pivo = arr[start].clone();

    let mut i = start;
    let mut j = end - 1;

    loop {
        while arr[i] < pivo {
            i += 1;
        }

        while arr[j] > pivo {
            j -= 1;
        }

        if i >= j {
            return j;
        }

        let temporario = arr[i].clone();
        arr[i] = arr[j].clone();
        arr[j] = temporario;

        i += 1;
        j -= 1;
    }
}

fn lomuto_partition<T: PartialOrd + Clone>(arr: &mut [T], start: usize, end: usize) -> usize {
    let pivo = arr[end - 1].clone();
    let mut i = start;

    for j in start..end - 1 {
        if arr[j] <= pivo {
            i += 1;

            let temp = arr[i - 1].clone();
            arr[i - 1] = arr[j].clone();
            arr[j] = temp;
        }
    }

    let temp = arr[end - 1].clone();
    arr[end - 1] = arr[i].clone();
    arr[i] = temp;

    i
}

fn hoare_quick_sort<T: PartialOrd + Clone>(arr: &mut [T], start: usize, end: usize) {
    if end - start > 1 {
        let q = hoare_partition(arr, start, end);
        hoare_quick_sort(arr, start, q + 1);
        hoare_quick_sort(arr, q + 1, end);
    }
}

fn lomuto_quick_sort<T: PartialOrd + Clone>(arr: &mut [T], start: usize, end: usize) {
    if end - start > 1 {
        let q = lomuto_partition(arr, start, end);
        lomuto_quick_sort(arr, start, q);
        lomuto_quick_sort(arr, q, end);
    }
}

fn main() {
    println!("Hoare");
    {
        let mut a = [4, 2, 3, 7, 1, 8, 9, 4, 3, 1];

        hoare_quick_sort(&mut a, 0, 10);

        for i in 0..a.len() {
            println!("{}", a[i]);
        }
    }

    println!("Lomuto");
    {
        let mut a = [4, 2, 3, 7, 1, 8, 9, 4, 3, 1];

        lomuto_quick_sort(&mut a, 0, 10);

        for i in 0..a.len() {
            println!("{}", a[i]);
        }
    }
}
