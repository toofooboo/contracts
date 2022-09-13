use std::collections::HashMap;

use cosmwasm_std::{to_binary, Binary, ContractResult, QuerierResult, SystemError};
use osmosis_std::types::osmosis::gamm::twap::v1beta1::{
    GetArithmeticTwapRequest, GetArithmeticTwapResponse,
};
use osmosis_std::types::osmosis::gamm::v1beta1::{
    QueryPoolRequest, QueryPoolResponse, QuerySpotPriceRequest, QuerySpotPriceResponse,
    QuerySwapExactAmountInRequest, QuerySwapExactAmountInResponse, SwapAmountInRoute,
};
use prost::{DecodeError, Message};

// NOTE: We can't use osmo_bindings::Swap (as key) for HashMap because it doesn't implement Hash
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct PriceKey {
    pub pool_id: u64,
    pub denom_in: String,
    pub denom_out: String,
}

#[derive(Clone, Default)]
pub struct OsmosisQuerier {
    pub pools: HashMap<u64, QueryPoolResponse>,

    pub spot_prices: HashMap<PriceKey, QuerySpotPriceResponse>,
    pub twap_prices: HashMap<PriceKey, GetArithmeticTwapResponse>,

    /// key comes from `prepare_estimate_swap_key` function
    pub estimate_swaps: HashMap<String, QuerySwapExactAmountInResponse>,
}

impl OsmosisQuerier {
    pub fn handle_stargate_query(&self, path: &str, data: &Binary) -> Result<QuerierResult, ()> {
        if path == "/osmosis.gamm.v1beta1.Query/Pool" {
            let parse_osmosis_query: Result<QueryPoolRequest, DecodeError> =
                Message::decode(data.as_slice());
            if let Ok(osmosis_query) = parse_osmosis_query {
                return Ok(self.handle_query_pool_request(osmosis_query));
            }
        }

        if path == "/osmosis.gamm.v1beta1.Query/SpotPrice" {
            let parse_osmosis_query: Result<QuerySpotPriceRequest, DecodeError> =
                Message::decode(data.as_slice());
            if let Ok(osmosis_query) = parse_osmosis_query {
                return Ok(self.handle_query_spot_request(osmosis_query));
            }
        }

        if path == "/osmosis.gamm.twap.v1beta1.Query/GetArithmeticTwap" {
            let parse_osmosis_query: Result<GetArithmeticTwapRequest, DecodeError> =
                Message::decode(data.as_slice());
            if let Ok(osmosis_query) = parse_osmosis_query {
                return Ok(self.handle_query_twap_request(osmosis_query));
            }
        }

        if path == "/osmosis.gamm.v1beta1.Query/EstimateSwapExactAmountIn" {
            let parse_osmosis_query: Result<QuerySwapExactAmountInRequest, DecodeError> =
                Message::decode(data.as_slice());
            if let Ok(osmosis_query) = parse_osmosis_query {
                return Ok(self.handle_query_estimate_request(osmosis_query));
            }
        }

        Err(())
    }

    fn handle_query_pool_request(&self, request: QueryPoolRequest) -> QuerierResult {
        let pool_id = request.pool_id;
        let res: ContractResult<Binary> = match self.pools.get(&pool_id) {
            Some(query_response) => to_binary(&query_response).into(),
            None => Err(SystemError::InvalidRequest {
                error: format!("QueryPoolResponse is not found for pool id: {}", pool_id),
                request: Default::default(),
            })
            .into(),
        };
        Ok(res).into()
    }

    fn handle_query_spot_request(&self, request: QuerySpotPriceRequest) -> QuerierResult {
        let price_key = PriceKey {
            pool_id: request.pool_id,
            denom_in: request.base_asset_denom,
            denom_out: request.quote_asset_denom,
        };
        let res: ContractResult<Binary> = match self.spot_prices.get(&price_key) {
            Some(query_response) => to_binary(&query_response).into(),
            None => Err(SystemError::InvalidRequest {
                error: format!(
                    "QuerySpotPriceResponse is not found for price key: {:?}",
                    price_key
                ),
                request: Default::default(),
            })
            .into(),
        };
        Ok(res).into()
    }

    fn handle_query_twap_request(&self, request: GetArithmeticTwapRequest) -> QuerierResult {
        let price_key = PriceKey {
            pool_id: request.pool_id,
            denom_in: request.base_asset,
            denom_out: request.quote_asset,
        };
        let res: ContractResult<Binary> = match self.twap_prices.get(&price_key) {
            Some(query_response) => to_binary(&query_response).into(),
            None => Err(SystemError::InvalidRequest {
                error: format!(
                    "GetArithmeticTwapResponse is not found for price key: {:?}",
                    price_key
                ),
                request: Default::default(),
            })
            .into(),
        };
        Ok(res).into()
    }

    fn handle_query_estimate_request(
        &self,
        request: QuerySwapExactAmountInRequest,
    ) -> QuerierResult {
        let routes_key = Self::prepare_estimate_swap_key(&request.routes);
        let res: ContractResult<Binary> = match self.estimate_swaps.get(&routes_key) {
            Some(query_response) => to_binary(&query_response).into(),
            None => Err(SystemError::InvalidRequest {
                error: format!(
                    "QuerySwapExactAmountInRequest is not found for routes: {:?}",
                    routes_key
                ),
                request: Default::default(),
            })
            .into(),
        };
        Ok(res).into()
    }

    pub fn prepare_estimate_swap_key(route: &[SwapAmountInRoute]) -> String {
        let routes: Vec<_> =
            route.iter().map(|step| format!("{}.{}", step.pool_id, step.token_out_denom)).collect();
        routes.join(",")
    }
}
