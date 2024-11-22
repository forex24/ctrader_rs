use super::Session;
use crate::{protos::spotware_message::*, Error};

impl Session {
    //+------------------------------------------------------------------+
    //|                        Asset & Symbol                            |
    //+------------------------------------------------------------------+

    // Request for a list of symbols available for a trading account.
    // Symbol entries are returned with the limited set of fields.
    pub async fn symbol_list(&self) -> Result<ProtoOaSymbolsListRes, Error> {
        let req = ProtoOaSymbolsListReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            include_archived_symbols: None,
        };

        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaSymbolsListRes::from)
    }

    // Request for the list of assets available for a trader's account.
    pub async fn asset_list(&self) -> Result<ProtoOaAssetListRes, Error> {
        let req = ProtoOaAssetListReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
        };

        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaAssetListRes::from)
    }

    // Request for getting a full symbol entity.
    pub async fn symbol_by_id(&self, symbol_ids: Vec<i64>) -> Result<ProtoOaSymbolByIdRes, Error> {
        let req = ProtoOaSymbolByIdReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            symbol_id: symbol_ids,
        };

        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaSymbolByIdRes::from)
    }

    // Request for a list of asset classes available for the trader's account.
    pub async fn asset_class_list(&self) -> Result<ProtoOaAssetClassListRes, Error> {
        let req = ProtoOaAssetClassListReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
        };

        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaAssetClassListRes::from)
    }

    // Request for a list of symbol categories available for a trading account.
    pub async fn symbol_category_list(&self) -> Result<ProtoOaSymbolCategoryListRes, Error> {
        let req = ProtoOaSymbolCategoryListReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
        };

        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaSymbolCategoryListRes::from)
    }
}
