enum IpAddrKindNaive {
    V4,
    V6,
}

enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),
}

impl IpAddrKind{
    pub fn is_loopback(&self) -> bool{
        match self{
            IpAddrKind::V4(127,0,0,1) => true,
            IpAddrKind::V6("::1") => true,
        }
    }
}



fn main(){

    assert!(IpAddrKind::V4(127,0,0,1).is_loopback(), "incorrect localhost");
    assert!(IpAddrKind::V6("::1".to_string()).is_loopback(), "incorrect localhost");
    assert!(!IpAddrKind::V4(192,168,0,1).is_loopback(), "not localhost");

}