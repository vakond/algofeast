//! Sorting algorithms.

#[allow(unused_imports)]
use std::fmt::Debug;

type Index = usize;

/// Sorts a sequence using the SelectionSort algorithm.
///  0, 1, 2, 3, 4, 5, 6, 7, 8, 9
/// [5, 8, 4, 3, 1, 6, 9, 2, 7, 0]
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

// Sorts a sequence using the HeapSort algorithm.
//pub fn heapsort<T: PartialOrd>(seq: &mut [T]) {}

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
