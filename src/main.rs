mod opts;

use clap::Parser;
use env_logger::Env;
use opts::*;
use serde_json::Value;
use std::fs::File;
use valico::json_schema;

/// Main entry point of the `subwasm` cli
fn main() {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	let opts: Opts = Opts::parse();

	match opts.subcmd {
		SubCommand::Check(check_opts) => {
			let schema_file = check_opts.schema;
			let yaml_file = check_opts.file;
			let json_schema: Value = serde_json::from_reader(File::open(schema_file).unwrap()).unwrap();
			let yaml: Value = serde_yaml::from_reader(File::open(yaml_file).unwrap()).unwrap();

			let mut scope = json_schema::Scope::new();
			let schema = scope.compile_and_return(json_schema, false).unwrap();

			let validation = schema.validate(&yaml);
			let validation_result = validation.is_valid();
			let validation_sresult = validation.is_strictly_valid();

			println!("{title:<15}: {validation_result}", title = "valid");
			println!("{title:<15}: {validation_sresult}\n", title = "strictly valid");

			if !(validation_result && validation_sresult) {
				println!("errors: {:#?}", validation.errors);
				println!("missing: {:#?}", validation.missing);
				std::process::exit(exitcode::DATAERR);
			}
		}
	};

	std::process::exit(exitcode::OK);
}
