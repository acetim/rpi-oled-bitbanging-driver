KDIR ?= $(HOME)/linux
ARCH := arm64
CROSS_COMPILE := aarch64-linux-gnu-
LLVM := 1

obj-m := oled_i2c_bitbang.o
oled_i2c_bitbang-objs := src/lib.o

all:
	$(MAKE) -C $(KDIR) M=$(PWD) \
		ARCH=$(ARCH) \
		CROSS_COMPILE=$(CROSS_COMPILE) \
		LLVM=$(LLVM) \
		modules

clean:
	$(MAKE) -C $(KDIR) M=$(PWD) clean
