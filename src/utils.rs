pub fn read_input(filename: &str) -> std::io::Result<String> {
    use std::fs::File;
    use std::io::Read;

    let mut f = File::open(filename)?;
    let mut res = String::new();
    f.read_to_string(&mut res)?;
    Ok(res)
}
