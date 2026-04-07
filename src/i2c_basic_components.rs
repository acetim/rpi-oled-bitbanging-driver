use kernel::bindings;
//TODO ADD ERROR HANDLING!!!!
unsafe extern "C" {
    unsafe fn gpio_to_desc(gpio: u32) -> *mut bindings::gpio_desc;
    unsafe fn gpiod_direction_output(desc: *mut bindings::gpio_desc, value: core::ffi::c_int) -> core::ffi::c_int;
    unsafe fn gpiod_direction_input(desc: *mut bindings::gpio_desc) -> core::ffi::c_int;
    unsafe fn gpiod_put(desc: *mut bindings::gpio_desc);
    unsafe fn udelay(usecs: core::ffi::c_ulong);
    unsafe fn gpiod_get_value(desc: *mut bindings::gpio_desc) -> core::ffi::c_int;
}
const GPIO_SDA: u32 = 514;
const GPIO_SCL: u32 = 515;
const DELAY:core::ffi::c_ulong =5;

struct I2CBasics{
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




    pub fn write_byte(&self,mut byte:u8)->bool{
        for i in (0..8){
            self.write_bit(byte&0x80!=0);
            byte<<=1
        }

        self.set_sda(true);
        self.set_scl(true);

        ack=!self.read_sda();//check if low

        self.set_scl(false);
        return ack
    }
    fn write_bit(&self,boolean:bool){
        self.set_sda(boolean);
        //send bit
        self.set_scl(true);
        //reset
        self.set_scl(false);

    }
    pub fn start(&self){
        //init
        self.set_sda(true);
        self.set_scl(true);
        
        self.set_sda(false);
        self.set_scl(false);
    }
    pub fn stop(&self){
        self.set_sda(false);
        self.set_scl(true);
        self.set_sda(true);
    }
    fn set_sda(&self, boolean:bool){
        if(boolean){
            unsafe{gpiod_direction_input(self.sda);}
        }
        else{
            unsafe{gpiod_direction_output(self.sda, 0);}
        }
        unsafe {udelay(DELAY);}
    }
    fn set_scl(&self, boolean:bool){

        if(boolean){
            unsafe{gpiod_direction_input(self.scl);}
        }
        else{
            unsafe{gpiod_direction_output(self.scl, 0);}
        }
        unsafe {udelay(DELAY);}
    }
    fn read_sda(&self)->bool{
        return unsafe{gpiod_get_value(self.sda)}!=0;
    }
    impl Drop for I2CBasics{
        fn drop(&mut self){
            unsafe{
                gpiod_put(self.sda);
                gpiod_put(self.scl);
            }
        }
    }
} 