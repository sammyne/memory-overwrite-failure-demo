use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{process, thread};

fn main() {
    println!("this is Alice");

    let foo = "This is some text from Alice";
    println!("originally foo is '{foo}'");

    let hacker_path = {
        let v = PathBuf::from(std::env::args().next().unwrap());
        v.parent()
            .expect("get parent path of the current process")
            .join("hacker")
    };

    println!("Now execute");
    println!(
        "  sudo {} {} {:p} {}",
        hacker_path.to_string_lossy(),
        process::id(),
        foo,
        foo.len()
    );

    println!("Wait for /tmp/ready.txt");
    while !Path::new("/tmp/ready.txt").exists() {
        thread::sleep(Duration::from_secs(1));
    }

    println!("now foo is '{foo}'");
}
