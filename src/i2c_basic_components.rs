use kernel::bindings;
unsafe extern "C" {
    unsafe fn gpio_to_desc(gpio: u32) -> *mut bindings::gpio_desc;
    unsafe fn gpiod_direction_output(desc: *mut bindings::gpio_desc, value: core::ffi::c_int) -> core::ffi::c_int;
    unsafe fn gpiod_direction_input(desc: *mut bindings::gpio_desc) -> core::ffi::c_int;
    unsafe fn gpiod_put(desc: *mut bindings::gpio_desc);
    unsafe fn gpiod_get_value(desc: *mut bindings::gpio_desc) -> core::ffi::c_int;
}
const GPIO_SDA: u32 = 514;
const GPIO_SCL: u32 = 515;
const DELAY:usize =4;
#[derive(Debug)]
pub enum I2cError{
    NoAck,
    InvalidBytes
}
pub struct I2CBasics{
    sda:*mut bindings::gpio_desc,
    scl:*mut bindings::gpio_desc,
}

impl I2CBasics{
    pub fn new() ->Self{
        I2CBasics {
            sda:(unsafe{gpio_to_desc(GPIO_SDA)}),
            scl: (unsafe{gpio_to_desc(GPIO_SCL)})
        }
    }
    pub fn write_byte(&self,mut byte:u8)->Result<(),I2cError>{
        for _ in 0..8{
            self.write_bit(byte&0x80!=0);
            byte<<=1
        }

        self.set_sda(true);
        self.set_scl(true);
        self.delay();

        let ack=!self.read_sda();//check if low

        self.set_scl(false);
        return if ack {Ok(())} else{Err(I2cError::NoAck)}
    }
    fn write_bit(&self,boolean:bool){
        self.set_sda(boolean);
        //send bit
        self.set_scl(true);
        self.delay();
        //reset
        self.set_scl(false);

    }
    pub fn start(&self){
        //init
        self.set_sda(true);
        self.set_scl(true);
        self.delay();
        self.set_sda(false);
        self.delay();
        self.set_scl(false);
        self.delay();
    }
    pub fn stop(&self){
        self.set_sda(false);
        self.set_scl(true);
        self.delay();
        self.set_sda(true);
        self.delay();
        
    }
    fn set_sda(&self, boolean:bool){
        if boolean{
            unsafe{gpiod_direction_input(self.sda);}
        }
        else{
            unsafe{gpiod_direction_output(self.sda, 0);}
        }
    }
    fn set_scl(&self, boolean:bool){
        if boolean{
            unsafe{gpiod_direction_input(self.scl);}
        }
        else{
            unsafe{gpiod_direction_output(self.scl, 0);}
        }  
    }
    fn delay(&self){
        unsafe { bindings::__udelay(DELAY) };
    }
    fn read_sda(&self)->bool{
        return unsafe{gpiod_get_value(self.sda)}!=0;
    }

} 
impl Drop for I2CBasics{
    fn drop(&mut self){
        unsafe{
            gpiod_put(self.sda);
            gpiod_put(self.scl);
        }
    }
}