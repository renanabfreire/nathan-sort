fn bubble_sort<T: std::cmp::PartialOrd + Clone>(arr: &mut [T]) {
    loop {
        let mut changed = false;
        for i in 0..(arr.len() - 1) {
            if arr[i] > arr[i + 1] {
                let pivo = arr[i].clone();
                arr[i] = arr[i + 1].clone();
                arr[i + 1] = pivo;

                changed = true;
            }
        }

        if !changed {
            break;
        }
    }
}

fn main() {
    let mut a = [4, 2, 3, 7, 1, 8, 9, 4, 3, 1];

    bubble_sort(&mut a);

    for i in 0..a.len() {
        println!("{}", a[i]);
    }
}
