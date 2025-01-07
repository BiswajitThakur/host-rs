use std::{
    fs,
    io::{self, BufReader, Cursor, Read},
};

use host_utils::HostRs;

fn block_valid_hosts(args: Vec<(&str, &str, Vec<&str>)>) {
    let mut stdout = Cursor::new(Vec::<u8>::new());
    let mut stderr = Cursor::new(Vec::<u8>::new());
    for (index, (etc_hosts, want_etc_hosts, args)) in args.into_iter().enumerate() {
        let mut app = HostRs::<Cursor<Vec<u8>>, Cursor<Vec<u8>>>::new::<BufReader<fs::File>>(
            etc_hosts,
            None,
            &mut stdout,
            &mut stderr,
        )
        .unwrap();
        app.add_block(args);
        let mut etc_hosts_file = Cursor::new(Vec::<u8>::new());
        let mut db = Cursor::new(Vec::<u8>::new());
        app.save::<Cursor<Vec<u8>>, Cursor<Vec<u8>>>(&mut etc_hosts_file, &mut db)
            .unwrap();
        db.set_position(0);
        etc_hosts_file.set_position(0);
        let mut got_hosts = String::new();
        etc_hosts_file.read_to_string(&mut got_hosts).unwrap();
        if got_hosts.as_str() != want_etc_hosts {
            eprintln!("Test Faild Index: {index}");
        }
        assert_eq!(got_hosts.as_str(), want_etc_hosts);
    }
}

#[test]
fn test_block_valid_host() {
    let tests = vec![(
        r#"127.0.0.1	localhost

# The following lines are desirable for IPv6 capable hosts
::1     ip6-localhost ip6-loopback
fe00::0 ip6-localnet
ff00::0 ip6-mcastprefix
ff02::1 ip6-allnodes
ff02::2 ip6-allrouters"#,
        r#"127.0.0.1	localhost

# The following lines are desirable for IPv6 capable hosts
::1     ip6-localhost ip6-loopback
fe00::0 ip6-localnet
ff00::0 ip6-mcastprefix
ff02::1 ip6-allnodes
ff02::2 ip6-allrouters
#host-rs-beg#
0.0.0.0 120.54.33.10
0.0.0.0 33.70.100.45
0.0.0.0 example.com
0.0.0.0 test.in
0.0.0.0 www.abc.xyz
#host-rs-end#
#r-host-rs-beg#
#r-host-rs-end#
"#,
        vec![
            "example.com",
            "test.in",
            "120.54.33.10",
            "33.70.100.45",
            "www.abc.xyz",
        ],
    )];
    block_valid_hosts(tests);
}
