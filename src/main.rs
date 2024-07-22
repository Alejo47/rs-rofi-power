use std::{
    env,
    process::{Command, Output},
};

fn shutdown() -> Result<Output, std::io::Error> {
    Command::new("shutdown").arg("0").output()
}

fn reboot() -> Result<Output, std::io::Error> {
    Command::new("reboot").output()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("\0prompt\x1fSelect action");
        println!("Shutdown\0info\x1fshutdown");
        println!("Reboot\0info\x1freboot");
        println!("Cancel");
        return;
    }

    match args[1].to_lowercase().as_str() {
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
        _ => {
            println!("\0prompt\x1fSelect action");
            println!("Shutdown\0info\x1fshutdown");
            println!("Reboot\0info\x1freboot");
            println!("Cancel");
        }
    }
}
