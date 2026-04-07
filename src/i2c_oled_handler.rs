use crate::i2c_basic_components::I2CBasics;
struct I2COled{
    i2c:&I2CBasics
}
impl I2COled{
    pub fn new (){
        I2COled{i2c:I2CBasics::new()};
    }
}