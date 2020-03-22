extern crate flips;

const DATA1: &[u8] = include_bytes!("data/data1.bin");
const DATA2: &[u8] = include_bytes!("data/data2.bin");
const DATA3: &[u8] = include_bytes!("data/data3.bin");

const PATCH_1TO2: &[u8] = include_bytes!("data/patch1to2.bps");
const PATCH_2TO1: &[u8] = include_bytes!("data/patch2to1.bps");

#[test]
fn test_apply_correct() {
    let output = flips::BpsPatch::new(PATCH_1TO2).apply(DATA1).unwrap();
    assert_eq!(output.as_ref(), DATA2);
    let output = flips::BpsPatch::new(PATCH_2TO1).apply(DATA2).unwrap();
    assert_eq!(output.as_ref(), DATA1);
}

#[test]
fn test_apply_to_output() {
    let result = flips::BpsPatch::new(PATCH_1TO2).apply(DATA2);
    assert_eq!(result.unwrap_err(), flips::Error::ToOutput);
    let result = flips::BpsPatch::new(PATCH_2TO1).apply(DATA1);
    assert_eq!(result.unwrap_err(), flips::Error::ToOutput);
}

#[test]
fn test_apply_not_this() {
    let result = flips::BpsPatch::new(PATCH_1TO2).apply(DATA3);
    assert_eq!(result.unwrap_err(), flips::Error::NotThis);
    let result = flips::BpsPatch::new(PATCH_2TO1).apply(DATA3);
    assert_eq!(result.unwrap_err(), flips::Error::NotThis);
}

#[test]
fn test_apply_invalid() {
    let study = flips::BpsPatch::new(DATA1).apply(DATA2);
    assert_eq!(study.unwrap_err(), flips::Error::Invalid);
}
