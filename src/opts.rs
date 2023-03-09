use std::path::PathBuf;

use clap::{crate_authors, crate_version, ColorChoice, Parser, Subcommand};

/// `subwasm` allows fetching, parsing and calling some methods on WASM runtimes of Substrate based chains.
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

/// Get/Download the runtime wasm from a running node through rpc
#[derive(Parser)]
pub struct CheckOpts {
	#[clap(short, long, default_value = "schema.json")]
	pub schema: PathBuf,

	#[clap(short, long)]
	pub file: PathBuf,
}
