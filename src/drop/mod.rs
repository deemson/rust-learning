#[test]
fn test_drop_signal() {
    struct DropSignaler<'a> {
        is_dropped: &'a mut bool,
    }
    impl <'a> Drop for DropSignaler<'a> {
        fn drop(&mut self) {
            *self.is_dropped = true;
        }
    }
    let mut is_dropped = false;
    let drop_signaler = DropSignaler { is_dropped: &mut is_dropped };
    drop(drop_signaler);
    assert!(is_dropped);
}