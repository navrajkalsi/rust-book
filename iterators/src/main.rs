fn main() {
    let v1 = vec![1, 2, 3];

    // this does nothing as iterators are LAZY
    let v1_iter = v1.iter();

    // iterators have two types of methods defined on them:
    // cosuming adapters
    // iterator adapters

    // consuming adapters are methods that call next() on the iterator until they receive none
    // these methods take the ownership of the iterator and alter its state
    // the iterator then cannot be used further
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

    // iterator adapters return a new iterator after altering it
    let v1: Vec<i32> = vec![1, 2, 3];
    let _ = v1.iter().map(|x| x + 1); // this will do nothing as iterators are lazy, need to
    // call collect to actually perform the operation and store the collection
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    // checkout lib.rs
}
