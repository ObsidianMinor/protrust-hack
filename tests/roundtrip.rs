#![feature(try_from)]

mod shared;

use protrust::prelude::*;

#[test]
fn roundtrip_proto2_test_all_types() -> shared::Result {
    let value = shared::util::make_test_all_types_proto2();
    let serialized = value.write_to_vec()?;
    let deserialized = shared::gen::unittest_proto::TestAllTypes::read_new(&mut serialized.as_slice())?;
    assert_eq!(value, deserialized);
    Ok(())
}