fn insertion_sort<T: std::cmp::PartialOrd>(arr: &mut [T]) {
    for i in 1..arr.len() {

        let mut j = i - 1;

        while (j >= 0) && (arr[j] > pivo) {
            arr[j + 1] = arr[j];
            j -= 1;
        }

        arr[j + 1] = pivo;
    }
}

fn main() {
    
}
