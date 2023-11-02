use std::{
    env,
    process::{Command, Output},
};

fn shutdown() -> Result<Output, std::io::Error> {
    return Command::new("shutdown").arg("0").output();
}

fn reboot() -> Result<Output, std::io::Error> {
    return Command::new("reboot").output();
}

fn main() {
    let mut args = env::args();

    match args.nth(1).as_ref() {
        Some(a) => match a.to_lowercase().as_str() {
            "ok" => match env::var("ROFI_INFO") {
                Ok(val) => match val.as_str() {
                    "shutdown" => {
                        let _ = shutdown();
                    }
                    "reboot" => {
                        let _ = reboot();
                    }
                    _ => {}
                },
                _ => {}
            },
            "reboot" => {
                println!("\0prompt\x1fAre you sure you want to reboot?");
                println!("OK\0info\x1freboot");
                println!("Cancel");
            }
            "shutdown" => {
                println!("\0prompt\x1fAre you sure you want to shutdown?");
                println!("OK\0info\x1fshutdown");
                println!("Cancel");
            }
            _ => {}
        },
        None => {
            println!("\0prompt\x1fSelect action");
            println!("Shutdown\0info\x1fshutdown");
            println!("Reboot\0info\x1freboot");
            println!("Cancel");
        }
    }
}
