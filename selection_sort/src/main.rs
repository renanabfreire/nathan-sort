fn selection_sort<T: std::cmp::PartialOrd + Clone>(arr: &mut [T]) {
    for i in 0..(arr.len() - 1) {
        let mut i_min = i;

        for j in (i + 1)..arr.len() {
            if arr[j] < arr[i_min] {
                i_min = j;
            }
        }

        if i != i_min {
            let temp = arr[i].clone();
            arr[i] = arr[i_min].clone();
            arr[i_min] = temp;
        }
    }
}

fn main() {
    let mut a = [4, 2, 3, 7, 1, 8, 9, 4, 3, 1];

    selection_sort(&mut a);

    for i in 0..a.len() {
        println!("{}", a[i]);
    }
}
