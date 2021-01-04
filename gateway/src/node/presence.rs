// Copyright 2020 Nym Technologies SA
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::config::Config;
use validator_client::models::gateway::GatewayRegistrationInfo;
use validator_client::ValidatorClientError;

// there's no point in keeping the validator client persistently as it might be literally hours or days
// before it's used again
pub(crate) async fn register_with_validator(
    gateway_config: &Config,
    identity_key: String,
    sphinx_key: String,
) -> Result<(), ValidatorClientError> {
    let config = validator_client::Config::new(gateway_config.get_validator_rest_endpoint());
    let validator_client = validator_client::Client::new(config);

    let registration_info = GatewayRegistrationInfo::new(
        gateway_config.get_mix_announce_address(),
        gateway_config.get_clients_announce_address(),
        identity_key,
        sphinx_key,
        gateway_config.get_version().to_string(),
        gateway_config.get_location(),
        gateway_config.get_incentives_address(),
    );

    validator_client.register_gateway(registration_info).await
}

pub(crate) async fn unregister_with_validator(
    validator_endpoint: String,
    identity_key: String,
) -> Result<(), ValidatorClientError> {
    let config = validator_client::Config::new(validator_endpoint);
    let validator_client = validator_client::Client::new(config);

    validator_client.unregister_node(&identity_key).await
}
