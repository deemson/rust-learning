use std::ops::{Index, IndexMut};

#[test]
fn test_index_operator() {
    struct VecWrapper<T> {
        vec: Vec<T>,
    }

    impl<I: std::slice::SliceIndex<[T]>, T> Index<I> for VecWrapper<T> {
        type Output = I::Output;

        fn index(&self, index: I) -> &Self::Output {
            &self.vec[index]
        }
    }

    impl <I: std::slice::SliceIndex<[T]>, T> IndexMut<I> for VecWrapper<T> {
        fn index_mut(&mut self, index: I) -> &mut Self::Output {
            &mut self.vec[index]
        }
    }

    let mut w = VecWrapper { vec: vec![1, 2, 3] };
    assert_eq!(1, w[0]);
    assert_eq!(2, w[1]);
    assert_eq!(3, w[2]);
    w[0] = 4;
    assert_eq!(4, w[0]);
}