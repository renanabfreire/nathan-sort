fn insertion_sort<T: std::cmp::PartialOrd + Clone>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let pivo = arr[i].clone();

        let mut j = i;

        while (j > 0) && (arr[j - 1] > pivo) {
            arr[j] = arr[j - 1].clone();
            j -= 1;
        }

        arr[j] = pivo;
    }
}

fn main() {
    let mut a = [4, 2, 3, 7, 1, 8, 9, 4, 3, 1];

    insertion_sort(&mut a);

    for i in 0..a.len() {
        println!("{}", a[i]);
    }
}
