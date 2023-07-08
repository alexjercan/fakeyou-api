#![warn(unused_crate_dependencies)]
#![forbid(unsafe_code)]
#![warn(clippy::all)]

//!<div align="center">
//! <!-- Build -->
//! <img src="https://img.shields.io/github/actions/workflow/status/alexjercan/fakeyou-api/rust.yml?style=flat-square"
//! alt="Github Worflow Status" />
//! <!-- Version -->
//! <a href="https://crates.io/crates/fakeyou-api">
//!   <img src="https://img.shields.io/crates/v/fakeyou-api?style=flat-square"
//!   alt="Crates.io version" />
//! </a>
//! <!-- Docs -->
//! <a href="https://docs.rs/fakeyou-api">
//!   <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
//!     alt="docs.rs docs" />
//! </a>
//! <!-- Downloads -->
//! <a href="https://crates.io/crates/fakeyou-api">
//!   <img src="https://img.shields.io/crates/d/fakeyou-api?style=flat-square"
//!     alt="Crates.io downloads" />
//! </a>
//! <!-- License -->
//! <a href="https://github.com/alexjercan/fakeyou-api/blob/master/LICENSE">
//!   <img src="https://img.shields.io/github/license/alexjercan/fakeyou-api?style=flat-square"
//!     alt="Crates.io downloads" />
//! </a>
//!</div>
//!
//! A very simple Rust library for FakeYou API.
//!
//! ## API
//!
//! Check the [official](https://docs.fakeyou.com/) API reference.
//!
//! |API|Support|
//! |---|---|
//! |Text to Speech|✔️|
//! |Voices|❌|
//! |Categories|❌|
//!
//! ## Usage
//!
//! Install the library using the Cargo.toml file or run the following command.
//!
//! ```console
//! cargo add fakeyou-api
//! ```
//!
//! Export your API key into the environment variables (if you use the paid version).
//!
//! ```console
//! export FAKEYOU_API_KEY=...
//! ```
//!
//! Then use the crate in your Rust code:
//!
//! ```no_run
//! # fn main() {
//! // Import the dependencies
//! use fakeyou_api::{tts::InferenceBody, util::tts::TtsApiSync, *};
//!
//! // You can create a default client without any api key.
//! // You can also load the API key from environment FAKEYOU_API_KEY.
//! // You can also hadcode through `Auth::new(<your_api_key>)`, but it is not recommended.
//! let auth = Auth::default();
//! let fakeyou = FakeYou::new(auth, FAKEYOU_API_URL);
//!
//! // Create the TTS body
//! let inference_body =
//!     InferenceBody::new("TM:ebgxj0j4fvzp", "Hello, World! What should we do today?");
//!
//! // Call the TTS API
//! // This uses the util module of this crate and will block the thread until the task is done
//! let output_result = fakeyou.create_tts_task(&inference_body).unwrap();
//!
//! // Do what you need with the audio file
//! std::fs::write("output.wav", output_result.bytes).unwrap();
//! # }
//! ```
//!
//! ## License
//! This project is licensed under MIT
//!

pub mod apis;
pub use apis::*;

pub mod fakeyou;
pub use fakeyou::*;

pub mod error;
pub use error::*;

pub mod util;
