mod i2c_basic_components;
mod i2c_oled_handler;

use kernel::bindings;
use kernel::prelude::*;
use kernel::uaccess::UserSlice;
use crate::i2c_oled_handler::I2COled;
use kernel::alloc::allocator::Kmalloc;
use kernel::alloc::KBox;
module! {
    type: BitbangI2CDriver,
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
struct FOPSWrapper(bindings::file_operations);

static OLED_FOPS: FOPSWrapper = FOPSWrapper(unsafe{
    let mut ops = core::mem::zeroed::<bindings::file_operations>();
    ops.owner = core::ptr::addr_of_mut!(bindings::__this_module);
    ops.write = Some(oled_write);
    ops.open = Some(oled_open);
    ops
});
static mut OLED_MISC: bindings::miscdevice = unsafe{
    let mut misc = core::mem::zeroed::<bindings::miscdevice>();
    misc.minor = bindings::MISC_DYNAMIC_MINOR as i32;
    misc.name = b"oled\0".as_ptr() as *const core::ffi::c_char;
    misc.fops = &OLED_FOPS.0;
    misc
};
static mut OLED: *mut I2COled = core::ptr::null_mut();

struct BitbangI2CDriver;

impl kernel::Module for BitbangI2CDriver {
    fn init(_module: &'static ThisModule) -> Result<Self> {

        let handler =I2COled::new();
        let global_handler = KBox::new(handler, GFP_KERNEL).map_err(|_| ENOMEM)?;
        unsafe { OLED = Box::into_raw(global_handler) };

        let ret = unsafe { misc_register(&raw mut OLED_MISC) };
        if ret<0{
            return Err(ENODEV)
        }

        Ok(BitbangI2CDriver)
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
    /*
    this loads the oled handler & transfers buf to kernel mem
    then writes data to oled
    on fail - returns -1
     */
    if count!=1024{return -1}
    let oled = unsafe { &*((*file).private_data as *const I2COled) };
    
    let ubuf = UserSlice::new(buf as _, count);
    let ureader = ubuf.reader();
    
    let mut kvec = match KVec::with_capacity(count, GFP_KERNEL){
        Ok(r)=>r,
        Err(e)=>{pr_err!("error while trying to allocate memory");return -1}
    };
    match ureader.read_all(&mut kvec, GFP_KERNEL){
        Ok(r)=>{}
        Err(e)=>{pr_err!("trouble while copying data from userspace");return -1}
    };
    let bytes=kvec.as_slice();

    return 1024;

}

impl Drop for BitbangI2CDriver {
    fn drop(&mut self) {
        unsafe{
            misc_deregister(&raw mut OLED_MISC);
            drop(Box::<I2COled, Kmalloc>::from_raw(OLED));
            OLED = core::ptr::null_mut();
            pr_info!("GPIO test unloaded\n");
        }
    }
}

unsafe impl Send for BitbangI2CDriver {}
unsafe impl Sync for BitbangI2CDriver {}
unsafe impl Sync for FOPSWrapper {}