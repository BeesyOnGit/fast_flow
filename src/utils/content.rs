pub fn config_example() -> String {
    return format!(
        "{{
    // BUILD CONFIGURATION
    // Array of shell commands to compile the application
    // Executed in sequence from the repository root
    \"build\": [
        \"cargo build --release\"  // example Rust release build or npm run build
    ],

    // DEPLOYMENT MAPPING
    // Array of file operations to deploy build artifacts
    // Each entry specifies:
    // - \"from\": Source path (relative to repo root)
    // - \"to\": Absolute destination path on target system
    \"mouve\": [
        {{
            \"from\": \"target/release/myapp\",  // Built binary
            \"to\": \"/var/www/api.myapp/\"  // Production Directory location
    }}
    ],

    // REPOSITORY CONFIGURATION
    // Git repository URL for version control integration
    // Supports both HTTPS and SSH formats
    \"repo\": \"https://github.com/MyUser/myapp.git\"
    "
    );
}
