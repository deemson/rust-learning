#[test]
fn test_vec_of_closures_that_return_i32() {
    let functions: Vec<Box<dyn Fn() -> i32>> = vec![
        Box::new(|| 1),
        Box::new(|| 2),
        Box::new(|| 3),
    ];
    let results = functions.iter().map(|f| f()).collect::<Vec<i32>>();
    assert_eq!([1, 2, 3], results.as_slice())
}

#[test]
fn test_vec_of_closures_that_return_i32_wrapper_struct() {
    struct Wrapper(i32);
    let functions: Vec<Box<dyn Fn() -> Wrapper>> = vec![
        Box::new(|| Wrapper(1)),
        Box::new(|| Wrapper(2)),
        Box::new(|| Wrapper(3)),
    ];
    let results = functions.iter().map(|f| f().0).collect::<Vec<i32>>();
    assert_eq!([1, 2, 3], results.as_slice())
}

#[test]
fn test_vec_of_closures_that_return_vec_of_i32() {
    let functions: Vec<Box<dyn Fn() -> Vec<i32>>> = vec![
        Box::new(|| vec![1, 2, 3]),
        Box::new(|| vec![4, 5, 6]),
        Box::new(|| vec![7, 8, 9]),
    ];
    assert_eq!([1, 2, 3], functions[0]().as_slice());
    assert_eq!([4, 5, 6], functions[1]().as_slice());
    assert_eq!([7, 8, 9], functions[2]().as_slice());
}

#[test]
fn test_vec_of_closures_that_return_dyn_trait() {
    trait Tr {
        fn method(&self) -> i32;
    }

    struct Impl1;
    impl Tr for Impl1 { fn method(&self) -> i32 { 1 } }

    struct Impl2;
    impl Tr for Impl2 { fn method(&self) -> i32 { 2 } }

    let functions: Vec<Box<dyn Fn() -> Box<dyn Tr>>> = vec![
        Box::new(|| Box::new(Impl1 {})),
        Box::new(|| Box::new(Impl2 {})),
    ];
    let traits: Vec<Box<dyn Tr>> = functions.iter().map(|f| f()).collect();
    let results: Vec<i32> = traits.iter().map(|t| t.method()).collect();
    assert_eq!([1, 2], results.as_slice())
}

#[test]
fn test_drop_values_in_vec() {
    let v = vec![1, 2, 3];
    for value in v.iter() {
        drop(value);
    }
}