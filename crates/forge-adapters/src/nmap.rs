use std::process::Command;

fn main() {
    let output = Command::new("nmap")
        .args(["-oX", "-", "localhost"])
        .output()
        .expect("Failed to execute nmap");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}