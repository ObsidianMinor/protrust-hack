// simple sanity checks

mod shared;

use pretty_assertions::assert_eq;
use protrust::prelude::*;
use protrust::reflect::Descriptor;

use shared::gen::unittest_proto::{
    ForeignEnum,
    TestAllTypes as TestAllTypes2, 
    test_all_types::NestedEnum as NestedEnum2
};
use shared::gen::unittest_import_proto::ImportEnum;

#[test]
#[cfg(not(checked_size))]
fn empty_test_all_size_is_zero() {
    let m = TestAllTypes2::new();
    assert_eq!(m.calculate_size(), 0);
}

#[test]
#[cfg(checked_size)]
fn empty_test_all_size_is_zero() {
    let m = TestAllTypes2::new();
    assert_eq!(m.calculate_size(), Some(0));
}

#[test]
fn empty_test_all_is_equal() {
    let m = TestAllTypes2::new();
    assert_eq!(m, m);
}

#[test]
fn empty_test_all_output_is_none() -> shared::Result {
    let m = TestAllTypes2::new();
    let out = m.write_to_vec()?;
    assert_eq!(out.len(), 0);

    Ok(())
}

#[test]
fn empty_test_all_roundtrip_is_equal() -> shared::Result {
    let m = TestAllTypes2::new();
    let m2 =
        TestAllTypes2::read_new(&mut m.write_to_vec()?.as_slice())?;
    assert_eq!(m, m2);

    Ok(())
}


#[test]
fn merged_new_is_same_merged_from() -> shared::Result {
    let file = protrust::descriptor::FileDescriptorProto::descriptor().file().proto();
    let mut proto = protrust::descriptor::FileDescriptorProto::new();
    proto.merge(file);
    assert_eq!(&proto, file);

    Ok(())
}

#[test]
fn proto2_default_values() -> shared::Result {
    let m = TestAllTypes2::new();
    assert_eq!(m.optional_int32(), 0);
    assert_eq!(m.optional_int64(), 0);
    assert_eq!(m.optional_uint32(), 0);
    assert_eq!(m.optional_uint64(), 0);
    assert_eq!(m.optional_sint32(), 0);
    assert_eq!(m.optional_sint64(), 0);
    assert_eq!(m.optional_fixed32(), 0);
    assert_eq!(m.optional_fixed64(), 0);
    assert_eq!(m.optional_sfixed32(), 0);
    assert_eq!(m.optional_sfixed64(), 0);
    assert_eq!(m.optional_float(), 0.0);
    assert_eq!(m.optional_double(), 0.0);
    assert_eq!(m.optional_bool(), false);
    assert_eq!(m.optional_string(), "");
    assert_eq!(m.optional_bytes(), &[]);
    assert_eq!(m.optionalgroup(), None);
    assert_eq!(m.optional_nested_message(), None);
    assert_eq!(m.optional_foreign_message(), None);
    assert_eq!(m.optional_import_message(), None);
    assert_eq!(m.optional_nested_enum(), EnumValue::from(0));
    assert_eq!(m.optional_foreign_enum(), EnumValue::from(0));
    assert_eq!(m.optional_import_enum(), EnumValue::from(0));
    assert_eq!(m.optional_string_piece(), "");
    assert_eq!(m.optional_cord(), "");
    assert_eq!(m.optional_public_import_message(), None);
    assert_eq!(m.optional_lazy_message(), None);

    assert_eq!(m.repeated_int32().len(), 0);
    assert_eq!(m.repeated_int64().len(), 0);
    assert_eq!(m.repeated_uint32().len(), 0);
    assert_eq!(m.repeated_uint64().len(), 0);
    assert_eq!(m.repeated_sint32().len(), 0);
    assert_eq!(m.repeated_sint64().len(), 0);
    assert_eq!(m.repeated_fixed32().len(), 0);
    assert_eq!(m.repeated_fixed64().len(), 0);
    assert_eq!(m.repeated_sfixed32().len(), 0);
    assert_eq!(m.repeated_sfixed64().len(), 0);
    assert_eq!(m.repeated_float().len(), 0);
    assert_eq!(m.repeated_double().len(), 0);
    assert_eq!(m.repeated_bool().len(), 0);
    assert_eq!(m.repeated_string().len(), 0);
    assert_eq!(m.repeated_bytes().len(), 0);
    assert_eq!(m.repeatedgroup().len(), 0);
    assert_eq!(m.repeated_nested_message().len(), 0);
    assert_eq!(m.repeated_foreign_message().len(), 0);
    assert_eq!(m.repeated_import_message().len(), 0);
    assert_eq!(m.repeated_nested_enum().len(), 0);
    assert_eq!(m.repeated_foreign_enum().len(), 0);
    assert_eq!(m.repeated_import_enum().len(), 0);
    assert_eq!(m.repeated_string_piece().len(), 0);
    assert_eq!(m.repeated_cord().len(), 0);
    assert_eq!(m.repeated_lazy_message().len(), 0);

    assert_eq!(m.default_int32(), 41);
    assert_eq!(m.default_int64(), 42);
    assert_eq!(m.default_uint32(), 43);
    assert_eq!(m.default_uint64(), 44);
    assert_eq!(m.default_sint32(), -45);
    assert_eq!(m.default_sint64(), 46);
    assert_eq!(m.default_fixed32(), 47);
    assert_eq!(m.default_fixed64(), 48);
    assert_eq!(m.default_sfixed32(), 49);
    assert_eq!(m.default_sfixed64(), -50);
    assert_eq!(m.default_float(), 51.5);
    assert_eq!(m.default_double(), 52e3);
    assert_eq!(m.default_bool(), true);
    assert_eq!(m.default_string(), "hello");
    assert_eq!(m.default_bytes(), b"world");
    assert_eq!(m.default_nested_enum(), Defined(NestedEnum2::Bar));
    assert_eq!(m.default_foreign_enum(), Defined(ForeignEnum::ForeignBar));
    assert_eq!(m.default_import_enum(), Defined(ImportEnum::ImportBar));
    assert_eq!(m.default_string_piece(), "abc");
    assert_eq!(m.default_cord(), "123");

    assert_eq!(m.oneof_uint32(), None);
    assert_eq!(m.oneof_nested_message(), None);
    assert_eq!(m.oneof_string(), None);
    assert_eq!(m.oneof_bytes(), None);
    Ok(())
}