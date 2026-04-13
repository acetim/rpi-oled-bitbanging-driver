



my first kernel module/driver!

[demo](https://www.youtube.com/watch?v=q4vWCGMs8_4)

using the bit-banging technique to control my i2c oled display on my raspberry pi
![](https://i.postimg.cc/Fs1nSrJL/Screenshot-2026-04-11-040517.png)
# what is bitbanging?
bitbanging is the technique of controling hardware components by manually turning gpio pins on/off according to a certain communication protocol (i2c\spi etc..)- 

this project is a driver that controls an oled display that uses the i2c dta bus

# GPIO 2/3 IS ALREADY IN USE? NO PROBLEM!
just change the constants in i2c_basic_components.rs GPIO_SCL and GPIO_SDA to be your gpio pin number and recompile the program!

### im currently working on a library to make interaction with the display even more user friendly
