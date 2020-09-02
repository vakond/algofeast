use algotest::tree::Tree;

#[test]
fn it_works() {
    let mut tree = Tree::new();

    tree.insert(1);
    tree.insert(3);
    tree.insert(2);
    tree.insert(5);
    tree.insert(4);
    tree.insert(2);
    tree.insert(9);
    tree.insert(7);
    tree.insert(8);
    tree.insert(6);
    tree.insert(6);
    tree.insert(1);

    assert!(tree.contains(4));
    assert!(!tree.contains(999));

    let values = tree.walk();
    assert_eq!(values, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
