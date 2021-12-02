pub fn parse_i32(input: &str) -> Result<i32, String> {
    input.to_string().parse().map_err(|e| format!("{}", e))
}

pub fn parse_u128(input: &str) -> Result<u128, String> {
    input.to_string().parse().map_err(|e| format!("{}", e))
}
