use crate::i2c_basic_components::{I2CBasics, I2cError};
const OLED_ADDR:u8 =0x78;
const OLED_PAGES:u8=8;
const OLED_PAGE_BYTES:usize=128;
use kernel::prelude::*;
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
        let oled_command:u8=0x00;

        self.i2c.start();

        self.i2c.write_byte(OLED_ADDR)?;
        self.i2c.write_byte(oled_command)?;
        
        for command in init_bytes{
            self.i2c.write_byte(command)?
        }
        self.i2c.stop();
        Ok(())
    }
    pub fn write(&self,bytes:&[u8])-> Result<(),I2cError>{ 
        /*
        writes raw data to the oled display
        expects a byte slice of size screenWidth*screenHeight
         */
        
        if(bytes.len()<OLED_PAGES as usize * OLED_PAGE_BYTES){
            return Err(I2cError::InvalidBytes)
        }

        for page in 0..OLED_PAGES{

            self.i2c.start();
            self.i2c.write_byte(OLED_ADDR)?;
            self.i2c.write_byte(0x00)?;//command mode
            self.i2c.write_byte(0xB0+page)?;//set offset to start of the page
            self.i2c.write_byte(0x02)?;
            self.i2c.write_byte(0x10)?;
            self.i2c.stop();
            //switch to data mode
            self.i2c.start();
            self.i2c.write_byte(OLED_ADDR)?;
            self.i2c.write_byte(0x40)?;
            for offset in 0..OLED_PAGE_BYTES{
                self.i2c.write_byte(bytes[(((page as usize)*OLED_PAGE_BYTES)+offset)])?;
            }
            self.i2c.stop();
        }
        pr_info!("written to screen!\n");
        Ok(())
    }
}