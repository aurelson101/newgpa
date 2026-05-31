use anyhow::Result;
use std::process::Command;

pub fn print_doctor_report() -> Result<()> {
    println!("NewGPA doctor");
    println!("high-security defaults: enabled");
    println!("network by default: disabled");
    println!("post-quantum lab: disabled by default");
    for tool in ["gpg", "gpgsm", "gpg-agent", "pinentry"] {
        let status = Command::new("sh")
            .arg("-c")
            .arg(format!("command -v {tool} >/dev/null 2>&1"))
            .status();
        println!(
            "{tool}: {}",
            if matches!(status, Ok(s) if s.success()) {
                "found"
            } else {
                "missing"
            }
        );
    }
    Ok(())
}
