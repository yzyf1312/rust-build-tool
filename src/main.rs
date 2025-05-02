use clap::{Arg, Command};
use std::error::Error;

mod build_system;
mod cargo_config;
mod dependency_checker;
mod platform_helper;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("rust_build_tool")
        .version("0.1.0")
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
            build_system.run()?;
        }
        Some(("depcheck", _)) => {
            dependency_checker::check_unused_dependencies()?;
        }
        _ => unreachable!(),
    }

    Ok(())
}
