// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    // Decode the hex string into bytes
    let str_bytes = hex::decode(raw_tx_hex).map_err(|e| format!("Hex decode error: {}", e))?;

    // Check if the transaction is at least 4 bytes long
    if str_bytes.len() < 4 {
        return Err("Transaction data too short to extract version".to_string());
    }

    // Extract first 4 bytes as the version
    let version_bytes: [u8; 4] = str_bytes[0..4].try_into().unwrap();
    let version = u32::from_le_bytes(version_bytes);

    Ok(version)
}
