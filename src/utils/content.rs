pub fn config_example() -> String {
    return format!(
        "{{
    // BUILD INSTRUCTIONS
    // Array of shell commands to compile/package application components
    // Recommendation: Use one entry per microservice/components (frontend/backend/CDN)
    // Each command should be idempotent and environment-agnostic
    \"build\": [],

    // DEPLOYMENT INSTRUCTIONS
    // Array of shell commands to deploy artifacts to target environments
    // Best practices:
    // 1. Use absolute paths for production reliability
    // 2. Consider atomic deployments (versioned directories + symlink rotation)
    // 3. Validate filesystem permissions post-deployment
    \"mouve\": [],

    // VERSION CONTROL CONFIGURATION
    // Git repository URL for change tracking and version synchronization
    // Supports SSH/HTTPS protocols (ensure proper deploy key configuration)
    \"repo\": \"\",

    // CHANGE DETECTION INTERVAL
    // Polling frequency (in seconds) for repository change checks
    // Security/reliability tradeoff: 
    // - Lower values increase responsiveness
    // - Higher values reduce API rate limit risks
    \"interval_in_sec\": \"60\"
    }} 
    // DO NOT FORGET REMOVE COMMENTS
    "
    );
}
