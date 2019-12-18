pub fn main(v: &Vec<i32>) -> Vec<i32> {
    let mut v2: Vec<i32> = v.clone();
    for i in 0..v2.len() {
        let min_index: usize = get_min(&v2, i);
        v2.swap(i, min_index);
    }
    return v2;
}

fn get_min(v: &Vec<i32>, left_index: usize) -> usize {
    let mut min_index: usize = left_index;
    for i in left_index..v.len() {
        if v[min_index] > v[i] {
            min_index = i;
        }
    }
    return min_index;
}
