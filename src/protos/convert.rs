use prost::bytes::BytesMut;
use prost::Message;
use std::convert::From;

use crate::protos::spotware_message::*;

macro_rules! convert_impl {
    ($req:ident, $res:ty) => {
        to_message2!($req, $req);
        to_response!($res);
    };
    ($req:ident, $alias:ident, $res:ty) => {
        to_message2!($req, $alias);
        to_response!($res);
    };
}

macro_rules! to_message {
    ($req:ident) => {
        to_message2!($req, $req);
    };
}

macro_rules! to_message2 {
    ($req:ty, $req_id:ident) => {
        impl From<$req> for ProtoMessage {
            fn from(req: $req) -> Self {
                let v = req.encode_to_vec();
                ProtoMessage {
                    payload_type: ProtoOaPayloadType::$req_id as u32,
                    payload: Some(v),
                    client_msg_id: None,
                }
            }
        }
    };
}

macro_rules! to_response {
    ($res:ty) => {
        impl From<ProtoMessage> for $res {
            fn from(message: ProtoMessage) -> Self {
                let buf = BytesMut::from(message.payload.unwrap().as_slice());
                <$res>::decode(buf).unwrap()
            }
        }
    };
}

convert_impl!(ProtoOaApplicationAuthReq, ProtoOaApplicationAuthRes);
convert_impl!(ProtoOaAccountAuthReq, ProtoOaAccountAuthRes);
convert_impl!(ProtoOaVersionReq, ProtoOaVersionRes);
to_message!(ProtoOaNewOrderReq);
to_message!(ProtoOaCancelOrderReq);
to_message!(ProtoOaAmendOrderReq);
to_message!(ProtoOaAmendPositionSltpReq);
to_message!(ProtoOaClosePositionReq);

convert_impl!(ProtoOaAssetListReq, ProtoOaAssetListRes);
convert_impl!(ProtoOaSymbolsListReq, ProtoOaSymbolsListRes);

convert_impl!(ProtoOaSymbolByIdReq, ProtoOaSymbolByIdRes);
convert_impl!(
    ProtoOaSymbolsForConversionReq,
    ProtoOaSymbolsForConversionRes
);
convert_impl!(ProtoOaAssetClassListReq, ProtoOaAssetClassListRes);
convert_impl!(ProtoOaTraderReq, ProtoOaTraderRes);
convert_impl!(ProtoOaReconcileReq, ProtoOaReconcileRes);
convert_impl!(ProtoOaDealListReq, ProtoOaDealListRes);
convert_impl!(ProtoOaOrderListReq, ProtoOaOrderListRes);
convert_impl!(ProtoOaExpectedMarginReq, ProtoOaExpectedMarginRes);
convert_impl!(ProtoOaCashFlowHistoryListReq, ProtoOaCashFlowHistoryListRes);

convert_impl!(
    ProtoOaGetAccountListByAccessTokenReq,
    ProtoOaGetAccountsByAccessTokenReq,
    ProtoOaGetAccountListByAccessTokenRes
);
convert_impl!(ProtoOaRefreshTokenReq, ProtoOaRefreshTokenRes);
convert_impl!(ProtoOaSubscribeSpotsReq, ProtoOaSubscribeSpotsRes);
convert_impl!(ProtoOaUnsubscribeSpotsReq, ProtoOaUnsubscribeSpotsRes);
convert_impl!(
    ProtoOaSubscribeLiveTrendbarReq,
    ProtoOaSubscribeLiveTrendbarRes
);
convert_impl!(
    ProtoOaUnsubscribeLiveTrendbarReq,
    ProtoOaUnsubscribeLiveTrendbarRes
);
convert_impl!(ProtoOaGetTrendbarsReq, ProtoOaGetTrendbarsRes);
convert_impl!(
    ProtoOaGetTickDataReq,
    ProtoOaGetTickdataReq,
    ProtoOaGetTickDataRes
);
convert_impl!(
    ProtoOaGetCtidProfileByTokenReq,
    ProtoOaGetCtidProfileByTokenRes
);
convert_impl!(
    ProtoOaSubscribeDepthQuotesReq,
    ProtoOaSubscribeDepthQuotesRes
);
convert_impl!(
    ProtoOaUnsubscribeDepthQuotesReq,
    ProtoOaUnsubscribeDepthQuotesRes
);
convert_impl!(
    ProtoOaSymbolCategoryListReq,
    ProtoOaSymbolCategoryReq,
    ProtoOaSymbolCategoryListRes
);
convert_impl!(ProtoOaAccountLogoutReq, ProtoOaAccountLogoutRes);
convert_impl!(ProtoOaMarginCallListReq, ProtoOaMarginCallListRes);
convert_impl!(ProtoOaMarginCallUpdateReq, ProtoOaMarginCallUpdateRes);
convert_impl!(
    ProtoOaGetDynamicLeverageByIdReq,
    ProtoOaGetDynamicLeverageReq,
    ProtoOaGetDynamicLeverageByIdRes
);

convert_impl!(
    ProtoOaDealListByPositionIdReq,
    ProtoOaDealListByPositionIdRes
);

// version 88
convert_impl!(ProtoOaOrderDetailsReq, ProtoOaOrderDetailsRes);

convert_impl!(
    ProtoOaOrderListByPositionIdReq,
    ProtoOaOrderListByPositionIdRes
);

convert_impl!(ProtoOaDealOffsetListReq, ProtoOaDealOffsetListRes);

// TODO:why to_meeage2
convert_impl!(
    ProtoOaGetPositionUnrealizedPnLReq,
    ProtoOaGetPositionUnrealizedPnlReq,
    ProtoOaGetPositionUnrealizedPnLRes
);

// Event
to_response!(ProtoOaErrorRes);
to_response!(ProtoOaClientDisconnectEvent);
to_response!(ProtoOaAccountsTokenInvalidatedEvent);
to_response!(ProtoOaExecutionEvent);
to_response!(ProtoOaTrailingSlChangedEvent);
to_response!(ProtoOaSymbolChangedEvent);
to_response!(ProtoOaTraderUpdatedEvent);
to_response!(ProtoOaOrderErrorEvent);
to_response!(ProtoOaMarginChangedEvent);
to_response!(ProtoOaSpotEvent);
to_response!(ProtoOaDepthEvent);
to_response!(ProtoOaAccountDisconnectEvent);
to_response!(ProtoOaMarginCallUpdateEvent);
to_response!(ProtoOaMarginCallTriggerEvent);
//
to_response!(ProtoErrorRes);
to_response!(ProtoHeartbeatEvent);

impl From<ProtoHeartbeatEvent> for ProtoMessage {
    fn from(req: ProtoHeartbeatEvent) -> Self {
        let v = req.encode_to_vec();
        ProtoMessage {
            payload_type: ProtoPayloadType::HeartbeatEvent as u32,
            payload: Some(v),
            client_msg_id: None,
        }
    }
}
