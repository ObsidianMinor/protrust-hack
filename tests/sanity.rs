// simple sanity checks

mod shared;

use protrust::prelude::*;

#[test]
#[cfg(not(checked_size))]
fn empty_test_all_size_is_zero() {
    let m = shared::gen::unittest_proto::TestAllTypes::new();
    assert_eq!(m.calculate_size(), 0);
}

#[test]
#[cfg(checked_size)]
fn empty_test_all_size_is_zero() {
    let m = shared::gen::unittest_proto::TestAllTypes::new();
    assert_eq!(m.calculate_size(), Some(0));
}

#[test]
fn empty_test_all_is_equal() {
    let m = shared::gen::unittest_proto::TestAllTypes::new();
    assert_eq!(m, m);
}

#[test]
fn empty_test_all_output_is_none() -> shared::Result {
    let m = shared::gen::unittest_proto::TestAllTypes::new();
    let out = m.write_to_vec()?;
    assert_eq!(out.len(), 0);

    Ok(())
}

#[test]
fn empty_test_all_roundtrip_is_equal() -> shared::Result {
    let m = shared::gen::unittest_proto::TestAllTypes::new();
    let m2 =
        shared::gen::unittest_proto::TestAllTypes::read_new(&mut m.write_to_vec()?.as_slice())?;
    assert_eq!(m, m2);

    Ok(())
}
