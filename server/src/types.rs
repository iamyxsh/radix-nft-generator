use scrypto::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(ScryptoSbor, Clone, NonFungibleData, ManifestSbor, Debug, Deserialize, Serialize)]
pub struct NFTData {
    pub name: String,
    pub description: String,
    pub image_uri: String,
}
