use algotest::tree; // macro tree!
use algotest::tree::Tree;

#[test]
fn it_works() {
    let tree = tree!(1, 3, 2, 5, 4, 2, 9, 7, 8, 6, 6, 1);

    assert!(tree.contains(4));
    assert!(!tree.contains(999));

    let values = tree.walk();
    assert_eq!(values, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
