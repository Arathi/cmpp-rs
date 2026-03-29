pub fn format<S>(input: &[u8], sep: S, uppercase: bool) -> String
where
    S: Into<String>,
{
    let mut bytes: Vec<String> = vec![];
    for byte in input {
        bytes.push(format!("{:02x}", byte));
    }
    let mut output = bytes.join(sep.into().as_str());
    if uppercase {
        output = output.to_uppercase();
    }
    output
}
