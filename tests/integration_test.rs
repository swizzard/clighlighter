use std::io;
#[test]
fn test_ts() -> io::Result<()> {
    let input = include_str!("./input.ts");
    let expected_output = include_str!("./output-ts.html");
    let mut out: Box<io::Cursor<Vec<u8>>> = Box::new(io::Cursor::new(Vec::new()));
    clighlighter::highlight(&clighlighter::highlight::ts::TS, &input, &mut out)?;
    assert_eq!(out.get_ref(), expected_output.as_bytes());
    Ok(())
}
