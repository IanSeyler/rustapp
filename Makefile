LD=ld
RUSTC=rustc
NASM=nasm
QEMU=qemu-system-x86_64

all: rust.app

.SUFFIXES: .o .rs

.PHONY: clean

.rs.o:
	$(RUSTC) -O --crate-type lib -o $@ --emit obj $<

rust.app: rust.o
	$(LD) -T app.ld -o rust.app rust.o
clean:
	rm -f *.bin *.o *.img *.sys
