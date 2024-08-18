use host_utils::{etc_host_reader, HashList, H, R};
#[test]
fn test_etc_host_reader_0() {
    let input = r#"
127.0.0.1	localhost
127.0.1.1	eagle

# The following lines are desirable for IPv6 capable hosts
::1     ip6-localhost ip6-loopback
fe00::0 ip6-localnet
ff00::0 ip6-mcastprefix
ff02::1 ip6-allnodes
ff02::2 ip6-allrouters

#host-rs-beg#
127.0.0.1 aa-123.com
# 0.0.0.0 example.com
la.com         b.com
120.88.99.1 google.com  

  0.0.0.0   99.xyz
kh90-m.in  99.0.0.100

fb.in  fb.com
#host-rs-end#
185.199.110.133 raw.githubusercontent.com

0.0.0.0 example.com
#r-host-rs-beg#
127.0.0.1    google.com  
127.0.0.1	localhost
localhost   facebook.com

#0.0.0.0  comment.com
0.0.0.0 testing.in
   127.0.0.1   m.com

0.0.0.0  b.com

#r-host-rs-end#
127.0.0.1    c.in
        "#;
    let h_want = HashList::from(vec![
        H::new("aa-123.com"),
        H::new("b.com"),
        H::new("google.com"),
        H::new("99.xyz"),
        H::new("99.0.0.100"),
        H::new("fb.com"),
    ]);
    let r_want = HashList::from(vec![
        R::new("127.0.0.1", "google.com"),
        R::new("127.0.0.1", "localhost"),
        R::new("localhost", "facebook.com"),
        R::new("0.0.0.0", "testing.in"),
        R::new("127.0.0.1", "m.com"),
        R::new("0.0.0.0", "b.com"),
    ]);
    let mut h_got = HashList::with_capacity(10);
    let mut r_got = HashList::with_capacity(10);
    etc_host_reader(input, &mut h_got, &mut r_got);
    assert_eq!(h_got, h_want);
    assert_eq!(r_got, r_want);
}
