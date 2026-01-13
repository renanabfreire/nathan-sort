use num_traits::Bounded;

fn merge<T: PartialOrd + Clone + Bounded>(arr: &mut [T]) {
    let mid = arr.len() / 2;

    let mut l: Vec<T> = arr[..mid].to_vec();
    let mut m: Vec<T> = arr[mid..].to_vec();

    // adiciona sentinelas
    l.push(T::max_value());
    m.push(T::max_value());

    let mut i = 0;
    let mut j = 0;

    for k in 0..arr.len() {
        if l[i] <= m[j] {
            arr[k] = l[i].clone();
            i += 1;
        } else {
            arr[k] = m[j].clone();
            j += 1;
        }
    }
}

fn merge_sort<T: PartialOrd + Clone + Bounded>(arr: &mut [T]) {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);
        merge(arr);
    }
}

fn main() {
    let mut a = [4, 2, 3, 7, 1, 8, 9, 4, 3, 1];
    merge_sort(&mut a);

    for x in &a {
        println!("{}", x);
    }
}
