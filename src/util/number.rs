pub fn parse_i32(input: &String) -> Result<i32, String> {
    input.to_string().parse().map_err(|e| format!("{}", e))
}