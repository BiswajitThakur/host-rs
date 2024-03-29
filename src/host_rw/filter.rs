use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn host(v: String) -> HashSet<String> {
    let re: Regex = Regex::new(r"^\s*[\w\.\-]+\s+([a-zA-z0-9]+\.[\w\.\-]+)\s*$").unwrap();
    let re1: Regex = Regex::new(r"\s+").unwrap();
    let is_cmt0: Regex = Regex::new(r"^\s*$").unwrap();
    let is_cmt1: Regex = Regex::new(r"^\s*#+[^#]*.*$").unwrap();
    v.lines()
        .filter(|v| !is_comment(v, &is_cmt0, &is_cmt1) && re.is_match(v))
        .map(|v| find_host(v, &re1).to_owned())
        .collect()
}

fn find_host<'a>(s: &'a str, r: &Regex) -> &'a str {
    r.split(s.trim()).nth(1).unwrap_or_default()
}

pub fn redirect(v: String) -> HashMap<String, String> {
    let re: Regex =
        Regex::new(r"^\s*(?P<c0>[a-zA-z0-9]+\.[\w\.\-]+)\s+(?P<c1>[a-zA-z0-9]+\.[\w\.\-]+)\s*$")
            .unwrap();
    let is_cmt0: Regex = Regex::new(r"^\s*$").unwrap();
    let is_cmt1: Regex = Regex::new(r"^\s*#+[^#]*.*$").unwrap();
    let lines = v.lines();
    let mut datas = HashMap::<String, String>::new();
    for i in lines {
        let caps = re.captures(i);
        if let Some(c) = caps {
            if !is_comment(i, &is_cmt0, &is_cmt1) {
                datas.insert(c["c1"].to_string(), c["c0"].to_string());
            };
        };
    }
    datas
}

pub fn is_comment(s: &str, r1: &Regex, r2: &Regex) -> bool {
    r1.is_match(s) || r2.is_match(s)
}

