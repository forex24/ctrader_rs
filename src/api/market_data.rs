
pub trait LiveMarketDataService {
    async fn subscribe_bar();
    async fn subscribe_quote();
    async fn subscribe_depth();
}

pub trait HistoricalMarketDataService {
    async fn get_histrical_bar();
    async fn get_histrical_tick();
}

pub trait OrderService {
    async fn new_order();
    async fn cacel_order();
    async fn modify_order();
}

pub trait AuthService {
    async fn auth();
    async fn logout();
}

pub trait PositionService {

}

pub trait AccountService {
    async fn account_data();
    async fn logout();
}

pub trait SymbolService {
    async fn symbol_list();
    async fn asset_class_list();
    async fn asset_list();
}

