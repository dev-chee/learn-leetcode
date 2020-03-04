pub fn heap_max(heap: &[i32]) -> i32 {
    heap[0]
}

pub fn heap_extract_max(heap: &mut [i32]) -> i32 {
    let max = heap_max(heap);

    let end = heap.len() - 1;
    heap.swap(0, end);
    max_heapify(&mut heap[0..end], 0);

    max
}

pub fn heap_increase_key(heap: &mut [i32], idx: usize, key: i32) -> Result<(), &'static str> {
    if key < heap[idx] {
        return Err("New key is smaller than current key");
    }

    let mut idx = idx;

    heap[idx] = key;

    let mut parent = (idx - 1) / 2;
    while heap[parent] < heap[idx] {
        heap.swap(parent, idx);
        idx = parent;
        parent = (idx - 1) / 2;
    }

    Ok(())
}

pub fn max_heap_insert(heap: &mut Vec<i32>, key: i32) -> Result<(), &'static str> {
    heap.push(std::i32::MIN);
    let idx = heap.len() - 1;
    heap_increase_key(heap, idx, key)
}

fn max_heapify(heap: &mut [i32], idx: usize) {
    let left = 2 * idx + 1;
    let right = left + 1;
    let mut larg = idx;

    if left < heap.len() && heap[left] > heap[larg] {
        larg = left;
    }

    if right < heap.len() && heap[right] > heap[larg] {
        larg = right;
    }

    if larg != idx {
        heap.swap(idx, larg);
        max_heapify(heap, larg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut heap = [15, 13, 9, 5, 12, 8, 7, 4, 0, 6, 2, 1];
        assert_eq!(heap_extract_max(&mut heap), 15);
    }

    #[test]
    fn test2() {
        let mut heap = vec![15, 13, 9, 5, 12, 8, 7, 4, 0, 6, 2, 1];
        max_heap_insert(&mut heap, 10).unwrap();
        assert_eq!(heap, vec![15, 13, 10, 5, 12, 9, 7, 4, 0, 6, 2, 1, 8]);
    }
}