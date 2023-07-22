//! This module contains tokyo-draft hooks

use thiserror::Error;

mod render_hook;

pub use render_hook::RenderHook;

type HookResult<T> = Result<T, HookError>;

#[derive(Error, Debug)]
pub enum HookError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
}