pub fn filter_host(webs: &str) -> Option<String> {
    let re = Regex::new(r"^\s*https?://+(?<host>[^\s\?\/:]+).*$").unwrap();
    if re.is_match(&webs) {
        return Some(re.captures(&webs).unwrap()["host"].to_string());
    };
    let re = Regex::new(r"^\s*(?<host>[a-zA-Z0-9]+\.[^\s\?\/:]+).*$").unwrap();
    if re.is_match(&webs) {
        return Some(re.captures(&webs).unwrap()["host"].to_string());
    };
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_comment() {
        let is_cmt0: Regex = Regex::new(r"^\s*$").unwrap();
        let is_cmt1: Regex = Regex::new(r"^\s*#+[^#]*.*$").unwrap();
        assert_eq!(is_comment("", &is_cmt0, &is_cmt1), true);
        assert_eq!(is_comment(" ", &is_cmt0, &is_cmt1), true);
        assert_eq!(is_comment("     ", &is_cmt0, &is_cmt1), true);
        assert_eq!(is_comment("#", &is_cmt0, &is_cmt1), true);
        assert_eq!(is_comment("####", &is_cmt0, &is_cmt1), true);
        assert_eq!(is_comment(" #", &is_cmt0, &is_cmt1), true);
        assert_eq!(is_comment("   #", &is_cmt0, &is_cmt1), true);
        assert_eq!(is_comment("# ", &is_cmt0, &is_cmt1), true);
        assert_eq!(is_comment("#    ", &is_cmt0, &is_cmt1), true);
        assert_eq!(is_comment("##    ", &is_cmt0, &is_cmt1), true);
        assert_eq!(is_comment("   #   ", &is_cmt0, &is_cmt1), true);
        assert_eq!(is_comment("   ####    ", &is_cmt0, &is_cmt1), true);
        assert_eq!(is_comment("# hel54..-+lo", &is_cmt0, &is_cmt1), true);
        assert_eq!(is_comment("    # hi..iiii", &is_cmt0, &is_cmt1), true);
        assert_eq!(is_comment("#### testing...", &is_cmt0, &is_cmt1), true);
        assert_eq!(is_comment("# te..st # ++test #", &is_cmt0, &is_cmt1), true);
        assert_eq!(is_comment("   ##3 hlo", &is_cmt0, &is_cmt1), true);
        assert_eq!(is_comment("hii #", &is_cmt0, &is_cmt1), false);
        assert_eq!(
            is_comment("    testing # testing", &is_cmt0, &is_cmt1),
            false
        );
        assert_eq!(is_comment("    rust   #", &is_cmt0, &is_cmt1), false);
        assert_eq!(is_comment("// hello.::+-", &is_cmt0, &is_cmt1), false);
        assert_eq!(is_comment("<!--html#css#js#-->", &is_cmt0, &is_cmt1), false);
        assert_eq!(is_comment("/////", &is_cmt0, &is_cmt1), false);
        assert_eq!(is_comment("//// #### //// ####", &is_cmt0, &is_cmt1), false);
    }

    #[test]
    fn test_host() {
        let input0: &str = "127.0.0.1 localhost
# 0.0.0.0 example.com
185.199.110.133 raw.githubusercontent.com
127.0.0.1 google.com  

::1     ip6-localhost ip6-loopback
fe00::0 ip6-localnet
ff00::0 ip6-mcastprefix
ff02::1 ip6-allnodes
ff02::2 ip6-allrouters
  0.0.0.0   yxz
localhost  fooo.in
";
        let got: HashSet<String> = host(input0.to_owned());
        let want: HashSet<String> = HashSet::from([
            "raw.githubusercontent.com".to_owned(),
            "google.com".to_owned(),
            "fooo.in".to_owned(),
        ]);
        assert_eq!(got, want);
    }

    #[test]
    fn test_redirect() {
        let input0: &str = "127.0.0.1 localhost
# 0.0.0.0 example.com
185.199.110.133 raw.githubusercontent.com
127.0.0.1 google.com  

::1     ip6-localhost ip6-loopback
fe00::0 ip6-localnet
ff00::0 ip6-mcastprefix
    f.com                 z.in
ff02::1 ip6-allnodes
kk.ii bb.ll oo.ll
ff02::2 ip6-allrouters
  0.0.0.0   yxz
localhost  fooo.in";
        let got = redirect(input0.to_string());
        let want = HashMap::from([
            ("raw.githubusercontent.com".into(), "185.199.110.133".into()),
            ("google.com".into(), "127.0.0.1".into()),
            ("z.in".into(), "f.com".into()),
        ]);
        assert_eq!(got, want);
    }
    #[test]
    fn test_filter_host() {
        assert_eq!(filter_host(""), None);
        assert_eq!(filter_host("    "), None);
        assert_eq!(filter_host("hhff?lk=89"), None);
        assert_eq!(filter_host("/q123.com?name=BT"), None);
        assert_eq!(filter_host("/q123.com?name=BT#hello"), None);
        assert_eq!(
            filter_host("example.com"),
            Some("example.com".to_string())
        );
        assert_eq!(
            filter_host("http://example.com"),
            Some("example.com".to_string())
        );
        assert_eq!(
            filter_host("https://example.com"),
            Some("example.com".to_string())
        );
        assert_eq!(
            filter_host("http://example.com/about"),
            Some("example.com".to_string())
        );
        assert_eq!(
            filter_host("   https://example.com/url?qq=123&hello=testing"),
            Some("example.com".to_string())
        );
        assert_eq!(
            filter_host("   example.com/url?qq=123&hello=testing"),
            Some("example.com".to_string())
        );
        assert_eq!(
            filter_host("example.com"),
            Some("example.com".to_string())
        );
        assert_eq!(
            filter_host("https://123h.in?kk=99&m=rr"),
            Some("123h.in".to_string())
        );
        assert_eq!(
            filter_host("123h.in?kk=99&m=rr"),
            Some("123h.in".to_string())
        );
        assert_eq!(
            filter_host("127.0.0.1:8080"),
            Some("127.0.0.1".to_string())
        );
        assert_eq!(
            filter_host("127.0.0.1:8080/"),
            Some("127.0.0.1".to_string())
        );
        assert_eq!(
            filter_host("127.0.0.1:8080?"),
            Some("127.0.0.1".to_string())
        );
        assert_eq!(
            filter_host("127.0.0.1"),
            Some("127.0.0.1".to_string())
        );
        assert_eq!(
            filter_host("http://127.0.0.1:8080/"),
            Some("127.0.0.1".to_string())
        );
        assert_eq!(
            filter_host("https://127.0.0.1:8080/"),
            Some("127.0.0.1".to_string())
        );
        assert_eq!(
            filter_host("https://127.0.0.1:8080/login"),
            Some("127.0.0.1".to_string())
        );
        assert_eq!(
            filter_host("  127.0.0.1:80/login?uname=test_user&pass=12345678"),
            Some("127.0.0.1".to_string())
        );
        assert_eq!(
            filter_host("  http://127.0.0.1/login?uname=test_user&pass=12345678"),
            Some("127.0.0.1".to_string())
        );
        assert_eq!(
            filter_host("  127.0.0.1/login?uname=test_user&pass=12345678"),
            Some("127.0.0.1".to_string())
        );
        assert_eq!(
            filter_host("https://1.1.1.1:8080/home"),
            Some("1.1.1.1".to_string())
        );
        assert_eq!(
            filter_host("123.com"),
            Some("123.com".to_string())
        );
    }
}
