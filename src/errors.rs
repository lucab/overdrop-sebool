use clap;
use selinur;
use std::io;
use toml;

error_chain!{
    foreign_links {
        Io(io::Error);
        Cli(clap::Error);
        SELinux(selinur::errors::Error);
        Toml(toml::de::Error);
    }
}
