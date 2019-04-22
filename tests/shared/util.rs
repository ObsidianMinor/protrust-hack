#![allow(dead_code)]

use super::gen;
use protrust::prelude::*;

pub fn make_test_all_types_proto2() -> gen::unittest_proto::TestAllTypes {
    let mut msg = gen::unittest_proto::TestAllTypes::new();
    msg.set_optional_int32(1);
    msg.set_optional_int64(2);
    msg.set_optional_uint32(3);
    msg.set_optional_uint64(4);
    msg.set_optional_sint32(5);
    msg.set_optional_sint64(6);
    msg.set_optional_fixed32(7);
    msg.set_optional_fixed64(8);
    msg.set_optional_sfixed32(9);
    msg.set_optional_sfixed64(10);
    msg.set_optional_float(11.11);
    msg.set_optional_double(12.12);
    msg.set_optional_bool(true);
    msg.optional_string_mut().push_str("Hello world!");
    msg.optional_bytes_mut().extend(b"Hello world!");
    msg.optionalgroup_mut().set_a(25);

    msg
}
