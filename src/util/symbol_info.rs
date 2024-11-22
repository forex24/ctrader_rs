use std::collections::HashMap;

use crate::{
    protos::spotware_message::{ProtoOaAsset, ProtoOaSymbol},
    Error, Session,
};

#[derive(Debug, Clone)]
pub struct SpotwareSymbolInfo {
    pub name: String,
    pub asset_class: String,
    pub info: ProtoOaSymbol,
    pub base_decimals: u8,
    pub quote_decimals: u8,
}

pub async fn get_symbol_infos(client: &Session) -> Result<Vec<SpotwareSymbolInfo>, Error> {
    let mut symbol_infos: Vec<SpotwareSymbolInfo> = Vec::new();

    let asset_class_list = client.asset_class_list().await?.asset_class;

    let mut asset_class_id_to_asset_class_name: HashMap<i64, String> = HashMap::new();

    for asset_class in asset_class_list {
        let id = asset_class.id();
        let name = asset_class.name();
        asset_class_id_to_asset_class_name.insert(id, name.to_string());
    }

    let mut asset_id_to_asset_info: HashMap<i64, ProtoOaAsset> = HashMap::new();

    let asset_list = client.asset_list().await?.asset;

    for asset in asset_list {
        let id = asset.asset_id;
        asset_id_to_asset_info.insert(id, asset);
    }

    let symbol_category_list = client.symbol_category_list().await?.symbol_category;

    let mut category_id_to_assert_class_id: HashMap<i64, i64> = HashMap::new();

    for category in &symbol_category_list {
        category_id_to_assert_class_id.insert(category.id, category.asset_class_id);
    }

    let symbol_list = client.symbol_list().await?.symbol;

    let enabled_symbol_list = symbol_list
        .iter()
        .filter(|s| s.enabled())
        .collect::<Vec<_>>();

    let symbol_ids = enabled_symbol_list
        .iter()
        .map(|s| s.symbol_id)
        .collect::<Vec<_>>();

    let symbol_full_infos = client.symbol_by_id(symbol_ids).await?.symbol;

    let mut full_info_by_id: HashMap<i64, ProtoOaSymbol> = HashMap::new();

    for info in symbol_full_infos {
        full_info_by_id.insert(info.symbol_id, info);
    }

    for symbol in enabled_symbol_list {
        let id = symbol.symbol_id;
        let category_id = symbol.symbol_category_id();
        let asset_class_id = category_id_to_assert_class_id[&category_id];
        let asset_class_name = asset_class_id_to_asset_class_name[&asset_class_id].clone();
        let full_info = full_info_by_id[&id].clone();
        let base_decimals = asset_id_to_asset_info[&symbol.base_asset_id()].digits();
        let quote_decimals = asset_id_to_asset_info[&symbol.quote_asset_id()].digits();

        let symbol_info = SpotwareSymbolInfo {
            name: symbol.symbol_name().to_string(),
            asset_class: asset_class_name,
            info: full_info,
            base_decimals: base_decimals as u8,
            quote_decimals: quote_decimals as u8,
        };
        symbol_infos.push(symbol_info);
    }

    symbol_infos.sort_by(|a, b| a.info.symbol_id.cmp(&b.info.symbol_id));

    Ok(symbol_infos)
}
