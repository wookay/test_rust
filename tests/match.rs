#[test]
fn rust_match() {
    let val = match 1 {
       1 => 2,
       2 => 3,
       _ => 5,
    };
    assert_eq!(val, 2);
}
