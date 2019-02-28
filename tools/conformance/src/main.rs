#![feature(try_from)]

#[rustfmt::skip]
mod gen {
    include!(concat!(env!("OUT_DIR"), "/gen/mod.rs"));
}

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crate::gen::{conformance_proto::*, test_messages_proto2_proto as proto2_messages, test_messages_proto3_proto as proto3_messages};
use protrust::prelude::*;
use protrust::reflect::DescriptorPool;
use std::io::{Write, Read, stdin, stdout, ErrorKind};

fn main() -> Result<(), Box<std::error::Error>> {
    let stdin = stdin();
    let stdout = stdout();

    let mut stdinlock = stdin.lock();
    let mut stdoutlock = stdout.lock();

    loop {
        match stdinlock.read_i32::<LittleEndian>() {
            Ok(len) => {
                let mut take = (&mut stdinlock).take(len as u64);
                let response = process_request(ConformanceRequest::read_new(&mut take)?);
                stdoutlock.write_i32::<LittleEndian>(response.calculate_size())?;
                response.write(&mut stdoutlock)?;
                stdoutlock.flush()?;
            },
            Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => {
                break;
            },
            Err(e) => return Err(Box::new(e)),
        }
    }

    Ok(())
}

fn process_request(request: ConformanceRequest) -> ConformanceResponse {
    let pools = [proto2_messages::pool(), proto3_messages::pool()];
    let pool = DescriptorPool::build_from_pools(&pools);

    let mut msg_type = request.message_type().clone();
    msg_type.insert(0, '.');
    let descriptor: &protrust::reflect::MessageDescriptor;
    if let Some(found) = pool.find_message_by_name(&msg_type) {
        descriptor = found;
    } else {
        return runtime_error("Could not find message type in pool")
    }

    let mut instance = descriptor.new_message().unwrap();
    match request.payload() {
        ConformanceRequest_Payload::ProtobufPayload(bin) => {
            if let Err(err) = instance.merge_from_read(&mut bin.as_slice()) {
                return parse_error(&format!("Could not parse: {:?}", err))
            }
        },
        _ => return skip("Unsupported payload")
    }

    match request.requested_output_format() {
        EnumValue::Defined(WireFormat::Protobuf) => {
            match instance.write_to_vec() {
                Ok(payload) => protobuf_payload(payload),
                Err(err) => serialize_error(&format!("Could not serialize: {:?}", err))
            }
        },
        _ => skip("Unsupported output")
    }
}

fn skip(reason: &str) -> ConformanceResponse {
    respond(ConformanceResponse_Result::Skipped(reason.to_string()))
}

fn runtime_error(reason: &str) -> ConformanceResponse {
    respond(ConformanceResponse_Result::RuntimeError(reason.to_string()))
}

fn parse_error(reason: &str) -> ConformanceResponse {
    respond(ConformanceResponse_Result::ParseError(reason.to_string()))
}

fn serialize_error(reason: &str) -> ConformanceResponse {
    respond(ConformanceRequest_Result::SerializeError(reason.to_string()))
}

fn protobuf_payload(payload: Vec<u8>) -> ConformanceResponse {
    respond(ConformanceRequest_Result::Payload(payload))
}

fn respond(result: ConformanceResponse_Result) -> ConformanceResponse {
    let mut response = ConformanceResponse::new();
    *response.result_mut() = result;
    response
}