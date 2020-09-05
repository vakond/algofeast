use algotest::sort;

#[test]
fn it_works() {
    do_test("Selection Sort", sort::selectsort);
    do_test("Heap Sort", sort::heapsort);
    do_test("Quick Sort", sort::quicksort);
}

/// Generic test for all sorting functions.
fn do_test<F: Fn(&mut [i32])>(title: &str, f: F) {
    println!("{}", title);
    println!("{}", "-".repeat(title.len()));

    let mut seq = vec![];
    f(&mut seq);
    assert_eq!(seq, &[]);

    let mut seq = vec![1];
    f(&mut seq);
    assert_eq!(seq, &[1]);

    let mut seq = vec![2, 1];
    f(&mut seq);
    assert_eq!(seq, &[1, 2]);

    let mut seq = vec![5, 8, 4, 3, 1, 6, 9, 2, 7, 0];
    f(&mut seq);
    assert_eq!(seq, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[allow(dead_code)]
fn test_test<F: Fn(&mut [i32])>(title: &str, f: F) {
    println!("{}", title);
    println!("{}", "-".repeat(title.len()));

    let mut seq = vec![5, 8, 4, 3, 1, 6, 9, 2, 7, 0];
    f(&mut seq);
    assert_eq!(seq, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
