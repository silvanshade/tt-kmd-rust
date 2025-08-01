# SPDX-License-Identifier: GPL-2.0

ifeq ("$(origin V)", "command line")
  KBUILD_VERBOSE = $(V)
endif

quiet = quiet_
Q = @

ifneq ($(findstring 1, $(KBUILD_VERBOSE)),)
  quiet =
  Q =
endif

JOBS = $(shell nproc)
LLVM ?= 1
KDIR ?= "/lib/modules/$(shell uname -r)/build"
PWD := "$(shell pwd)"
BUILD_DIR := "$(PWD)/build"

RUST_TOOLCHAIN_NIGHTLY = 2025-08-01
RUSTDOC = rustdoc
RUSTFMT = rustup run "nightly-$(RUST_TOOLCHAIN_NIGHTLY)" rustfmt

PHONY :=

default:
	# Create build directory if it doesn't exist
	mkdir -p $(BUILD_DIR)
	$(MAKE) -C $(KDIR) M=$(PWD) MO=$(BUILD_DIR) LLVM=$(LLVM) -j$(JOBS)

check: default

clean:
	make -C $(KDIR) M=$(PWD) MO=$(BUILD_DIR) LLVM=$(LLVM) clean

PHONY += fmt
fmt: rustfmt

PHONY += rustavailable
rustavailable:
	make -C $(KDIR) M=$(PWD) LLVM=$(LLVM) rustavailable

PHONY += rustfmt
rustfmt:
	$(Q)find $(srctree) $(RCS_FIND_IGNORE) -type f -a -name '*.rs' -a ! -name '*generated*' -print \
	    | xargs $(RUSTFMT) $(rustfmt_flags)

PHONY += rustfmtcheck
rustfmtcheck: rustfmt_flags = --check
rustfmtcheck: rustfmt

PHONY += rust-analyzer
rust-analyzer:
	make -C $(KDIR) M=$(PWD) LLVM=$(LLVM) rust-analyzer

PHONY += rust-analyzer-fmt
rust-analyzer-fmt:
	@$(RUSTFMT)
