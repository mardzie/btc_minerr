pub trait ToIpV6 {
    fn to_v6(self) -> std::net::Ipv6Addr;
}

pub trait NetworkInformation {
    fn port(&self) -> u16;

    /// The same as [`magic_number`] only as bytes in little-endian.
    fn magic_bytes(&self) -> [u8; 4];

    /// The same as [`magic_bytes`] only as a `u32` in little-endian.
    fn magic_number(&self) -> u32;
}
