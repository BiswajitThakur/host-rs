use std::io::{self, Cursor};

use host_utils::App;

#[test]
fn test_app_insert_valid_host_block_empty_etc_hosts() {
    let mut stdout = io::sink();
    let mut stderr = io::sink();

    let mut app = App::new::<Cursor<Vec<u8>>>("", None, &mut stdout, &mut stderr).unwrap();

    app.add_block(["example.com", "github.com", "12abcxyz.in"].into_iter());

    let mut etc_content = Vec::<u8>::new();
    let mut new_data = Vec::<u8>::new();

    app.save_1(&mut etc_content, &mut new_data).unwrap();

    let etc_want = r#"#host-rs-beg#
0.0.0.0 12abcxyz.in
0.0.0.0 example.com
0.0.0.0 github.com
#host-rs-end#
#r-host-rs-beg#
#r-host-rs-end#
"#;
    assert_eq!(
        unsafe { std::str::from_utf8_unchecked(&etc_content) },
        etc_want
    );

    let new_app = App::new("", Some(Cursor::new(new_data)), &mut stdout, &mut stderr).unwrap();
    let mut etc = Vec::<u8>::new();
    new_app.save_1(&mut etc, &mut io::sink()).unwrap();
    assert_eq!(unsafe { std::str::from_utf8_unchecked(&etc) }, etc_want);
}
