// Copyright (C) Parity Technologies and the various Polkadot contributors, see Contributions.md
// for a list of specific contributors.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use clap::Parser;
use sc_chain_spec::ChainSpec;
use std::collections::HashMap;

mod common;
mod relay_chain_specs;
mod system_parachains_specs;

#[derive(Parser)]
struct Cli {
	/// The chain spec to generate.
	chain: String,

	/// Generate the chain spec as raw?
	#[arg(long)]
	raw: bool,
}

fn main() -> Result<(), String> {
	let cli = Cli::parse();

	let supported_chains =
		HashMap::<&str, Box<dyn Fn() -> Result<Box<dyn ChainSpec>, String>>>::from([
			#[cfg(feature = "polkadot")]
			("polkadot-dev", Box::new(relay_chain_specs::polkadot_development_config) as Box<_>),
			#[cfg(feature = "polkadot")]
			(
				"polkadot-local",
				Box::new(relay_chain_specs::polkadot_local_testnet_config) as Box<_>,
			),
			#[cfg(feature = "kusama")]
			("kusama-dev", Box::new(relay_chain_specs::kusama_development_config) as Box<_>),
			#[cfg(feature = "kusama")]
			("kusama-local", Box::new(relay_chain_specs::kusama_local_testnet_config) as Box<_>),
			#[cfg(feature = "asset-hub-kusama")]
			(
				"asset-hub-kusama-local",
				Box::new(system_parachains_specs::asset_hub_kusama_local_testnet_config) as Box<_>,
			),
			#[cfg(feature = "asset-hub-polkadot")]
			(
				"asset-hub-polkadot-local",
				Box::new(system_parachains_specs::asset_hub_polkadot_local_testnet_config)
					as Box<_>,
			),
			#[cfg(feature = "collectives-polkadot")]
			(
				"collectives-polkadot-local",
				Box::new(system_parachains_specs::collectives_polkadot_local_testnet_config)
					as Box<_>,
			),
			#[cfg(feature = "bridge-hub-polkadot")]
			(
				"bridge-hub-polkadot-local",
				Box::new(system_parachains_specs::bridge_hub_polkadot_local_testnet_config)
					as Box<_>,
			),
			#[cfg(feature = "bridge-hub-kusama")]
			(
				"bridge-hub-kusama-local",
				Box::new(system_parachains_specs::bridge_hub_kusama_local_testnet_config) as Box<_>,
			),
			#[cfg(feature = "glutton-kusama")]
			(
				"glutton-kusama-local",
				Box::new(system_parachains_specs::glutton_kusama_local_testnet_config) as Box<_>,
			),
			#[cfg(feature = "encointer-kusama")]
			(
				"encointer-kusama-local",
				Box::new(system_parachains_specs::encointer_kusama_local_testnet_config) as Box<_>,
			),
			#[cfg(feature = "coretime-kusama")]
			(
				"coretime-kusama",
				Box::new(system_parachains_specs::coretime_kusama_config) as Box<_>,
			),
			#[cfg(feature = "coretime-kusama")]
			(
				"coretime-kusama-local",
				Box::new(system_parachains_specs::coretime_kusama_local_testnet_config) as Box<_>,
			),
			#[cfg(feature = "coretime-polkadot")]
			(
				"coretime-polkadot",
				Box::new(system_parachains_specs::coretime_polkadot_config) as Box<_>,
			),
			#[cfg(feature = "coretime-polkadot")]
			(
				"coretime-polkadot-local",
				Box::new(system_parachains_specs::coretime_polkadot_local_testnet_config) as Box<_>,
			),
			#[cfg(feature = "people-kusama")]
			(
				"people-kusama-local",
				Box::new(system_parachains_specs::people_kusama_local_testnet_config) as Box<_>,
			),
			#[cfg(feature = "people-polkadot")]
			(
				"people-polkadot-local",
				Box::new(system_parachains_specs::people_polkadot_local_testnet_config) as Box<_>,
			),
		]);

	if let Some(function) = supported_chains.get(&*cli.chain) {
		let chain_spec = (*function)()?.as_json(cli.raw)?;
		print!("{chain_spec}");
		Ok(())
	} else {
		let supported = supported_chains.keys().enumerate().fold(String::new(), |c, (n, k)| {
			let extra = if n + 1 < supported_chains.len() { ", " } else { "" };
			format!("{c}{k}{extra}")
		});
		if cli.chain.ends_with(".json") {
			let chain_spec = common::from_json_file(&cli.chain, supported)?.as_json(cli.raw)?;
			print!("{chain_spec}");
			Ok(())
		} else {
			Err(format!(
				"Unknown chain: '{}', only supported: '{supported}' or a json file",
				cli.chain,
			))
		}
	}
}
