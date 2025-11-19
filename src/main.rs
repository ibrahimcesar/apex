//! xcargo CLI entry point

use anyhow::Result;
use clap::{Parser, Subcommand};
use xcargo::build::{Builder, BuildOptions};
use xcargo::config::Config;
use xcargo::output::{helpers, tips};
use xcargo::target::Target;
use xcargo::toolchain::ToolchainManager;

/// xcargo - Cross-compilation, zero friction 🎯
#[derive(Parser)]
#[command(name = "xcargo")]
#[command(author, version, about, long_about = None)]
#[command(after_help = "TIP: Run 'xcargo build --target <triple>' to cross-compile")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Build for target platform(s)
    Build {
        /// Target triple (e.g., x86_64-pc-windows-gnu)
        #[arg(short, long)]
        target: Option<String>,

        /// Build for all configured targets
        #[arg(long, conflicts_with = "target")]
        all: bool,

        /// Build in release mode
        #[arg(short, long)]
        release: bool,

        /// Toolchain to use (e.g., stable, nightly)
        #[arg(long)]
        toolchain: Option<String>,

        /// Additional cargo arguments
        #[arg(last = true)]
        cargo_args: Vec<String>,
    },

    /// Manage targets
    Target {
        #[command(subcommand)]
        action: TargetAction,
    },

    /// Display configuration
    Config {
        /// Show default config
        #[arg(long)]
        default: bool,
    },

    /// Show version information
    Version,
}

#[derive(Subcommand)]
enum TargetAction {
    /// Add a target
    Add {
        /// Target name or triple
        target: String,

        /// Toolchain to add target to
        #[arg(long, default_value = "stable")]
        toolchain: String,
    },

    /// List targets
    List {
        /// Show only installed targets
        #[arg(long)]
        installed: bool,

        /// Toolchain to list targets for
        #[arg(long)]
        toolchain: Option<String>,
    },

