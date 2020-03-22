extern crate flips;

const DATA1: &[u8] = include_bytes!("data/data1.bin");
const DATA2: &[u8] = include_bytes!("data/data2.bin");

const PATCH_1TO2: &[u8] = include_bytes!("data/patch1to2.ips");
const PATCH_2TO1: &[u8] = include_bytes!("data/patch2to1.ips");

#[test]
fn test_apply_correct() {
    let output = flips::IpsPatch::new(PATCH_1TO2).apply(DATA1).unwrap();
    assert_eq!(output.as_ref(), DATA2);
    let output = flips::IpsPatch::new(PATCH_2TO1).apply(DATA2).unwrap();
    assert_eq!(output.as_ref(), DATA1);
}

#[test]
fn test_apply_to_output() {
    let result = flips::IpsPatch::new(PATCH_1TO2).apply(DATA2);
    assert_eq!(result.unwrap_err(), flips::Error::ToOutput);
    let result = flips::IpsPatch::new(PATCH_2TO1).apply(DATA1);
    assert_eq!(result.unwrap_err(), flips::Error::ToOutput);
}

#[test]
fn test_create_identical() {
    let result = flips::IpsBuilder::new().source(DATA1).target(DATA1).build();
    assert_eq!(result.unwrap_err(), flips::Error::Identical);
    let result = flips::IpsBuilder::new().source(DATA2).target(DATA2).build();
    assert_eq!(result.unwrap_err(), flips::Error::Identical);
}
