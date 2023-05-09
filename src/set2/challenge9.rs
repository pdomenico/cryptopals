pub fn pkcs7(bytes: &[u8], size: usize) -> Vec<u8> {
    let mut padded = Vec::from(bytes);
    let padding = size - (padded.len() % size);
    padded.extend(vec![padding as u8; padding]);
    padded
}
