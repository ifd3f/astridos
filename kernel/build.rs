use std::process::Command;

fn main() {
    let linker_script = "arch/x86_64/link.ld";
    let entrypoint = "arch/x86_64/boot.S";
    Command::new("echo")
        .arg("123")
        .spawn()
        .expect("failed to spawn process");
}
