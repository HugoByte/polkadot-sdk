// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
	xcm::{CallOf, WeightInfo, XCM},
	AccountIdOf, Box, Config, RawOrigin,
};
use frame_support::{
	dispatch::PostDispatchInfo, pallet_prelude::DispatchResultWithPostInfo, weights::Weight,
};
use frame_system::pallet_prelude::BlockNumberFor;
use pallet_xcm::WeightInfo as XcmWeightInfo;
use sp_runtime::{DispatchError, DispatchErrorWithPostInfo, DispatchResult};
use xcm::{v3::MultiLocation, VersionedMultiLocation, VersionedXcm};
use xcm_executor::traits::{QueryHandler, QueryResponseStatus};

/// A pallet-xcm adapter for the XCM trait.
pub struct PalletXCMAdapter<T: pallet_xcm::Config>(sp_std::marker::PhantomData<T>);

impl<T> WeightInfo for PalletXCMAdapter<T>
where
	T: pallet_xcm::Config,
{
	fn execute() -> Weight {
		<T as pallet_xcm::Config>::WeightInfo::execute()
	}
	fn send() -> Weight {
		<T as pallet_xcm::Config>::WeightInfo::send()
	}
	fn query() -> Weight {
		<T as pallet_xcm::Config>::WeightInfo::new_query()
	}
	fn take_response() -> Weight {
		<T as pallet_xcm::Config>::WeightInfo::take_response()
	}
}

impl<T: Config> XCM<T> for PalletXCMAdapter<T>
where
	T: pallet_xcm::Config,
{
	type QueryId = <pallet_xcm::Pallet<T> as QueryHandler>::QueryId;
	type WeightInfo = Self;

	fn execute(
		origin: &AccountIdOf<T>,
		message: VersionedXcm<CallOf<T>>,
		max_weight: Weight,
	) -> DispatchResultWithPostInfo {
		// TODO since we are doing more than just calling pallet_xcm::Pallet::<T>::execute, we
		// should benchmark the filtering part
		use frame_support::traits::Contains;
		use xcm::prelude::{Transact, Xcm};

		let mut message: Xcm<CallOf<T>> =
			message.try_into().map_err(|_| pallet_xcm::Error::<T>::BadVersion)?;

		message
			.iter_mut()
			.try_for_each(|inst| -> Result<(), DispatchError> {
				let Transact { ref mut call, .. } = inst else { return Ok(()) };
				let call = call.ensure_decoded().map_err(|_| pallet_xcm::Error::<T>::BadVersion)?;

				if !<T as Config>::CallFilter::contains(call) {
					return Err(frame_system::Error::<T>::CallFiltered.into())
				}

				Ok(())
			})
			.map_err(|err| DispatchErrorWithPostInfo {
				post_info: PostDispatchInfo {
					actual_weight: Some(Weight::zero()),
					pays_fee: Default::default(),
				},
				error: err.into(),
			})?;

		let origin = RawOrigin::Signed(origin.clone()).into();
		pallet_xcm::Pallet::<T>::execute(origin, Box::new(VersionedXcm::from(message)), max_weight)
	}

	fn send(
		origin: &AccountIdOf<T>,
		dest: VersionedMultiLocation,
		msg: VersionedXcm<()>,
	) -> DispatchResult {
		let origin = RawOrigin::Signed(origin.clone()).into();
		pallet_xcm::Pallet::<T>::send(origin, Box::new(dest), Box::new(msg))
	}

	fn query(
		origin: &AccountIdOf<T>,
		timeout: BlockNumberFor<T>,
		match_querier: VersionedMultiLocation,
	) -> Result<Self::QueryId, DispatchError> {
		use frame_support::traits::EnsureOrigin;

		let origin = RawOrigin::Signed(origin.clone()).into();
		let responder = <T as pallet_xcm::Config>::ExecuteXcmOrigin::ensure_origin(origin)?;

		let query_id = <pallet_xcm::Pallet<T> as QueryHandler>::new_query(
			responder,
			timeout.into(),
			MultiLocation::try_from(match_querier)
				.map_err(|_| Into::<DispatchError>::into(pallet_xcm::Error::<T>::BadVersion))?,
		);

		Ok(query_id)
	}

	fn take_response(query_id: Self::QueryId) -> QueryResponseStatus<BlockNumberFor<T>> {
		<pallet_xcm::Pallet<T> as QueryHandler>::take_response(query_id)
	}
}
