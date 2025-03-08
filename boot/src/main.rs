#![no_main]
#![no_std]

mod smoltcp_uefi;
mod time;

use core::{
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

use log::info;
use smoltcp::{
    iface::{Config, Interface},
    socket::icmp,
    wire::{HardwareAddress, IpAddress, IpCidr},
};
use smoltcp::{
    iface::{SocketSet, SocketStorage},
    wire::Ipv4Address,
};
use smoltcp::{
    socket::{AnySocket, Socket},
    wire::Ipv6Address,
};
use smoltcp_uefi::{SimpleNetworkDevice, TimestampClock, uefi_to_smoltcp_macaddress};
use time::now_rdtsc;
use uefi::{
    Identify,
    boot::{ScopedProtocol, open_protocol_exclusive},
    prelude::*,
    proto::{
        misc::Timestamp,
        network::{MacAddress, snp::SimpleNetwork},
    },
};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    info!("hello world from astridos-bootos!");

    let (h, snp) = init_network().unwrap();
    info!(
        "initialized network on handle {:?} with mac {:x?}",
        h.as_ptr(),
        &snp.mode().current_address.0[..6]
    );

    send_loop(snp.get().unwrap());

    Status::SUCCESS
}

fn send_loop(snp: &SimpleNetwork) {
    let current_address = snp.mode().current_address;

    let mut device = SimpleNetworkDevice::new(snp);
    let mut iface = Interface::new(
        Config::new(HardwareAddress::Ethernet(uefi_to_smoltcp_macaddress(
            &current_address,
        ))),
        &mut device,
        now_rdtsc(),
    );

    iface.update_ip_addrs(|ip_addrs| {
        ip_addrs
            .push(IpCidr::new(IpAddress::v4(192, 168, 69, 1), 24))
            .unwrap();
        ip_addrs
            .push(IpCidr::new(
                IpAddress::v6(0xfe80, 0, 0, 0, 0, 0, 0x12, 0x34),
                64,
            ))
            .unwrap();
    });
    iface
        .routes_mut()
        .add_default_ipv4_route(Ipv4Address::new(192, 168, 69, 100))
        .unwrap();
    iface
        .routes_mut()
        .add_default_ipv6_route(Ipv6Address::new(0xfe80, 0, 0, 0, 0, 0, 0, 0x100))
        .unwrap();

    macro_rules! make_buffer {
        ($name:ident) => {
            let mut metadata = [icmp::PacketMetadata::EMPTY; 256];
            let mut payload = [0u8; 256];
            let $name =
                icmp::PacketBuffer::new(&mut metadata as &mut [_], &mut payload as &mut [_]);
        };
    }

    make_buffer!(icmp_rx_buffer);
    make_buffer!(icmp_tx_buffer);
    let sockets = &mut [SocketStorage::EMPTY] as &mut [SocketStorage<'_>];
    let mut sockets = SocketSet::new(sockets);
    let handle = sockets.add(icmp::Socket::new(icmp_rx_buffer, icmp_tx_buffer));

    let mut sent_packets = 0u64;
    loop {
        let icmp_socket = sockets.get_mut::<icmp::Socket>(handle);
        if !icmp_socket.is_open() {
            icmp_socket.bind(icmp::Endpoint::Ident(0x22b)).unwrap();
        }
        let payload = icmp_socket
            .send(3, IpAddress::from_str("fe80::1").unwrap())
            .unwrap();
        payload[..3].copy_from_slice(b"owo");

        info!("tx {}", sent_packets);
        iface.poll(now_rdtsc(), &mut device, &mut sockets);
        /*
        loop {
            let result = iface.poll(now_rdtsc(), &mut device, &mut sockets);

            boot::stall(100_000);
            match result {
                smoltcp::iface::PollResult::None => continue,
                smoltcp::iface::PollResult::SocketStateChanged => break,
            }
        } */

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
    net.initialize(0, 0)?;
    net.start()?;
    Ok((handle, net))
}
