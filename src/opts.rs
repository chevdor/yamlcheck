use std::path::PathBuf;

use clap::{crate_authors, crate_version, ColorChoice, Parser, Subcommand};

/// `yamlcheck` allows checking a yaml file based on a json schema
#[derive(Parser)]
#[clap(version = crate_version!(), author = crate_authors!(), color=ColorChoice::Always)]
pub struct Opts {
	#[clap(subcommand)]
	pub subcmd: SubCommand,
}

/// You can find all available commands below.
#[derive(Subcommand)]
pub enum SubCommand {
	#[clap(version = crate_version!(), author = crate_authors!())]
	Check(CheckOpts),
}

/// Check a file against a schema
#[derive(Parser)]
pub struct CheckOpts {
	#[clap(short, long, default_value = "schema.json")]
	pub schema: PathBuf,

	#[clap(short, long)]
	pub file: PathBuf,
}
