use std::io::{self, Cursor, Read};

use host_utils::App;

#[test]
fn test_app_insert_valid_host_block() {
    let mut stdout = io::sink();
    let mut stderr = io::sink();
    let mut app = App::new::<Cursor<Vec<u8>>>("", None, &mut stdout, &mut stderr).unwrap();
    app.add_block(["example.com", "github.com", "12abcxyz.in"].into_iter());
    let mut etc = Cursor::new(Vec::<u8>::new());
    let mut new_data = Cursor::new(Vec::<u8>::new());
    app.save_1(&mut etc, &mut new_data).unwrap();
    etc.set_position(0);
    new_data.set_position(0);
    let mut etc_str_got = String::new();
    etc.read_to_string(&mut etc_str_got).unwrap();
    let etc_want = r#"#host-rs-beg#
0.0.0.0 12abcxyz.in
0.0.0.0 example.com
0.0.0.0 github.com
#host-rs-end#
#r-host-rs-beg#
#r-host-rs-end#
"#;
    assert_eq!(etc_str_got.as_str(), etc_want);
    let new_app = App::new("", Some(new_data), &mut stdout, &mut stderr).unwrap();
    let mut etc = Cursor::new(Vec::<u8>::new());
    new_app.save_1(&mut etc, &mut io::sink()).unwrap();
    let mut new_etc_str_got = String::new();
    etc.read_to_string(&mut new_etc_str_got).unwrap();
    assert_eq!(new_etc_str_got.as_str(), etc_want);
}
