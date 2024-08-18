#[allow(unused)]
pub fn is_comment(value: &str) -> bool {
    let v = value.trim();
    if v.is_empty() || v.starts_with('#') {
        return true;
    };
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_comment() {
        assert_eq!(is_comment(""), true);
        assert_eq!(is_comment("     "), true);
        assert_eq!(is_comment("#"), true);
        assert_eq!(is_comment("####"), true);
        assert_eq!(is_comment("   #"), true);
        assert_eq!(is_comment("# "), true);
        assert_eq!(is_comment("#    "), true);
        assert_eq!(is_comment("##    "), true);
        assert_eq!(is_comment("   #   "), true);
        assert_eq!(is_comment("   ####    "), true);
        assert_eq!(is_comment("# hel54..-+lo"), true);
        assert_eq!(is_comment("    # hi..iiii"), true);
        assert_eq!(is_comment("#### testing..."), true);
        assert_eq!(is_comment("# te..st # ++test #"), true);
        assert_eq!(is_comment("   ##3 hlo"), true);
        assert_eq!(is_comment("hii #"), false);
        assert_eq!(is_comment("    testing # testing"), false);
        assert_eq!(is_comment("    rust   #"), false);
        assert_eq!(is_comment("// hello.::+-"), false);
        assert_eq!(is_comment("<!--html#css#js#-->"), false);
        assert_eq!(is_comment("/////"), false);
        assert_eq!(is_comment("//// #### //// ####"), false);
    }
}
