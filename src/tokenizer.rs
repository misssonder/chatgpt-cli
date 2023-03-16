use tiktoken_rs::tiktoken::cl100k_base;

pub fn encode(input: &str) -> Vec<usize> {
    let bpe = cl100k_base().unwrap();
    bpe.encode_with_special_tokens(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let enc = encode("This is a test         with a lot of spaces");
        assert_eq!(enc.len(), 10)
    }
}
