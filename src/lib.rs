// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    // Try to decode the hex string into bytes
    let str_bytes = hex::decode(raw_tx_hex).map_err(|e| format!("Hex decode error: {}", e))?;


    Ok(version)
}
