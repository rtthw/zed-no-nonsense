
use std::{ops::Range, sync::Arc};

use gpui::SharedString;
use itertools::Itertools;
use url::Url;

use crate::repository::RepoPath;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PullRequest {
    pub number: u32,
    pub url: Url,
}

#[derive(Clone)]
pub struct GitRemote {
    pub owner: SharedString,
    pub repo: SharedString,
}

impl std::fmt::Debug for GitRemote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GitRemote")
            .field("owner", &self.owner)
            .field("repo", &self.repo)
            .finish()
    }
}

pub struct BuildCommitPermalinkParams<'a> {
    pub sha: &'a str,
}

pub struct BuildPermalinkParams<'a> {
    pub sha: &'a str,
    /// URL-escaped path using unescaped `/` as the directory separator.
    pub path: String,
    pub selection: Option<Range<u32>>,
}

impl<'a> BuildPermalinkParams<'a> {
    pub fn new(sha: &'a str, path: &RepoPath, selection: Option<Range<u32>>) -> Self {
        Self {
            sha,
            path: path.components().map(urlencoding::encode).join("/"),
            selection,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParsedGitRemote {
    pub owner: Arc<str>,
    pub repo: Arc<str>,
}
