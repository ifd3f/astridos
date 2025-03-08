use core::mem::MaybeUninit;

use bytes::BytesMut;
use log::{debug, error};
use smoltcp::{
    phy::{Device, DeviceCapabilities, Medium},
    time::Instant,
    wire::{EthernetAddress, HardwareAddress},
};
use uefi::{
    proto::{
        misc::Timestamp,
        network::{MacAddress, snp::SimpleNetwork},
    },
    runtime::Time,
};

pub struct SimpleNetworkDevice<'a> {
    snp: &'a SimpleNetwork,
}

impl<'a> SimpleNetworkDevice<'a> {
    pub fn new(snp: &'a SimpleNetwork) -> Self {
        Self { snp }
    }
}

impl<'a> Device for SimpleNetworkDevice<'a> {
    type RxToken<'b>
        = UefiRxToken
    where
        Self: 'b;

    type TxToken<'b>
        = UefiTxToken<'b>
    where
        Self: 'b;

    fn receive(
        &mut self,
        _timestamp: smoltcp::time::Instant,
    ) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        let mut rx = UefiRxToken {
            packet: [0; 1536],
            size: 0,
        };
        let tx = UefiTxToken { snp: &self.snp };
        match rx_mac_frame(self.snp, &mut rx.packet) {
            Ok(size) => {
                rx.size = size;
                Some((rx, tx))
            }
            Err(_) => None,
        }
    }

    fn transmit(&mut self, _timestamp: smoltcp::time::Instant) -> Option<Self::TxToken<'_>> {
        Some(UefiTxToken { snp: &self.snp })
    }

    fn capabilities(&self) -> smoltcp::phy::DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.medium = Medium::Ethernet;
        caps.max_transmission_unit = self.snp.mode().max_packet_size as usize;
        caps
    }
}

pub struct UefiRxToken {
    packet: [u8; 1536],
    size: usize,
}

impl smoltcp::phy::RxToken for UefiRxToken {
    fn consume<R, F>(self, f: F) -> R
    where
        F: FnOnce(&[u8]) -> R,
    {
        f(&self.packet[..self.size])
    }
}

pub struct UefiTxToken<'a> {
    snp: &'a SimpleNetwork,
}

impl<'a> smoltcp::phy::TxToken for UefiTxToken<'a> {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mut buf = [0u8; 1536];
        let result = f(&mut buf[..len]);

        if let Err(e) = tx_mac_frame(self.snp, &buf[..len]) {
            error!("error during tx! {e}")
        }
        result
    }
}

struct MacFrame<'a> {
    payload_buf: &'a mut [u8],
    header_size: usize,
    src_addr: MacAddress,
    dst_addr: MacAddress,
    ethertype: u16,
}

pub fn uefi_to_smoltcp_macaddress(a: &MacAddress) -> EthernetAddress {
    let mut out = EthernetAddress([0; 6]);
    out.0.copy_from_slice(&a.0[..6]);
    out
}

pub struct TimestampClock<'a> {
    ts: &'a Timestamp,
    frequency: u64,
}

impl<'a> TimestampClock<'a> {
    pub fn new(ts: &'a Timestamp) -> Result<Self, uefi::Error> {
        let props = ts.get_properties()?;
        Ok(Self {
            ts,
            frequency: props.frequency,
        })
    }

    pub fn now(&self) -> Instant {
        let micros = self.ts.get_timestamp() * 1000000 / self.frequency;
        Instant::from_micros(micros as i64)
    }
}

fn rx_mac_frame<'a>(snp: &SimpleNetwork, buf: &'a mut [u8]) -> Result<usize, uefi::Error> {
    // snp is already a L2 protocol, but smoltcp asks
    // Device consumers to read the raw MAC frame.

    // skip the 14-byte ethernet header
    let payload_buf = &mut buf[14..];

    let mut header_size: usize = 0;
    let mut src_addr: MacAddress = MacAddress([0; 32]);
    let mut dst_addr: MacAddress = MacAddress([0; 32]);
    let mut ethertype: u16 = 0;
    let total_bytes = snp.receive(
        payload_buf,
        Some(&mut header_size),
        Some(&mut src_addr),
        Some(&mut dst_addr),
        Some(&mut ethertype),
    )?;

    // fill in the rest of the header
    buf[0..6].copy_from_slice(&dst_addr.0[0..6]);
    buf[6..12].copy_from_slice(&src_addr.0[0..6]);
    buf[13] = (ethertype >> 8) as u8;
    buf[14] = (ethertype & 0xff) as u8;

    Ok(total_bytes)
}

fn tx_mac_frame(snp: &SimpleNetwork, buf: &[u8]) -> Result<(), uefi::Error> {
    debug!("sending {:x?}", buf);

    // snp is already a L2 protocol, but smoltcp asks
    // Device consumers to construct the MAC frame. so,
    // we will parse out the fields we want.
    let dst_raw = &buf[0..6];
    let src_raw = &buf[6..12];
    let ethertype = (buf[12] as u16) << 8 | buf[13] as u16;
    let payload = &buf[14..];

    // efi is really funny because mac addresses are passed around as
    // 32 whole bytes, where the first 6 are the actual mac and the others
    // seem to just be ignored!
    let mut dst = MacAddress([0; 32]);
    dst.0[..6].copy_from_slice(dst_raw);
    let mut src = MacAddress([0; 32]);
    src.0[..6].copy_from_slice(src_raw);


    snp.transmit(
        snp.mode().media_header_size as usize,
        payload,
        Some(src),
        Some(dst),
        Some(ethertype),
    )?;

    Ok(())
}