    /// Show target information
    Info {
        /// Target triple
        target: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build {
            target,
            all,
            release,
            toolchain,
            cargo_args,
        } => {
            let builder = Builder::new()?;

            let options = BuildOptions {
                target: target.clone(),
                release,
                cargo_args,
                toolchain,
                verbose: cli.verbose,
            };

            if all {
                // Build for all configured targets
                let config = Config::discover()?.map(|(c, _)| c).unwrap_or_default();

                if config.targets.default.is_empty() {
                    helpers::error("No default targets configured");
                    helpers::hint("Add targets to xcargo.toml: [targets] default = [\"x86_64-unknown-linux-gnu\"]");
                    helpers::tip(tips::CONFIG_FILE);
                    std::process::exit(1);
                }

                builder.build_all(&config.targets.default, &options)?;
            } else {
                builder.build(&options)?;
            }
        }

        Commands::Target { action } => match action {
            TargetAction::Add { target, toolchain } => {
                helpers::section("Add Target");

                let manager = ToolchainManager::new()?;
                let target_triple = Target::resolve_alias(&target)?;

                helpers::progress(format!(
                    "Adding target {} to toolchain {}...",
                    target_triple, toolchain
                ));

                manager.install_target(&toolchain, &target_triple)?;

                helpers::success(format!("Target {} added successfully", target_triple));
                helpers::tip(format!(
                    "Use 'xcargo build --target {}' to build for this target",
                    target_triple
                ));
            }

            TargetAction::List { installed, toolchain } => {
                helpers::section("Available Targets");

                if installed {
                    let manager = ToolchainManager::new()?;
                    let tc = toolchain.unwrap_or_else(|| "stable".to_string());

                    helpers::info(format!("Installed targets for toolchain '{}':", tc));
                    println!();

                    match manager.list_targets(&tc) {
                        Ok(targets) => {
                            if targets.is_empty() {
                                println!("  No targets installed");
                            } else {
                                for target in targets {
                                    println!("  • {}", target);
                                }
                            }
                        }
                        Err(e) => {
                            helpers::error(format!("Failed to list targets: {}", e));
                            std::process::exit(1);
                        }
                    }
                } else {
                    println!("Common cross-compilation targets:\n");

                    println!("Linux:");
                    println!("  • x86_64-unknown-linux-gnu   (Linux x86_64)");
                    println!("  • x86_64-unknown-linux-musl  (Linux x86_64, statically linked)");
                    println!("  • aarch64-unknown-linux-gnu  (Linux ARM64)");
                    println!();

                    println!("Windows:");
                    println!("  • x86_64-pc-windows-gnu      (Windows x86_64, MinGW)");
                    println!("  • x86_64-pc-windows-msvc     (Windows x86_64, MSVC)");
                    println!();

                    println!("macOS:");
                    println!("  • x86_64-apple-darwin        (macOS x86_64)");
                    println!("  • aarch64-apple-darwin       (macOS ARM64, M1/M2)");
                    println!();

                    helpers::hint("Use 'xcargo target list --installed' to see installed targets");
                    helpers::tip("Use 'xcargo target add <triple>' to install a new target");
                }
            }

            TargetAction::Info { target } => {
                helpers::section("Target Information");

                let target_triple = Target::resolve_alias(&target)?;
                match Target::from_triple(&target_triple) {
                    Ok(target) => {
                        println!("Triple:       {}", target.triple);
                        println!("Architecture: {}", target.arch);
                        println!("OS:           {}", target.os);
                        println!("Environment:  {}", target.env.as_deref().unwrap_or("default"));
                        println!("Tier:         {:?}", target.tier);
                        println!();

                        let requirements = target.get_requirements();
                        if requirements.linker.is_some() || !requirements.tools.is_empty() || !requirements.system_libs.is_empty() {
                            helpers::info("Requirements:");
                            if let Some(linker) = requirements.linker {
                                println!("  Linker: {}", linker);
                            }
                            if !requirements.tools.is_empty() {
                                println!("  Tools: {}", requirements.tools.join(", "));
                            }
                            if !requirements.system_libs.is_empty() {
                                println!("  System libs: {}", requirements.system_libs.join(", "));
                            }
                            println!();
                        }

                        let host = Target::detect_host()?;
                        if target.can_cross_compile_from(&host) {
                            helpers::success("Can cross-compile from this host");
                        } else {
                            helpers::warning("May require container for cross-compilation");
                        }

                        println!();
                        helpers::tip(format!("Add this target: xcargo target add {}", target.triple));
                        helpers::tip(format!("Build for this target: xcargo build --target {}", target.triple));
                    }
                    Err(e) => {
                        helpers::error(format!("Invalid target: {}", e));
                        std::process::exit(1);
                    }
                }
            }
        },

        Commands::Config { default } => {
            helpers::section("Configuration");

            if default {
                let config = Config::default();
                match config.to_toml() {
                    Ok(toml) => {
                        println!("{}", toml);
                        println!();
                        helpers::tip("Save this to xcargo.toml to customize your build");
                    }
                    Err(e) => {
                        helpers::error(format!("Failed to generate config: {}", e));
                        std::process::exit(1);
                    }
                }
            } else {
                match Config::discover() {
                    Ok(Some((config, path))) => {
                        helpers::info(format!("Configuration from: {}", path.display()));
                        println!();
                        match config.to_toml() {
                            Ok(toml) => println!("{}", toml),
                            Err(e) => {
                                helpers::error(format!("Failed to serialize config: {}", e));
                                std::process::exit(1);
                            }
                        }
                    }
                    Ok(None) => {
                        helpers::info("No xcargo.toml found, using defaults");
                        println!();
                        let config = Config::default();
                        match config.to_toml() {
                            Ok(toml) => println!("{}", toml),
                            Err(e) => {
                                helpers::error(format!("Failed to generate config: {}", e));
                                std::process::exit(1);
                            }
                        }
                        println!();
                        helpers::tip(tips::CONFIG_FILE);
                    }
                    Err(e) => {
                        helpers::error(format!("Failed to load config: {}", e));
                        std::process::exit(1);
                    }
                }
            }
        }

        Commands::Version => {
            println!("xcargo {}", env!("CARGO_PKG_VERSION"));
            println!("Cross-compilation, zero friction 🎯");
            println!();
            println!("https://github.com/ibrahimcesar/xcargo");
        }
    }

    Ok(())
}
