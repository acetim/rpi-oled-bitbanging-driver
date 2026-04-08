use crate::i2c_basic_components::I2CBasics;
pub struct I2COled{
    i2c:I2CBasics
}
impl I2COled{//TODO ADD WRITE!!
    pub fn new ()->Self{
        return I2COled{i2c:I2CBasics::new()};
    }
}