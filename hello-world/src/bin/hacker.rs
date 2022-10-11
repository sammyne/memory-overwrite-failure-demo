use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom, Write};

fn main() {
    let args: Vec<String> = std::env::args().take(4).collect();
    if args.len() != 4 {
        println!("{} pid addr length", args[0]);
        std::process::exit(1);
    }

    let pid: u32 = args[1].parse().expect("parse pid");
    let addr = u64::from_str_radix(args[2].trim_start_matches("0x"), 16).expect("parse addr");
    let length: usize = args[3].parse().expect("parse length");

    let mem_path = format!("/proc/{pid}/mem");
    println!("opening {mem_path}, address is {addr:x}");
    let mut mem = File::options()
        .read(true)
        .write(true)
        .open(&mem_path)
        .expect(&format!("open '{mem_path}'"));

    let mut buf = String::new();
    match check_read(&mut mem, pid, addr, length) {
        Ok(v) => buf = v,
        Err(err) => println!("read failed: {err}"),
    }

    check_write(&mut mem, addr, buf).expect("write failed");

    fs::write("/tmp/ready.txt", "ok").expect("generate /tmp/ready.txt");
}

fn check_read(
    mem: &mut File,
    pid: u32,
    addr: u64,
    expected_buf_length: usize,
) -> Result<String, String> {
    let _ = mem
        .seek(SeekFrom::Start(addr))
        .expect(&format!("offset mem to {addr}"));

    let mut buf = vec![0u8; expected_buf_length];
    let _ = mem
        .read_exact(buf.as_mut_slice())
        .map_err(|err| format!("read buf: {}", err))?;

    let old = String::from_utf8(buf).expect("parse buf as string");
    println!("string at 0x{addr:x} in process {pid} is:\n{}", old);

    let out = old.replace("Alice", "Bob:)");

    Ok(out)
}

fn check_write(mem: &mut File, addr: u64, data: String) -> Result<(), String> {
    let _ = mem
        .seek(SeekFrom::Start(addr))
        .expect(&format!("offset mem to {addr}"));
    let _ = mem
        .write_all(data.as_bytes())
        .map_err(|err| format!("write back new string: {err}"))?;

    Ok(())
}
