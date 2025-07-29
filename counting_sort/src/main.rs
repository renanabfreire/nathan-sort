fn couting_sort(arr: &mut [usize]) {
    let mut c: [usize; 10] = [0; 10];
    let mut b: [usize; 20] = [0; 20];

    for j in 0..arr.len() {
        c[arr[j]] += 1;
    }

    for i in 1..10 {
        c[i] += c[i - 1];
    }

    for (_, el) in arr.iter().enumerate().rev() {
        b[c[*el] - 1] = *el;
        c[*el] -= 1;
    }

    arr.copy_from_slice(&b[..arr.len()]);
}

fn main() {
    let mut a = [4, 2, 3, 7, 1, 8, 9, 4, 3, 1];

    couting_sort(&mut a);

    for i in 0..a.len() {
        println!("{}", a[i]);
    }
}
