// Copyright (C) Parity Technologies (UK) Ltd.
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

//! Put implementations of functions from staging APIs here.

use primitives::{
	ValidatorIndex,
};
use sp_std::prelude::*;

/// Implementation for `DisabledValidators`
pub fn disabled_validators<T: pallet_session::Config>() -> Vec<ValidatorIndex> {
	// <pallet_session::Pallet<T>>::disabled_validators()
	// 	.iter()
	// 	.cloned()
	// 	.map(|v| ValidatorIndex(v))
	// 	.collect()
	// TODO
	Vec::new()
}
