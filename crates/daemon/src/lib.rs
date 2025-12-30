pub mod di;
pub mod domain;
pub mod infra;
pub mod server;
pub mod ui;
pub mod usecase;

mod generated {
    include!("generated/daemon/v1/daemon.v1.rs");
}

pub use server::Server;
