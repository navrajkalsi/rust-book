fn main() {
    let _empty_needs_type_annotation: Vec<i32> = Vec::new();

    // vec! is quick macro for creating type-inferring vectors
    let _type_is_inferred = vec![1, 2, 3];

    let mut _type_infered_by_updating = Vec::new();
    _type_infered_by_updating.push(0);
    _type_infered_by_updating.push(1);
    _type_infered_by_updating.push(2);
    // all three vectors are i32 types

    // accessing out of bounds value will panic this code
    let third: &i32 = &_type_is_inferred[2];
    println!("third element by indexing: {third}");

    // accessing out of bounds value will just return None, with no errors
    let third: Option<&i32> = _type_is_inferred.get(2);
    match third {
        Some(third) => println!("third element by get: {third}"), // now third is a &i32
        None => println!("No third element"),
    }

    let _hold_reference = &_type_infered_by_updating[0];
    _type_infered_by_updating.push(3);

    // this line will not compile
    // println!("{_hold_reference}");
    // since _type_infered_by_updating is mutable and we are holding a reference to it, we cannot alter the vector
    // This error is due to the way vectors work: Because vectors put the values next to each other in memory,
    // adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space,
    // if there isn’t enough room to put all the elements next to each other where the vector is currently stored.
    // In that case, the reference to the first element would be pointing to deallocated memory.
}
