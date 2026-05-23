pub fn extract_repo_info(url: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = url.split('/').collect();

    // We need at least username and repo parts
    if parts.len() < 2 {
        return None;
    }

    // Get the last part (repo) and strip .git suffix
    let repo = parts.last()?.strip_suffix(".git")?;

    // The username should be second-to-last for standard GitHub URLs
    // Handle cases like "https://github.com/owner/repo.git"
    let username = parts.get(parts.len() - 2)?;

    Some((username, repo))
}
