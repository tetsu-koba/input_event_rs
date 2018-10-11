#![allow(dead_code)]

mod input;
use input::*;

use std::io;
use std::io::Read;
use std::fs::File;

fn read_input_event(f: &mut File) -> io::Result<(input_event)> {
    let mut buf = [0; std::mem::size_of::<input_event>()];
    f.read_exact(&mut buf)?;
    let ie: input_event = unsafe { std::mem::transmute(buf) };
    Ok(ie)
}

fn dump_input_event(fname: &str) {
    let mut f = File::open(fname).expect("open failed.");
    loop {
        match read_input_event(&mut f).expect("read error.") {
            input_event { type_, code, value, .. } if type_ as u32 == EV_KEY => {
                match code as u32 {
                    KEY_MUTE => {
                        if value == 1 {
                            println!("mute")
                        }
                    }
                    KEY_VOLUMEDOWN => {
                        if value != 0 {
                            println!("volume down")
                        }
                    }
                    KEY_VOLUMEUP => {
                        if value != 0 {
                            println!("volume up")
                        }
                    }
                    _ => {}
                }
            }
            //ie => println!("input_event ={:?}", ie),
            _ => {}
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} /dev/input/event0", &args[0]);
        return;
    }
    dump_input_event(&args[1]);
}
