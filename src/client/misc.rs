use super::Session;
use crate::{protos::spotware_message::*, Error};

impl Session {
    // Request for getting a conversion chain between two assets that consists of several symbols.
    // Use when no direct quote is available
    pub async fn symbol_for_conversion(
        &self,
        first_asset_id: i64,
        last_asset_id: i64,
    ) -> Result<ProtoOaSymbolsForConversionRes, Error> {
        let req = ProtoOaSymbolsForConversionReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            first_asset_id,
            last_asset_id,
        };

        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaSymbolsForConversionRes::from)
    }

    // Request for getting details of Trader's profile.
    pub async fn get_ctid_profile_by_token(
        &self,
        access_token: &str,
    ) -> Result<ProtoOaGetCtidProfileByTokenRes, Error> {
        let req = ProtoOaGetCtidProfileByTokenReq {
            payload_type: None,
            access_token: access_token.to_string(),
        };

        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaGetCtidProfileByTokenRes::from)
    }

    pub async fn order_details(&self, order_id: i64) -> Result<ProtoOaOrderDetailsRes, Error> {
        let req = ProtoOaOrderDetailsReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            order_id,
        };
        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaOrderDetailsRes::from)
    }

    pub async fn deal_offset_list(&self, deal_id: i64) -> Result<ProtoOaDealOffsetListRes, Error> {
        let req = ProtoOaDealOffsetListReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            deal_id,
        };
        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaDealOffsetListRes::from)
    }

    pub async fn get_position_unrealized_pnl(
        &self,
    ) -> Result<ProtoOaGetPositionUnrealizedPnLRes, Error> {
        let req = ProtoOaGetPositionUnrealizedPnLReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
        };
        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaGetPositionUnrealizedPnLRes::from)
    }
}
