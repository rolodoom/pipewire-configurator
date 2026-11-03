use std::process::Command;

// Read all settings metadata from PipeWire.
fn read_metadata() -> Result<String, String> {
    let output = Command::new("pw-metadata")
    .args(["-n", "settings"])
    .output()
    .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

// Extract the value associated with a metadata key.
fn extract_value(metadata: &str, key: &str) -> Option<String> {
    for line in metadata.lines() {
        if line.contains(&format!("key:'{}'", key)) {
            if let Some(value) = line.split("value:'").nth(1) {
                return Some(value.split('\'').next()?.to_string());
            }
        }
    }

    None
}

// Read the current quantum and sample rate.
pub fn get_current_settings() -> Result<(String, String), String> {
    let metadata = read_metadata()?;

    let quantum = extract_value(&metadata, "clock.force-quantum")
    .filter(|value| value != "0")
    .or_else(|| extract_value(&metadata, "clock.quantum"))
    .ok_or("Could not read current quantum")?;

    let rate = extract_value(&metadata, "clock.force-rate")
    .filter(|value| value != "0")
    .or_else(|| extract_value(&metadata, "clock.rate"))
    .ok_or("Could not read current sample rate")?;

    Ok((quantum, rate))
}

// Apply the selected settings.
fn set_metadata(key: &str, value: &str) -> Result<(), String> {
    let status = Command::new("pw-metadata")
    .args(["-n", "settings", "0", key, value])
    .status()
    .map_err(|e| e.to_string())?;

    if !status.success() {
        return Err(format!("pw-metadata exited with status {status}"));
    }

    Ok(())
}

pub fn apply_settings(quantum: &str, rate: &str) -> Result<(), String> {
    set_metadata("clock.force-quantum", quantum)?;
    set_metadata("clock.force-rate", rate)?;

    Ok(())
}
