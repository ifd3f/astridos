AS = x86_64-elf-as
LD = x86_64-elf-ld

test.bin: arch/x86_64/link.ld ../target/target/debug/libastridos.a boot.a
	$(LD) -o $@ $(LINKFLAGS) -T $^

boot.a: arch/x86_64/boot.S
	$(AS) -o $@ $<

../target/target/debug/libastridos.a:
	cargo build
	cp ../target/debug/astridos
