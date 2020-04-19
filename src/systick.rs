use cortex_m_semihosting::hprintln;
use core::ptr::{read_volatile, write_volatile};

const CSR_ADDR: usize = 0xE000_E010;
const RVR_ADDR: usize = 0xE000_E014;
const CVR_ADDR: usize = 0xE000_E018;
const CALIB_ADDR: usize = 0xE000_E01C;

pub fn init() {
    hprintln!("Systick init").unwrap();
    unsafe {
        write_volatile(CVR_ADDR as *mut u32, 0);
        let calib_val = read_volatile(CALIB_ADDR as *const u32) & 0x00FF_FFFF;
        write_volatile(RVR_ADDR as *mut u32, calib_val * 100);
        write_volatile(CSR_ADDR as *mut u32, 0x3);
    }
}