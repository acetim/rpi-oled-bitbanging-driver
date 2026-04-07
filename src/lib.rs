mod i2c_basic_components;
mod i2c_oled_handler;
use kernel::bindings;
use kernel::prelude::*;

use crate::i2c_oled_handler::I2COled;
module! {
    type: GpioDriver,
    name: "gpio_bitbang_i2c",
    author: "acetim",
    description: "i2c oled bitbang driver",
    license: "GPL",
}
unsafe extern "C" {
    unsafe fn misc_register(misc: *mut bindings::miscdevice) -> core::ffi::c_int;
    unsafe fn misc_deregister(misc: *mut bindings::miscdevice);
    unsafe fn _copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
}
static OLED_FOPS: bindings::file_operations = {
    let mut ops = unsafe { core::mem::zeroed::<bindings::file_operations>() };
    ops.owner = core::ptr::null_mut();
    ops.write = Some(oled_write);
    ops.open = Some(oled_open);
    ops
};
static mut OLED_MISC: bindings::miscdevice = {
    let mut misc = unsafe { core::mem::zeroed::<bindings::miscdevice>() };
    misc.minor = bindings::MISC_DYNAMIC_MINOR as i32;
    misc.name = b"oled\0".as_ptr() as *const core::ffi::c_char;
    misc.fops = &OLED_FOPS;
    misc
};
static mut OLED: *mut I2COled = core::ptr::null_mut();

struct BitbangI2CDriver;

impl kernel::Module for BitbangI2CDriver {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        let handler =I2COled::new();
        let global_handler = Box::<I2COled, Kmalloc>::new(handler, GFP_KERNEL).map_err(|_| ENOMEM)?;
        unsafe { OLED = Box::into_raw(handler) };

        let ret = unsafe { misc_register(&raw mut OLED_MISC) };
        if(ret<0){
            return Err(ENODEV)
        }

        Ok(OledDriver)
    }
    
}
unsafe extern "C" fn oled_open(
    _inode: *mut bindings::inode,
    file: *mut bindings::file,
) -> core::ffi::c_int {
    unsafe { (*file).private_data = OLED as *mut core::ffi::c_void };
    0
}
unsafe extern "C" fn oled_write(
    file: *mut bindings::file,
    buf: *const core::ffi::c_char,
    count: usize,
    _ppos: *mut bindings::loff_t,
) -> isize {
    let oled = unsafe { &*((*file).private_data as *const I2COled) };
    //TODO - convert buf to kernel space and call oled handler func
    
}

impl Drop for BitbangI2CDriver {
    fn drop(&mut self) {
        unsafe{
            misc_deregister(&raw mut OLED_MISC);
            drop(unsafe{Box::<I2COled, Kmalloc>::from_raw(OLED)});
            Oled = core::ptr::null_mut();
            pr_info!("GPIO test unloaded\n");
        }
    }
}

unsafe impl Send for BitbangI2CDriver {}
unsafe impl Sync for BitbangI2CDriver {}