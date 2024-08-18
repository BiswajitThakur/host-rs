use host_utils::R;

#[test]
fn method_new_eq() {
    let a = R::new("to", "from");
    let b = R::new("to", "from");
    assert_eq!(a, b);
}

#[test]
fn method_new_ne() {
    let a = R::new("hello", "world");
    let b = R::new("world", "hello");
    assert_ne!(a, b);
}

#[test]
fn tryfrom_str_r_0() {
    let input = R::try_from("hello world");
    assert!(input.is_ok());
    if let Ok(got) = input {
        let want = R::new("hello", "world");
        assert_eq!(got, want);
    };
}

#[test]
fn tryfrom_str_r_1() {
    let input = R::try_from("    hello    world     ");
    assert!(input.is_ok());
    if let Ok(got) = input {
        let want = R::new("hello", "world");
        assert_eq!(got, want);
    };
}

#[test]
fn tryfrom_str_r_2() {
    let input = R::try_from("        ");
    assert!(input.is_err());
}

#[test]
fn tryfrom_str_r_3() {
    let input = R::try_from("   hello     ");
    assert!(input.is_err());
}

#[test]
fn display_r() {
    let input = R::new("hello", "world");
    let got = format!("{}", input);
    let want = "hello world";
    assert_eq!(got, want);
}
