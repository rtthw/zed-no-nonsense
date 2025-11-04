
use anyhow::{Context as _, Result};
use url::Url;
use util::maybe;

pub fn get_host_from_git_remote_url(remote_url: &str) -> Result<String> {
    maybe!({
        if let Some(remote_url) = remote_url.strip_prefix("git@")
            && let Some((host, _)) = remote_url.trim_start_matches("git@").split_once(':')
        {
            return Some(host.to_string());
        }

        Url::parse(remote_url)
            .ok()
            .and_then(|remote_url| remote_url.host_str().map(|host| host.to_string()))
    })
    .context("URL has no host")
}

#[cfg(test)]
mod tests {
    use super::get_host_from_git_remote_url;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_host_from_git_remote_url() {
        let tests = [
            (
                "https://jlannister@github.com/some-org/some-repo.git",
                Some("github.com".to_string()),
            ),
            (
                "git@github.com:zed-industries/zed.git",
                Some("github.com".to_string()),
            ),
            (
                "git@my.super.long.subdomain.com:zed-industries/zed.git",
                Some("my.super.long.subdomain.com".to_string()),
            ),
        ];

        for (remote_url, expected_host) in tests {
            let host = get_host_from_git_remote_url(remote_url).ok();
            assert_eq!(host, expected_host);
        }
    }
}
