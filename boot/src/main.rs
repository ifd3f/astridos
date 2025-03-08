#![no_main]
#![no_std]

use core::{net::Ipv6Addr, str::FromStr};

use log::info;
use uefi::{
    Identify,
    boot::{ScopedProtocol, open_protocol_exclusive},
    prelude::*,
    proto::network::{IpAddress, MacAddress, snp::SimpleNetwork},
};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    info!("hello world from astridos-bootos!");

    let (h, net) = init_network().unwrap();
    let current_address = net.mode().current_address;
    info!(
        "initialized network on handle {:?} with mac {:x?}",
        h.as_ptr(),
        &current_address.0[..6]
    );

    let dst_ip = Ipv6Addr::from_str("ff02::1").unwrap();
    let dst_ip = IpAddress(dst_ip.octets());
    let dst_mac = net.mcast_ip_to_mac(true, dst_ip).unwrap();
    let local_ip = Ipv6Addr::from_str("fe80::1234").unwrap();
    let local_ip = IpAddress(local_ip.octets());
    info!("sending to ip {:x?} mac {:x?}", dst_ip, dst_mac);

    let mut sent_packets = 0u64;
    loop {
        let buffer = &[0, 1, 2, 3, 4];
        info!("tx {}", sent_packets);
        net.transmit(
            net.mode().media_header_size as usize,
            buffer,
            None,
            Some(dst_mac),
            Some(0x86DD),
        );
        sent_packets += 1;
        boot::stall(1_000000);
    }
}

fn make_mac_address(mac: [u8; 6]) -> MacAddress {
    let mut out = MacAddress([0; 32]);
    for i in 0..6 {
        out.0[i] = mac[i];
    }
    out
}

pub fn init_network() -> uefi::Result<(Handle, ScopedProtocol<SimpleNetwork>)> {
    let handle = boot::get_handle_for_protocol::<SimpleNetwork>()?;
    let net = boot::open_protocol_exclusive::<SimpleNetwork>(handle)?;
    net.initialize(0, 0);
    net.start()?;
    Ok((handle, net))
}
