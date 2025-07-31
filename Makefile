# SPDX-License-Identifier: GPL-2.0

JOBS=$(shell nproc)

LLVM ?= 1

KDIR ?= "/lib/modules/$(shell uname -r)/build"

PWD := "$(shell pwd)"

BUILD_DIR := "$(PWD)/build"

default:
	# Create build directory if it doesn't exist
	mkdir -p $(BUILD_DIR)
	$(MAKE) -C $(KDIR) M=$(PWD) MO=$(BUILD_DIR) LLVM=$(LLVM) -j$(JOBS)

clean:
	make -C $(KDIR) M=$(PWD) MO=$(BUILD_DIR) LLVM=$(LLVM) clean

rust-analyzer:
	make -C $(KDIR) M=$(PWD) LLVM=$(LLVM) rust-analyzer
