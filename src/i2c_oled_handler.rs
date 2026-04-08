use crate::i2c_basic_components::{I2CBasics, I2cError};

pub struct I2COled{
    i2c:I2CBasics
}
impl I2COled{//TODO ADD WRITE!!
    pub fn new ()->Self{
        return I2COled{i2c:I2CBasics::new()};
    }
    pub fn init(&self)->Result<(),I2cError>{
        /*
        sends the init sequence to set display brightness and internal hardware settings
        init bytes hardcoded for now 
         */
        let init_bytes:[u8;13]=[0xAE,0xA1,0xC8,0xA8,0x3F,0xD5,0x50,0x81,0x80,0xAD,0x8B,0xA6,0xAF];
        let oled_addr:u8=0x78;
        let oled_command:u8=0x00;

        self.i2c.start();

        self.i2c.write_byte(oled_addr)?;
        self.i2c.write_byte(oled_command)?;
        
        for command in init_bytes{
            self.i2c.write_byte(command)?
        }
        Ok(())
    }
}