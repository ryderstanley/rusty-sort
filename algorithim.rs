fn selection_sort<T: Ord>(list: &mut [i64]) {
    for i in 0..list.len() {
        if let Some((j, _)) = list.iter()
                .enumerate()
                .skip(i)
                .min_by_key(|x| x.1) {
        list.swap(small, i);
    }
}
list.iter():                [4, 3, 1, 2]
list.iter().enumerate():    [(0,4), (1, 3), (2,1), (3,2)]
