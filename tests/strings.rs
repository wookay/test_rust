use std::fmt::Display ;

fn string<T>(x: T) -> String
where T: Display
{
    x.to_string()
}

#[test]
fn rust_strings() {
    let s1 = string(1);
    assert_eq!(s1, "1");

    let s314 = string(3.14);
    assert_eq!(s314, "3.14");
}
