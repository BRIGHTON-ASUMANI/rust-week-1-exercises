// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    // Decode the hex string into bytes
    let str_bytes = hex::decode(raw_tx_hex).map_err(|e| format!("Hex decode error: {}", e))?;

    // Check if the transaction is at least 4 bytes long
    if str_bytes.len() < 4 {
        return Err("Transaction data too short to extract version".to_string());
    }


    Ok(version)
}
