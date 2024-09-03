use std::collections::HashSet;

use crate::{HashList, H};

pub fn etc_host_reader<'a>(lines: &Vec<&'a str>, h: &mut HashSet<H<'a>>) {
    let mut host_flag = false;
    let start_host = "#host-rs-beg#";
    let end_host = "#host-rs-end#";
    for i in lines {
        let j = i.trim();
        if host_flag {
            if j == end_host {
                host_flag = false;
                continue;
            };
            if let Ok(v) = H::try_from(j) {
                h.insert(v);
            };
            continue;
        };
        if j == start_host {
            host_flag = true;
        };
    }
}

pub fn host_reader<'a>(lines: Vec<&'a str>, h: &mut HashList<H<'a>>) {
    for line in lines.iter() {
        if let Ok(v) = H::try_from(line.trim()) {
            h.push(v);
        };
    }
}

#[allow(unused)]
pub fn get_host_from_url<T: AsRef<str> + ?Sized>(webs: &T) -> Option<&str> {
    let mut webs = webs.as_ref().trim();
    if let Some(v) = webs.find("http://") {
        if v == 0 {
            webs = &webs[7..];
        };
    };
    if let Some(v) = webs.find("https://") {
        if v == 0 {
            webs = &webs[8..];
        };
    };
    let mut end = webs.len();
    if let Some(v) = webs.find('/') {
        if v < end {
            end = v;
        };
    };
    if let Some(v) = webs.find('?') {
        if v < end {
            end = v;
        };
    };
    if let Some(v) = webs.find(':') {
        if v < end {
            end = v;
        };
    };
    let v = &webs[..end];
    if super::is_valid_host(v) {
        return Some(v);
    };
    None
}

#[cfg(test)]
mod tests {
    use crate::get_host_from_url;
    use crate::is_valid_host;
    /*
    macro_rules! test_is_valid_host {
        (test $func:ident;  $left:expr  $right:expr) => {
            #[test]
            fn func() {
                assert_eq!(is_valid_host($left), $right);
            }
        };
    }*/
    #[test]
    fn test_is_valid_host_0() {
        assert_eq!(is_valid_host(""), false);
    }

    #[test]
    fn test_is_valid_host_1() {
        assert_eq!(is_valid_host("   "), false);
    }

    #[test]
    fn test_is_valid_host_2() {
        assert_eq!(is_valid_host("    .  "), false);
    }

    #[test]
    fn test_is_valid_host_3() {
        assert_eq!(is_valid_host("  ;www.github.com-++"), false);
    }

    #[test]
    fn test_is_valid_host_4() {
        assert_eq!(is_valid_host("/q123.com?name=BT"), false);
    }

    #[test]
    fn test_is_valid_host_5() {
        assert_eq!(is_valid_host("example.com"), true);
    }

    #[test]
    fn test_is_valid_host_6() {
        assert_eq!(is_valid_host(String::from("127.0.0.1:8080")), false);
    }

    #[test]
    fn test_is_valid_host_7() {
        assert_eq!(is_valid_host("127.0.0.1"), true);
    }

    #[test]
    fn test_filter_host() {
        assert_eq!(get_host_from_url(""), None);
        assert_eq!(get_host_from_url("   "), None);
        assert_eq!(get_host_from_url("hhff?lk=89"), None);
        assert_eq!(get_host_from_url("/q123.com?name=BT"), None);
        assert_eq!(get_host_from_url("example.com"), Some("example.com"));
        assert_eq!(get_host_from_url("http://example.com"), Some("example.com"));
        assert_eq!(
            get_host_from_url("https://example.com"),
            Some("example.com")
        );
        assert_eq!(
            get_host_from_url("https://example.com"),
            Some("example.com")
        );
        assert_eq!(
            get_host_from_url("http://example.com/about"),
            Some("example.com")
        );
        assert_eq!(
            get_host_from_url("   https://example.com/url?qq=123&hello=testing"),
            Some("example.com")
        );
        assert_eq!(
            get_host_from_url("   example.com/url?qq=123&hello=testing"),
            Some("example.com")
        );
        assert_eq!(
            get_host_from_url("https://123h.in?kk=99&m=rr"),
            Some("123h.in")
        );
        assert_eq!(get_host_from_url("123h.in?kk=99&m=rr"), Some("123h.in"));
        assert_eq!(get_host_from_url("127.0.0.1:8080"), Some("127.0.0.1"));
        assert_eq!(get_host_from_url("127.0.0.1:8080/"), Some("127.0.0.1"));
        assert_eq!(get_host_from_url("127.0.0.1:8080?"), Some("127.0.0.1"));
        assert_eq!(get_host_from_url("127.0.0.1"), Some("127.0.0.1"));
        assert_eq!(
            get_host_from_url("https://1.1.1.1:8080/home?val=http://www.example.com"),
            Some("1.1.1.1")
        );
        assert_eq!(
            get_host_from_url("hellohttps://1.1.1.1:8080/home?val=http://www.example.com"),
            None
        );
        assert_eq!(
            get_host_from_url("hello https://1.1.1.1:8080/home?val=http://www.example.com"),
            None
        );
        assert_eq!(
            get_host_from_url("  http://127.0.0.1/login?uname=test_user&pass=12345678"),
            Some("127.0.0.1")
        );

        // assert_eq!(
        //    get_host_from_url("http://https://97.7.54.10/login?uname=test_user&pass=12345678"),
        //   None
        //);
    }
}
