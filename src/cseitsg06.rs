use libc;
use std::u8;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::time::Instant;

use rand;

const DEFAULT_BUFFER_SIZE: usize = 32 * 1024;

pub fn execute() {
    println!("executing procedure: CSE-ITSG-06");

    let filepath = "cseitsg06.data";
    let mut file_opts = OpenOptions::new();
    file_opts.read(true);
    file_opts.write(true);
    file_opts.create(true);
    file_opts.truncate(true);

    println!("using O_SYNC option with file");
    file_opts.custom_flags(libc::O_SYNC);

    let file_result = file_opts.open(filepath);
    if file_result.is_err() {
        println!("failed to create / open file {:?}, error: {:?}",
                 filepath,
                 file_result.err().unwrap());
        return;
    }

    // Pass 1 - write zeros
    let mut file = file_result.unwrap();
    let mut result = pass1(&mut file);
    if result.is_err() {
        println!("first pass failed, error: {:?}", result.err().unwrap());
        return;
    }

    // Pass 2 - write ones
    result = pass2(&mut file);
    if result.is_err() {
        println!("second pass failed, error: {:?}", result.err().unwrap());
        return;
    }

    // Pass 3 - write a random number from 0 to 255 and verify the write
    result = pass3(&mut file);
    if result.is_err() {
        println!("third pass failed, error: {:?}", result.err().unwrap());
        return;
    }
}

fn pass1(mut f: &mut File) -> Result<()> {
    let start = Instant::now();
    let buffer = [0; DEFAULT_BUFFER_SIZE];

    let count = try!(chunk_writes(&buffer, &mut f, false));
    try!(f.sync_all());
    println!("pass 1: wrote {:?} bytes of 0s in {:?} seconds",
             count,
             start.elapsed().as_secs());
    Ok(())
}

fn pass2(mut f: &mut File) -> Result<()> {
    try!(f.seek(SeekFrom::Start(0)));
    let start = Instant::now();
    let buffer = [u8::MAX; DEFAULT_BUFFER_SIZE];

    let count = try!(chunk_writes(&buffer, &mut f, false));
    try!(f.sync_all());
    println!("pass 2: wrote {:?} bytes of 1s in {:?} seconds",
             count,
             start.elapsed().as_secs());
    Ok(())
}

fn pass3(mut f: &mut File) -> Result<()> {
    try!(f.seek(SeekFrom::Start(0)));
    let start = Instant::now();
    let num = rand::random::<u8>();
    let buffer = [num; DEFAULT_BUFFER_SIZE];

    let count = try!(chunk_writes(&buffer, &mut f, true));
    try!(f.sync_all());
    println!("pass 3: wrote {:?} bytes of {:?} in {:?} seconds",
             count,
             num,
             start.elapsed().as_secs());
    println!("pass 3 completed with verification of each write.");
    Ok(())
}

fn chunk_writes(buf: &[u8], f: &mut File, verify: bool) -> Result<u64> {
    let mut count: u64 = 0;
    let mut ok: bool = true;
    let mut rbuf = [0; DEFAULT_BUFFER_SIZE];
    while ok {
        let res = f.write(&buf);
        ok = res.is_ok();
        if ok {
            let written = res.unwrap();
            if verify {
                try!(f.sync_data());
                try!(f.seek(SeekFrom::Current(written as i64 * -1)));
                let read = try!(f.read(&mut rbuf[..written]));
                if written != read || buf[..written] != rbuf[..read] {
                    return Err(Error::new(ErrorKind::InvalidData,
                                          "data read does not match what was written"));
                }
            }
            count += written as u64;
        } else {
            let e = res.err().unwrap();
            if !is_out_of_space_error(&e) {
                return Err(e);
            }
        }
        try!(f.sync_all());
    }
    Ok(count)
}

fn is_out_of_space_error(err: &Error) -> bool {
    match err.raw_os_error() {
        Some(ose) => ose == 28,
        None => false,
    }
}
