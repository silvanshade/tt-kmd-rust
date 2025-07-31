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

ifeq ("$(LLVM)", "1")
  LLVM_ARG=LLVM=1
else
  LLVM_ARG=
endif

JOBS = $(shell nproc)
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
	$(MAKE) -C $(KDIR) M=$(PWD) MO=$(BUILD_DIR) $(LLVM_ARG) -j$(JOBS)

check:
	$(MAKE) default $(MAKEOVERRIDES) CLIPPY=1 KRUSTFLAGS+=-Dwarnings

clean:
	make -C $(KDIR) M=$(PWD) MO=$(BUILD_DIR) $(LLVM_ARG) clean

PHONY += fmt
fmt: rustfmt

PHONY += klint
klint:
	LD_LIBRARY_PATH=$$(rustup run 1.88.0 bash -c 'echo $$LD_LIBRARY_PATH') \
	$(MAKE) RUSTC=klint $(filter-out klint,$(MAKECMDGOALS))

PHONY += rustavailable
rustavailable:
	make -C $(KDIR) M=$(PWD) $(LLVM_ARG) rustavailable

PHONY += rustfmt
rustfmt:
	$(Q)find $(srctree) $(RCS_FIND_IGNORE) -type f -a -name '*.rs' -a ! -name '*generated*' -print \
	    | xargs $(RUSTFMT) $(rustfmt_flags)

PHONY += rustfmtcheck
rustfmtcheck: rustfmt_flags = --check
rustfmtcheck: rustfmt

PHONY += rust-analyzer
rust-analyzer:
	make -C $(KDIR) M=$(PWD) $(LLVM_ARG) rust-analyzer

PHONY += rust-analyzer-fmt
rust-analyzer-fmt:
	@$(RUSTFMT)
