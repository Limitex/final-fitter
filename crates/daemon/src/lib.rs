pub mod di;
pub mod domain;
pub mod infra;
pub mod server;
pub mod ui;
pub mod usecase;

mod generated {
    include!("generated/ping/v1/ping.v1.rs");
}

pub use server::Server;
