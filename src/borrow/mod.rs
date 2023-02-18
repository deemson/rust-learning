#[test]
fn test_flip_borrowed_bool() {
    fn flip(flag: &mut bool) {
        *flag = true;
    }
    let mut flag = false;
    flip(&mut flag);
    assert!(flag, "flag was not flipped")
}