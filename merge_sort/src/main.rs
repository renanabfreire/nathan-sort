use num_traits::Bounded;
use std::cmp::PartialOrd;
use std::marker::Copy;

fn merge<T: PartialOrd + Clone + Bounded + Copy>(arr: &mut [T], p: usize, q: usize, r: usize) {
    let l = {
        let mut result: [T; 10] = [T::max_value(); 10];

        for i in 0..q - p {
            result[i] = arr[p + i].clone();
        }

        result
    };

    let m = {
        let mut result: [T; 10] = [T::max_value(); 10];

        for i in 0..r - q {
            result[i] = arr[q + i].clone();
        }

        result
    };

    let mut i = 0;
    let mut j = 0;

    for k in p..r {
        if l[i] <= m[j] {
            arr[k] = l[i];
            i += 1;
        } else {
            arr[k] = m[j];
            j += 1;
        }
    }
}

fn merge_sort<T: PartialOrd + Clone + Bounded + Copy>(arr: &mut [T], p: usize, r: usize) {
    if r - p > 1 {
        let q: usize = (p + r) / 2;

        merge_sort(arr, p, q);
        merge_sort(arr, q, r);

        merge(arr, p, q, r);
    }
}

fn main() {
    let mut a = [4, 2, 3, 7, 1, 8, 9, 4, 3, 1];

    merge_sort(&mut a, 0, 10);

    for i in 0..a.len() {
        println!("{}", a[i]);
    }
}
