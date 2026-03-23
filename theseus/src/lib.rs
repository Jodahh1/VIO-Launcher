/*!
# VIO Launcher

VIO Launcher is a powerful Minecraft launcher with advanced features including:
- Profile backup and management
- Screenshot manager
- Auto-mod updates
- Conflict detection
- Memory optimization
- Multi-account support
- Offline skins
- Console logger
*/
#![warn(unused_import_braces)]
#![deny(unused_must_use)]

#[macro_use]
mod util;

pub mod api;
mod config;
mod error;
mod event;
pub mod launcher;
mod logger;
pub mod state;

pub use api::*;
pub use error::*;
pub use event::{EventState, LoadingBar, LoadingBarType};
pub use logger::start_logger;
pub use state::InnerProjectPathUnix;
pub use state::State;
