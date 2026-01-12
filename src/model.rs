#![allow(clippy::needless_lifetimes)]

pub struct AssetToStreamingOiCap {
    coin: &'static str,
    cap: &'static str
}

pub struct AssetToFundingMultiplier {
    coin: &'static str,
    cap: &'static str
}


pub struct PerpDex {
    name: &'static str,
    full_name: &'static str,
    deployer: &'static str,
    oracle_updater: Option<&'static str>,
    fee_recipient: Option<&'static str>,
    asset_to_streaming_oi_cap: Vec<AssetToStreamingOiCap>,
    asset_to_funding_multiplier: Vec<AssetToFundingMultiplier>
}