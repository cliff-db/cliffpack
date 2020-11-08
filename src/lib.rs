#![feature(with_options)]

pub use creator::*;
pub use loader::*;

pub mod error;

mod creator;
mod loader;
mod reader;
mod writer;

#[cfg(test)]
mod tests;

pub(crate) const MAGIC_NUMBER: u32 = 0xC711FF;
pub(crate) const FILE_VERSION: u8 = 0;

/// This struct defines the plugin metadata
#[derive(Clone, Debug)]
pub struct PluginMeta {
    /// The plugin name
    pub name: String,
    /// The version string
    pub version: String,
    // The deno entrypoint
    pub entrypoint: String,
}
