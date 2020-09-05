//! Sorting algorithms.

#[allow(unused_imports)]
use std::fmt::Debug;

type Index = usize;

/// Sorts a sequence using the SelectionSort algorithm.
pub fn selectsort<T: PartialOrd>(seq: &mut [T]) {
    if seq.len() > 1 {
        for i in 0..(seq.len() - 1) {
            let j = i + 1 + select_min(&seq[(i + 1)..]);
            if seq[j] < seq[i] {
                seq.swap(i, j);
            }
        }
    }

    fn select_min<T: PartialOrd>(seq: &[T]) -> Index {
        let mut result = 0;
        for i in 0..seq.len() {
            if seq[i] < seq[result] {
                result = i;
            }
        }
        result
    }
}

/// Sorts a sequence using the HeapSort algorithm.
pub fn heapsort<T: PartialOrd>(mut seq: &mut [T]) {
    if seq.len() > 1 {
        make_min_heap(&mut seq);
        assert!(is_min_heap(&seq));
        for i in 0..(seq.len() - 1) {
            make_min_heap(&mut seq[(i + 1)..]);
        }
    }

    /// Converts array into minimal heap.
    fn make_min_heap<T: PartialOrd>(mut seq: &mut [T]) {
        for i in (0..seq.len() / 2).rev() {
            min_heapify(&mut seq, i);
        }
    }

    /// Maintains the minimal heap property for subtree with root at start.
    fn min_heapify<T: PartialOrd>(mut seq: &mut [T], start: Index) {
        let left = 2 * start + 1;
        let right = left + 1;
        let mut smallest = start;

        if left < seq.len() && seq[left] < seq[smallest] {
            smallest = left;
        }
        if right < seq.len() && seq[right] < seq[smallest] {
            smallest = right;
        }

        if smallest != start {
            seq.swap(start, smallest);
            min_heapify(&mut seq, smallest); // recur
        }
    }

    /// Checks if a sequence is correct minimal heap.
    fn is_min_heap<T: PartialOrd>(seq: &[T]) -> bool {
        fn recur<T: PartialOrd>(seq: &[T], root: Index) -> bool {
            let n = seq.len();
            if root >= n / 2 {
                return true;
            }
            let left = 2 * root + 1;
            if left >= n {
                return true;
            }
            let right = left + 1;
            if right >= n {
                return true;
            }
            if seq[root] <= seq[left]
                && seq[root] <= seq[right]
                && recur(&seq, left)
                && recur(&seq, right)
            {
                return true;
            }
            false
        }
        recur(seq, 0)
    }
}

/// Sorts a sequence using the classical QuickSort algorithm.
pub fn quicksort<T: PartialOrd>(seq: &mut [T]) {
    if seq.len() > 1 {
        recur(seq, 0, seq.len() - 1);
    }

    fn recur<T: PartialOrd>(seq: &mut [T], l: Index, h: Index) {
        if h > l {
            let p = partition(seq, l, h);
            if p > 0 {
                recur(seq, l, p - 1);
            }
            recur(seq, p + 1, h);
        }
    }

    fn partition<T: PartialOrd>(seq: &mut [T], l: Index, h: Index) -> Index {
        let p = h;
        let mut d = l;
        for i in l..=h {
            if seq[i] < seq[p] {
                seq.swap(i, d);
                d += 1;
            }
        }
        seq.swap(p, d);
        d
    }
}

// Sorts a sequence using the MergeSort algorithm.
//pub fn mergesort<T: PartialOrd>(seq: &mut [T]) {}

// Sorts a sequence using the ParallelMergeSort algorithm.
//pub fn parallel_mergesort<T: PartialOrd>(seq: &mut [T]) {}
