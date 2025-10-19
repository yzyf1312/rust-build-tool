use clap::{Arg, Command};
use std::env;
use std::error::Error;

mod build_system;
mod cargo_config;
mod dependency_checker;
mod platform_helper;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("rust_build_tool")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Build optimized Rust executables")
        .subcommand_required(true)
        .subcommand(
            Command::new("build")
                .about("Build the project")
                .arg(
                    Arg::new("target")
                        .long("target")
                        .help("Target platform (default: auto-detect)"),
                )
                .arg(
                    Arg::new("upx")
                        .long("upx")
                        .help("Enable UPX compression")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("clean")
                        .long("clean")
                        .help("Clean before building")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("clippy")
                        .long("clippy")
                        .help("Run clippy lint checks after build")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("deny")
                        .long("deny")
                        .help("Run cargo-deny checks after build")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("full-check")
                        .long("full-check")
                        .help("Run full workflow: clippy -> depcheck -> deny -> build (stops immediately on any failure)")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(Command::new("depcheck").about("Check and remove unused dependencies"))
        .get_matches();

    match matches.subcommand() {
        Some(("build", sub_matches)) => {
            let target = match sub_matches.get_one::<String>("target") {
                Some(t) => t.to_string(),
                None => platform_helper::get_default_target()?,
            };
            let use_upx = sub_matches.get_flag("upx");
            let clean = sub_matches.get_flag("clean");

            let build_system = build_system::BuildSystem::new(&target, use_upx, clean)?;
            if sub_matches.get_flag("full-check") {
                // Complete workflow: clippy -> depcheck -> deny -> build
                build_system.run_clippy()?;
                dependency_checker::check_unused_dependencies()?;
                build_system.run_cargo_deny()?;
                build_system.run()?;
            } else {
                if sub_matches.get_flag("clippy") {
                    build_system.run_clippy()?;
                }

                if sub_matches.get_flag("deny") {
                    build_system.run_cargo_deny()?;
                }
                build_system.run()?;
            }
        }
        Some(("depcheck", _)) => {
            dependency_checker::check_unused_dependencies()?;
        }
        _ => unreachable!(),
    }

    Ok(())
}
