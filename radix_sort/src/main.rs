fn ordena_por_digito(arr: &mut [usize], dez: usize) {
    let mut value: usize = 1;
    let mut chest: [[usize; 11]; 10] = [[0; 11]; 10];

    for _ in 1..dez {
        value *= 10;
    }

    for &item in arr.iter() {
        let i = (item / value) % 10;
        chest[i][chest[i][0] + 1] = item;
        chest[i][0] += 1;
    }

    let mut cont = 0;
    for i in 1..10 {
        for j in 1..=chest[i][0] {
            arr[cont] = chest[i][j];
            cont += 1;
        }
    }
}

fn radix_sort(arr: &mut [usize], digito: usize) {
    for i in 0..digito {
        ordena_por_digito(arr, i);
    }
}

fn main() {
    let mut a = [4, 2, 3, 7, 1, 8, 9, 4, 3, 1];

    radix_sort(&mut a, 1);

    for i in 0..a.len() {
        println!("{}", a[i]);
    }
}
