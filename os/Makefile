target := riscv64imac-unknown-none-elf
mode := debug
kernel := target/$(target)/$(mode)/os
tkernel := target/$(target)/$(mode)/deps/os-ec6796421cb4ed35
bin := target/$(target)/$(mode)/kernel.bin
tbin := target/$(target)/$(mode)/tkernel.bin

objdump := rust-objdump --arch-name=riscv64
objcopy := rust-objcopy --binary-architecture=riscv64

.PHONY: kernel build clean qemu run env

env:
	cargo install cargo-binutils
	rustup component add llvm-tools-preview rustfmt
	rustup target add $(target)

kernel:
	cargo build

tkernel:
	cargo xtest --bin os --no-run

$(bin): kernel
	$(objcopy) $(kernel) --strip-all -O binary $@

$(tbin): tkernel
	$(objcopy) $(tkernel) --strip-all -O binary $@

asm:
	$(objdump) -d $(kernel) | less

build: $(bin)
tbuild: $(tbin)

clean:
	cargo clean

qemu:
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios default \
		-s \
		-S \
		-device loader,file=$(bin),addr=0x80200000

tqemu:
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios default \
		-s \
		-S \
		-device loader,file=$(tbin),addr=0x80200000
run:  build qemu
test: tbuild tqemu
