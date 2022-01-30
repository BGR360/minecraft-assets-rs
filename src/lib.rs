//! Parsing Minecraft asset files and resource packs in Rust.
//!
//! This crate provides types that can be used with [`serde_json`] to parse the
//! data files in either the Minecraft `assets/` directory or in a resource pack.

#![warn(missing_docs)]

pub mod schemas;
pub mod versions;
