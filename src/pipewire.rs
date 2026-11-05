use std::process::Command;

// Read all metadata entries from the PipeWire settings metadata object.
fn read_metadata() -> Result<String, String> {
    let output = Command::new("pw-metadata")
        .args(["-n", "settings"])
        .output()
        .map_err(|e| e.to_string())?;

    // Return the command's error output if it exits unsuccessfully.
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    // Convert the command output into a string.
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

// Extract the value associated with a specific metadata key.
fn extract_value(metadata: &str, key: &str) -> Option<String> {
    for line in metadata.lines() {
        // Find the metadata entry matching the requested key.
        if line.contains(&format!("key:'{}'", key)) {
            // Extract the value field from the matching entry.
            if let Some(value) = line.split("value:'").nth(1) {
                return Some(value.split('\'').next()?.to_string());
            }
        }
    }

    // Return None if the requested key was not found.
    None
}

// Read the current quantum and sample rate from PipeWire metadata.
pub fn get_current_settings() -> Result<(String, String), String> {
    let metadata = read_metadata()?;

    // Prefer the forced quantum value when it is set.
    // Otherwise, use the current quantum value.
    let quantum = extract_value(&metadata, "clock.force-quantum")
        .filter(|value| value != "0")
        .or_else(|| extract_value(&metadata, "clock.quantum"))
        .ok_or("Could not read current quantum")?;

    // Prefer the forced sample rate when it is set.
    // Otherwise, use the current sample rate.
    let rate = extract_value(&metadata, "clock.force-rate")
        .filter(|value| value != "0")
        .or_else(|| extract_value(&metadata, "clock.rate"))
        .ok_or("Could not read current sample rate")?;

    Ok((quantum, rate))
}

// Set a PipeWire metadata key to the specified value.
fn set_metadata(key: &str, value: &str) -> Result<(), String> {
    let status = Command::new("pw-metadata")
        .args(["-n", "settings", "0", key, value])
        .status()
        .map_err(|e| e.to_string())?;

    // Check whether the metadata command completed successfully.
    if !status.success() {
        return Err(format!("pw-metadata exited with status {status}"));
    }

    Ok(())
}

// Apply the selected quantum and sample rate to PipeWire.
pub fn apply_settings(quantum: &str, rate: &str) -> Result<(), String> {
    // Set the forced quantum value.
    set_metadata("clock.force-quantum", quantum)?;

    // Set the forced sample rate.
    set_metadata("clock.force-rate", rate)?;

    Ok(())
}
