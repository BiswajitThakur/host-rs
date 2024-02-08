use regex::Regex;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn host(v: String) -> HashSet<String> {
    let re: Regex =
        Regex::new(r"^\s*\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\s+([^\s]+)\s*(#?[^\n]*)*$").unwrap();
    let re1: Regex = Regex::new(r"\s+").unwrap();
    let is_cmt0: Regex = Regex::new(r"^\s*$").unwrap();
    let is_cmt1: Regex = Regex::new(r"^\s*#+[^#]*.*$").unwrap();

    let lines: Vec<&str> = v.lines().collect();
    let mut host_list: HashSet<String> = HashSet::with_capacity(lines.len());
    for line in lines {
        if !is_comment(line, &is_cmt0, &is_cmt1) && re.is_match(line) {
            host_list.insert(find_host(line, &re1).to_owned());
        };
    }
    host_list
}

fn find_host<'a>(s: &'a str, r: &Regex) -> &'a str {
    r.split(s.trim()).nth(1).unwrap_or_default()
}

pub fn is_comment(s: &str, r1: &Regex, r2: &Regex) -> bool {
    r1.is_match(s) || r2.is_match(s)
}

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
127.0.0.1 google.com  #this is comment 
  0.0.0.0   yxz
localhost  fooo.in
";
    let got: HashSet<String> = host(input0.to_owned());
    let want: HashSet<String> = HashSet::from([
        "localhost".to_owned(),
        "raw.githubusercontent.com".to_owned(),
        "google.com".to_owned(),
        "yxz".to_owned(),
        //"fooo.in".to_owned(),
    ]);
    assert_eq!(got, want);
}
