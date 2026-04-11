



my first kernel module/driver!

[demo](https://www.youtube.com/watch?v=q4vWCGMs8_4)

using the bit-banging technique to control my i2c oled display on my raspberry pi
![](https://i.postimg.cc/Fs1nSrJL/Screenshot-2026-04-11-040517.png)
# what is bitbanging?
bitbanging is the technique of controling hardware components by manually turning gpio pins on/off according to a certain communication protocol (i2c\spi etc..)- 

this project is a driver that controls an oled display that uses the i2c dta bus

## for hackclub shipwrights:
if you intend to install and test this driver make sure you connect sda to gpio 2 and scl to gpio 3


you must ensure your raspi kernel is this version:6.12.79-v8 


to check version:
```bash
uname -r
````
AND IT MUST HAVE RUST SUPPORT
make sure your oled display is SH1106


to run the driver:

first download the oled_i2c_bitbang.ko file from this repo


then on your raspi: 
```bash
sudo insmod oled_i2c_bitbang.ko
```
and to write data to the display (in userspace) you just need to write to /dev/oled

every bit in your input represents a pixel 0 is off and 1 is on, the driver takes 1024 bytes of data (an SH1106 oled is 128 by 64 pixels=8192 bits=1024 bytes)

the writing is started from left to right and TOP TO BOTTOM(not horizontaly)

make sure you send at least 1024 bytes or else the driver will reject the write (if you send more its okay but it will take just the first 1024)


### im currently working on a library to make interaction with the display even more user friendly
