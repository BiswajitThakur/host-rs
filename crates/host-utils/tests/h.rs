use host_utils::H;

#[test]
fn method_new_eq() {
    let a: H = H::new("test");
    let b: H = H::new("test");
    assert_eq!(a, b);
}

#[test]
fn method_new_ne() {
    let a = H::new("hello");
    let b = H::new("world");
    assert_ne!(a, b);
}

#[test]
fn method_as_str_eq() {
    let input = H::new("hello");
    let got = input.as_str();
    let want = "hello";
    assert_eq!(got, want);
}

#[test]
fn method_as_str_ne() {
    let input = H::new("hello");
    let got = input.as_str();
    let want = "world";
    assert_ne!(got, want);
}

#[test]
fn from_h_for_str_eq() {
    let input = H::new("hello");
    let got: &str = input.into();
    let want = "hello";
    assert_eq!(got, want);
}

#[test]
fn from_h_for_str_ne() {
    let input = H::new("hello");
    let got: &str = input.into();
    let want = "world";
    assert_ne!(got, want);
}

#[test]
fn tryfrom_str_h_0_eq() {
    let input = "hello";
    let v = H::try_from(input);
    assert!(v.is_ok());
    if let Ok(got) = v {
        let want = H::new("hello");
        assert_eq!(got, want);
    };
}

#[test]
fn tryfrom_str_h_1_eq() {
    let input = "      hello     ";
    let v = H::try_from(input);
    assert!(v.is_ok());
    if let Ok(got) = v {
        let want = H::new("hello");
        assert_eq!(got, want);
    };
}

#[test]
fn tryfrom_str_h_2_eq() {
    let input = "      hello    world  ";
    let v = H::try_from(input);
    assert!(v.is_ok());
    if let Ok(got) = v {
        let want = H::new("world");
        assert_eq!(got, want);
    };
}

#[test]
fn tryfrom_str_h_3_eq() {
    let input = "      0.0.0.0    example.com   github.com ";
    let v = H::try_from(input);
    assert!(v.is_ok());
    if let Ok(got) = v {
        let want = H::new("example.com");
        assert_eq!(got, want);
    };
}

#[test]
fn tryfrom_str_h_0_err() {
    let input = "";
    let v = H::try_from(input);
    assert!(v.is_err());
}

#[test]
fn tryfrom_str_h_1_err() {
    let input = "     ";
    let v = H::try_from(input);
    assert!(v.is_err());
}

#[test]
fn h_display() {
    let input = H::new("hello");
    let got = format!("{}", input);
    let want = "hello";
    assert_eq!(got, want);
}
