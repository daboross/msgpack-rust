use msgpack::{encode::*, Marker};

#[test]
fn pass_pack_pfix() {
    let mut buf = [0x00];

    write_pfix(&mut &mut buf[..], 127).ok().unwrap();

    assert_eq!([0x7f], buf);
}

#[test]
fn fail_pack_pfix_too_small_buffer() {
    let mut buf = [];

    write_pfix(&mut &mut buf[..], 127).err().unwrap();
}

#[test]
#[should_panic(expected = "assertion failed")]
fn fail_pack_pfix_too_large() {
    let mut buf = [0x00];

    write_pfix(&mut &mut buf[..], 128).ok().unwrap();
}

#[test]
fn pass_pack_u8() {
    let mut buf = [0x00, 0x00];

    write_u8(&mut &mut buf[..], 255).ok().unwrap();

    assert_eq!([0xcc, 0xff], buf);
}

#[test]
fn pass_pack_u16() {
    let mut buf = [0x00, 0x00, 0x00];

    write_u16(&mut &mut buf[..], 65535).ok().unwrap();

    assert_eq!([0xcd, 0xff, 0xff], buf);
}

#[test]
fn pass_pack_u32() {
    let mut buf = [0x00, 0x00, 0x00, 0x00, 0x00];

    write_u32(&mut &mut buf[..], 4294967295).ok().unwrap();

    assert_eq!([0xce, 0xff, 0xff, 0xff, 0xff], buf);
}

#[test]
fn pass_pack_u64() {
    let mut buf = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    write_u64(&mut &mut buf[..], 18446744073709551615)
        .ok()
        .unwrap();

    assert_eq!([0xcf, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff], buf);
}

#[test]
fn pass_pack_nfix() {
    let mut buf = [0x00];

    write_nfix(&mut &mut buf[..], -32).ok().unwrap();

    assert_eq!([0xe0], buf);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn fail_pack_nfix_too_large() {
    let mut buf = [0x00];

    write_nfix(&mut &mut buf[..], 0).ok().unwrap();
}

#[test]
#[should_panic(expected = "assertion failed")]
fn fail_pack_nfix_too_small() {
    let mut buf = [0x00];

    write_nfix(&mut &mut buf[..], -33).ok().unwrap();
}

#[test]
fn pass_pack_i8() {
    let mut buf = [0x00, 0x00];

    write_i8(&mut &mut buf[..], -128).ok().unwrap();

    assert_eq!([0xd0, 0x80], buf);
}

#[test]
fn pass_pack_i16() {
    let mut buf = [0x00, 0x00, 0x00];

    write_i16(&mut &mut buf[..], -32768).ok().unwrap();

    assert_eq!([0xd1, 0x80, 0x00], buf);
}

#[test]
fn pass_pack_i32() {
    let mut buf = [0x00, 0x00, 0x00, 0x00, 0x00];

    write_i32(&mut &mut buf[..], -2147483648).ok().unwrap();

    assert_eq!([0xd2, 0x80, 0x00, 0x00, 0x00], buf);
}

#[test]
fn pass_pack_i64() {
    let mut buf = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    write_i64(&mut &mut buf[..], -9223372036854775808)
        .ok()
        .unwrap();

    assert_eq!([0xd3, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], buf);
}

#[test]
fn pass_pack_uint_fix() {
    let mut buf = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    assert_eq!(
        Marker::FixPos(127),
        write_uint(&mut &mut buf[..], 127).ok().unwrap()
    );

    assert_eq!([0x7f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], buf);
}

#[test]
fn pass_pack_uint_u8() {
    let mut buf = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    assert_eq!(Marker::U8, write_uint(&mut &mut buf[..], 255).ok().unwrap());

    assert_eq!([0xcc, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], buf);
}

#[test]
fn pass_pack_uint_u16() {
    let mut buf = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    assert_eq!(
        Marker::U16,
        write_uint(&mut &mut buf[..], 65535).ok().unwrap()
    );

    assert_eq!([0xcd, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], buf);
}

#[test]
fn pass_pack_uint_u32() {
    let mut buf = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    assert_eq!(
        Marker::U32,
        write_uint(&mut &mut buf[..], 4294967295).ok().unwrap()
    );

    assert_eq!([0xce, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00], buf);
}

#[test]
fn pass_pack_uint_u64() {
    let mut buf = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    assert_eq!(
        Marker::U64,
        write_uint(&mut &mut buf[..], 18446744073709551615)
            .ok()
            .unwrap()
    );

    assert_eq!([0xcf, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff], buf);
}

#[test]
fn pass_pack_sint_fix() {
    let mut buf = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    assert_eq!(
        Marker::FixNeg(-32),
        write_sint(&mut &mut buf[..], -32).ok().unwrap()
    );

    assert_eq!([0xe0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], buf);
}

#[test]
fn pass_pack_sint_i8_min() {
    let mut buf = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    assert_eq!(
        Marker::I8,
        write_sint(&mut &mut buf[..], -128).ok().unwrap()
    );

    assert_eq!([0xd0, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], buf);
}

#[test]
fn pass_pack_sint_i16_min() {
    let mut buf = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    assert_eq!(
        Marker::I16,
        write_sint(&mut &mut buf[..], -32768).ok().unwrap()
    );

    assert_eq!([0xd1, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], buf);
}

#[test]
fn pass_pack_sint_i16_max() {
    let mut buf = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    assert_eq!(
        Marker::U16,
        write_sint(&mut &mut buf[..], 32767).ok().unwrap()
    );

    assert_eq!([0xcd, 0x7f, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], buf);
}

#[test]
fn pass_pack_sint_i32_min() {
    let mut buf = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    assert_eq!(
        Marker::I32,
        write_sint(&mut &mut buf[..], -2147483648).ok().unwrap()
    );

    assert_eq!([0xd2, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], buf);
}

#[test]
fn pass_pack_sint_i32_max() {
    let mut buf = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    assert_eq!(
        Marker::U32,
        write_sint(&mut &mut buf[..], 2147483647).ok().unwrap()
    );

    assert_eq!([0xce, 0x7f, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00], buf);
}

#[test]
fn pass_pack_sint_i64_min() {
    let mut buf = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    assert_eq!(
        Marker::I64,
        write_sint(&mut &mut buf[..], -9223372036854775808)
            .ok()
            .unwrap()
    );

    assert_eq!([0xd3, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], buf);
}

#[test]
fn pass_pack_sint_i64_max() {
    let mut buf = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    assert_eq!(
        Marker::U64,
        write_sint(&mut &mut buf[..], 9223372036854775807)
            .ok()
            .unwrap()
    );

    assert_eq!([0xcf, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff], buf);
}
