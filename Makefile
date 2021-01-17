# s7-1200-web-control
# Commandline utility
# SPDX-License-Identifier: GPL-3.0-only
# Copyright (C) 2020 Benjamin Schilling

.PHONY: all build install clean uninstall

all: clean build

deps:
		curl https://sh.rustup.rs -sSf | sh -s -- -y && export PATH=$(PATH):$(HOME)/.cargo/bin && rustup toolchain install nightly && rustup default nightly

build: deps
		export PATH=$(PATH):$(HOME)/.cargo/bin && cargo build --release

install: 
		mkdir -p $(DESTDIR)/opt/s7-1200-web-control/
		cp -r target/release/s7-1200-web-control  $(DESTDIR)/opt/s7-1200-web-control/s7-1200-web-control
		mkdir -p $(DESTDIR)/etc/profile.d/
		printf '#!/bin/bash\nexport PATH=$$PATH:/opt/s7-1200-web-control' > $(DESTDIR)/etc/profile.d/101-s7-1200-web-control.sh
		chmod 755 $(DESTDIR)/etc/profile.d/101-s7-1200-web-control.sh

clean:
		rm -f -r target

uninstall:
		rm -r  $(DESTDIR)/opt/s7-1200-web-control