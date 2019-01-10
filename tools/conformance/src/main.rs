#![feature(try_from)]

#[rustfmt::skip]
#[allow(unused_variables, dead_code, non_camel_case_types, non_snake_case, unreachable_patterns)]
mod gen {
    include!(concat!(env!("OUT_DIR"), "/gen/mod.rs"));
}

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crate::gen::conformance_proto::*;
use protrust::prelude::*;
use std::io::{Read, stdin, stdout, ErrorKind};

fn main() {
    let stdin = stdin();
    let stdout = stdout();

    let mut stdinlock = stdin.lock();
    let mut stdoutlock = stdout.lock();

    loop {
        match stdinlock.read_i32::<LittleEndian>() {
            Ok(len) => {
                let mut take = (&mut stdinlock).take(len as u64);
                let response = process_request(ConformanceRequest::read_new(&mut take).expect("Could not read request"));
                stdoutlock.write_i32::<LittleEndian>(response.calculate_size()).expect("Could not write to stdout");
                response.write(&mut stdoutlock).expect("Could not write response to stdout");
            },
            Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => {
                break;
            },
            Err(e) => {
                panic!("{}", e)
            },
        }
    }
}

fn process_request(request: ConformanceRequest) -> ConformanceResponse {
    match request.payload() {
        ConformanceRequest_Payload::ProtobufPayload(bin) => {
            unimplemented!()
        },
        _ => {
            let mut response = ConformanceResponse::new();
            *response.result_mut() = ConformanceResponse_Result::Skipped("Unsupported test".to_string());
            response
        }
    }
}