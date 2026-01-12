//! Library module for the swifi speed test tool.
//!
//! This crate provides CLI tools and libraries for testing `WiFi` download and upload speeds.

mod cli;
mod server;
mod speed_test;

pub use {
    cli::{AppConfig, AppConfigBuilder, CliArgs},
    server::{Server, ServerList},
    speed_test::{Direction, SpeedMeasurement, SpeedTest, SpeedTestResult},
};