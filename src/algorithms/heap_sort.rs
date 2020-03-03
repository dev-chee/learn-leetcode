fn max_heapify(heap: &mut [i32], i: usize) {
    let l = i * 2 + 1;
    let r = l + 1;

    let mut lag = i;

    if l < heap.len() && heap[l] > heap[lag] {
        lag = l;
    }

    if r < heap.len() && heap[r] > heap[lag] {
        lag = r;
    }

    if lag != i {
        heap.swap(i, lag);
        max_heapify(heap, lag);
    }
}

fn build_max_heap(heap: &mut [i32]) {
    for i in (0..heap.len() / 2).rev() {
        max_heapify(heap, i);
    }
}

pub fn heap_sort(heap: &mut [i32]) {
    build_max_heap(heap);

    let mut len = heap.len();
    for i in (1..len).rev() {
        heap.swap(0, i);
        max_heapify(&mut heap[..i], 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut a = [4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
        build_max_heap(&mut a);
        assert_eq!(a, [16, 14, 10, 8, 7, 9, 3, 2, 4, 1]);
    }

    #[test]
    fn test_1() {
        let mut a = [5, 3, 17, 10, 84, 19, 6, 22, 9];
        build_max_heap(&mut a);
        assert_eq!(a, [84, 22, 19, 10, 3, 17, 6, 5, 9]);
    }

    #[test]
    fn test_2() {
        let mut a = [4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
        heap_sort(&mut a);
        assert_eq!(a, [1, 2, 3, 4, 7, 8, 9, 10, 14, 16]);
    }

    #[test]
    fn test_3() {
        let mut a = [5, 3, 17, 10, 84, 19, 6, 22, 9];
        heap_sort(&mut a);
        assert_eq!(a, [3, 5, 6, 9, 10, 17, 19, 22, 84]);
    }
}