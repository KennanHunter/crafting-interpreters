pub fn is_valid_literal_character(ch: &char) -> bool {
    ch.is_ascii_alphanumeric() || *ch == '_' || *ch == '-'
}
