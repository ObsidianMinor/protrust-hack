#[rustfmt::skip]
#[allow(non_camel_case_types, dead_code, non_snake_case)]
mod gen {
    include!(concat!(env!("OUT_DIR"), "/gen/mod.rs"));
}

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crate::gen::{
    conformance_proto::{
        WireFormat,
        ConformanceRequest,
        conformance_request::Payload,
        ConformanceResponse,
        conformance_response::Result as ConformanceResult
    },
    test_messages_proto2_proto as proto2_messages,
    test_messages_proto3_proto as proto3_messages
};
use protrust::prelude::*;
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
    let mut instance: Box<dyn CodedMessage> = 
        match &**request.message_type() {
            "protobuf_test_messages.proto2.TestAllTypesProto2" => Box::new(proto2_messages::TestAllTypesProto2::with_registry(Some(gen::extensions()))),
            "protobuf_test_messages.proto3.TestAllTypesProto3" => Box::new(proto3_messages::TestAllTypesProto3::new()),
            _ => return runtime_error("unknown request payload type")
        };

    match request.payload() {
        Payload::ProtobufPayload(bin) => {
            let mut slice = bin.as_slice();
            let mut input = protrust::io::CodedInput::new(&mut slice).with_registry(Some(gen::extensions()));
            if let Err(err) = instance.merge_from(&mut input) {
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
    respond(ConformanceResult::Skipped(reason.to_string()))
}

fn runtime_error(reason: &str) -> ConformanceResponse {
    respond(ConformanceResult::RuntimeError(reason.to_string()))
}

fn parse_error(reason: &str) -> ConformanceResponse {
    respond(ConformanceResult::ParseError(reason.to_string()))
}

fn serialize_error(reason: &str) -> ConformanceResponse {
    respond(ConformanceResult::SerializeError(reason.to_string()))
}

fn protobuf_payload(payload: Vec<u8>) -> ConformanceResponse {
    respond(ConformanceResult::ProtobufPayload(payload))
}

fn respond(result: ConformanceResult) -> ConformanceResponse {
    let mut response = ConformanceResponse::new();
    *response.result_mut() = result;
    response
}