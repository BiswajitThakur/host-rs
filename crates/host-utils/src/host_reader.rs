use crate::{HashList, H, R};

pub fn etc_host_reader<'a>(value: &'a str, h: &mut HashList<H<'a>>, r: &mut HashList<R<'a>>) {
    //let lines: Vec<&str> = value.lines().into_iter().collect();
    // let mut h = HashList::with_capacity(if let Cap::Capacity(v) = h_cap {
    //    v
    // } else {
    //    lines.len()
    // });
    // let mut r = HashList::with_capacity(if let Cap::Capacity(v) = r_cap { v } else { 521 });
    let mut host_flag = false;
    let mut redirect_flag = false;
    let start_host = "#host-rs-beg#";
    let end_host = "#host-rs-end#";
    let start_redirect = "#r-host-rs-beg#";
    let end_redirect = "#r-host-rs-end#";
    for i in value.lines().into_iter() {
        let j = i.trim();
        if host_flag {
            if j == end_host {
                host_flag = false;
                continue;
            };
            if let Ok(v) = H::try_from(j) {
                h.push(v);
            };
            continue;
        };
        if redirect_flag {
            if j == end_redirect {
                redirect_flag = false;
                continue;
            };
            if let Ok(v) = R::try_from(j) {
                r.push(v);
            };
            continue;
        };
        if j == start_host {
            host_flag = true;
        } else if j == start_redirect {
            redirect_flag = true;
        };
    }
}

pub fn host_reader<'a>(value: &'a str, h: &mut HashList<H<'a>>) {
    for line in value.lines().into_iter() {
        if let Ok(v) = H::try_from(line.trim()) {
            h.push(v);
        };
    }
}

#[allow(unused)]
pub fn get_host_from_url<'a>(webs: &'a str) -> Option<&'a str> {
    let mut webs = webs.trim();
    if let Some(v) = webs.find("http://") {
        if v == 0 {
            webs = &webs[7..];
        };
    }; // else
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
    if is_valid_host(v) {
        return Some(v);
    };
    None
}

#[allow(unused)]
fn is_valid_host(value: &str) -> bool {
    if value.len() == 0
        || value.len() > 63
        || value.starts_with(' ')
        || value.ends_with(' ')
        || value.starts_with('.')
        || value.ends_with('.')
        || !value.contains('.')
    {
        return false;
    };
    for c in value.chars() {
        if (c >= 'A' && c <= 'Z')
            || (c >= 'a' && c <= 'z')
            || (c >= '0' && c <= '9')
            || (c == '.')
            || (c == '-')
        {
            continue;
        } else {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(is_valid_host("127.0.0.1:8080"), false);
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