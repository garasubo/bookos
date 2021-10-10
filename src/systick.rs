use cortex_m_semihosting::hprintln;
use core::ptr::write_volatile;

const CSR_ADDR: usize = 0xE000_E010;
const RVR_ADDR: usize = 0xE000_E014;
const CVR_ADDR: usize = 0xE000_E018;

pub fn init() {
    hprintln!("Systick init").unwrap();
    unsafe {
        write_volatile(CVR_ADDR as *mut u32, 0);
        write_volatile(RVR_ADDR as *mut u32, 1 << 23);
        write_volatile(CSR_ADDR as *mut u32, 0x3);
    }
}