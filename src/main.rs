extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate selinur;
extern crate toml;
#[macro_use]
extern crate serde_derive;

mod cli;
mod errors;
mod od_cfg;

fn main() {
    env_logger::Builder::from_default_env()
        .default_format_module_path(false)
        .default_format_timestamp(false)
        .init();

    let r = cli::run();
    if let Err(e) = r {
        eprintln!("{}", e);
        ::std::process::exit(1);
    }
}
