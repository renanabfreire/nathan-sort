fn insertion_sort(arr: &mut [fsize]) {
    pivo = null;
    let mut j;

    for i in range(1..arr.len()) {
        pivo = arr[i];

        j = i - 1;

        while (j >= 0) && (A[j] > pivo) {
            A[j + 1] = A[j];
            j -= 1;
        }

        A[j + 1] = pivo;
    }
}

fn main() {}
