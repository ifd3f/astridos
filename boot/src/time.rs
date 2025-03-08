use smoltcp::time::Instant;

pub fn now_rdtsc() -> Instant {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        Instant::from_micros(core::arch::x86_64::_rdtsc() as i64)
    }

    #[cfg(target_arch = "aarch64")]
    unsafe {
        let mut ticks: u64;
        core::arch::asm!("mrs {}, cntvct_el0", out(reg) ticks);
        Instant::from_micros(ticks as i64)
    }
}