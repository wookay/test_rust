use test_rust::mylib::func1;

#[test]
fn mylib_func1() {
    func1();
    assert_eq!(3, 1+2);
}

// #[test]
// fn mylib_func2() {
//     todo!();
// }
