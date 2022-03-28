use std::convert::TryInto;
use std::error::Error;
use uuid::Uuid;

use tinkoff_invest_types::{
    self, instruments_service_client::InstrumentsServiceClient,
    market_data_service_client::MarketDataServiceClient,
    operations_service_client::OperationsServiceClient, orders_service_client::OrdersServiceClient,
    users_service_client::UsersServiceClient, GetAccountsRequest, GetCandlesRequest,
    GetTradingStatusRequest, InstrumentIdType, InstrumentRequest, InstrumentsRequest,
    OperationsRequest, PostOrderRequest,
};
use tonic::{
    codegen::InterceptedService,
    service::Interceptor,
    transport::{Channel, ClientTlsConfig, Endpoint},
};

use crate::{enums, types, TinkoffInvestError, TinkoffInvestInterceptor};

pub struct TinkoffInvestBuilder<I>
where
    I: Interceptor + Clone,
{
    endpoint: Endpoint,
    interceptor: Option<I>,
    enable_users_service_client: bool,
    enable_instruments_service_client: bool,
    enable_market_data_service_client: bool,
    enable_operations_service_client: bool,
    enable_orders_service_client: bool,
}

impl<I> TinkoffInvestBuilder<I>
where
    I: Interceptor + Clone,
{
    #[inline]
    pub fn new() -> Self {
        let endpoint = Channel::from_static("https://invest-public-api.tinkoff.ru")
            .tls_config(ClientTlsConfig::new())
            .unwrap();
        Self {
            endpoint,
            interceptor: None,
            enable_users_service_client: false,
            enable_instruments_service_client: false,
            enable_market_data_service_client: false,
            enable_operations_service_client: false,
            enable_orders_service_client: false,
        }
    }

    #[inline]
    pub fn endpoint(&mut self, endpoint: Endpoint) -> &Self {
        self.endpoint = endpoint;
        self
    }

    #[inline]
    pub fn interceptor(&mut self, interceptor: Option<I>) -> &Self {
        self.interceptor = interceptor;
        self
    }

    #[inline]
    pub fn enable_users_service_client(&mut self, value: bool) -> &Self {
        self.enable_users_service_client = value;
        self
    }

    #[inline]
    pub fn enable_instruments_service_client(&mut self, value: bool) -> &Self {
        self.enable_instruments_service_client = value;
        self
    }

    #[inline]
    pub fn enable_market_data_service_client(&mut self, value: bool) -> &Self {
        self.enable_market_data_service_client = value;
        self
    }

    #[inline]
    pub fn enable_operations_service_client(&mut self, value: bool) -> &Self {
        self.enable_operations_service_client = value;
        self
    }

    #[inline]
    pub fn enable_orders_service_client(&mut self, value: bool) -> &Self {
        self.enable_orders_service_client = value;
        self
    }

    #[inline]
    pub fn build(self) -> Result<TinkoffInvest<I>, Box<dyn Error>> {
        let channel = self.endpoint.connect_lazy();
        let interceptor = self
            .interceptor
            .ok_or(TinkoffInvestError::InterceptorNotSet)?;
        let users_service_client = if self.enable_users_service_client {
            Some(UsersServiceClient::with_interceptor(
                channel.clone(),
                interceptor.clone(),
            ))
        } else {
            None
        };
        let instruments_service_client = if self.enable_instruments_service_client {
            Some(InstrumentsServiceClient::with_interceptor(
                channel.clone(),
                interceptor.clone(),
            ))
        } else {
            None
        };
        let market_data_service_client = if self.enable_market_data_service_client {
            Some(MarketDataServiceClient::with_interceptor(
                channel.clone(),
                interceptor.clone(),
            ))
        } else {
            None
        };
        let operations_service_client = if self.enable_operations_service_client {
            Some(OperationsServiceClient::with_interceptor(
                channel.clone(),
                interceptor.clone(),
            ))
        } else {
            None
        };
        let orders_service_client = if self.enable_orders_service_client {
            Some(OrdersServiceClient::with_interceptor(
                channel.clone(),
                interceptor.clone(),
            ))
        } else {
            None
        };
        Ok(TinkoffInvest {
            account: None,
            users_service_client,
            instruments_service_client,
            market_data_service_client,
            operations_service_client,
            orders_service_client,
        })
    }
}

pub struct TinkoffInvest<I>
where
    I: Interceptor,
{
    account: Option<types::Account>,
    users_service_client: Option<UsersServiceClient<InterceptedService<Channel, I>>>,
    instruments_service_client: Option<InstrumentsServiceClient<InterceptedService<Channel, I>>>,
    market_data_service_client: Option<MarketDataServiceClient<InterceptedService<Channel, I>>>,
    operations_service_client: Option<OperationsServiceClient<InterceptedService<Channel, I>>>,
    orders_service_client: Option<OrdersServiceClient<InterceptedService<Channel, I>>>,
}

impl TinkoffInvest<TinkoffInvestInterceptor> {
    pub fn new(token: String) -> Result<Self, Box<dyn Error>> {
        let interceptor = TinkoffInvestInterceptor::new(token);
        let mut builder = TinkoffInvestBuilder::new();
        builder.interceptor(Some(interceptor));
        builder.enable_users_service_client(true);
        builder.enable_instruments_service_client(true);
        builder.enable_market_data_service_client(true);
        builder.enable_operations_service_client(true);
        builder.build()
    }
}

impl<I> TinkoffInvest<I>
where
    I: Interceptor,
{
    #[inline]
    pub fn set_account(&mut self, account: Option<types::Account>) -> &Self {
        self.account = account;
        self
    }

    pub async fn accounts(&mut self) -> Result<Vec<types::Account>, Box<dyn Error>> {
        let client = self
            .users_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::UsersServiceClientNotInit)?;
        let request = GetAccountsRequest {};
        let accounts = client.get_accounts(request).await?.into_inner().accounts;
        Ok(accounts.iter().map(|v| v.into()).collect())
    }

    pub async fn shares(&mut self) -> Result<Vec<types::Share>, Box<dyn Error>> {
        let client = self
            .instruments_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::InstrumentsServiceClientNotInit)?;
        let mut request = InstrumentsRequest::default();
        request.set_instrument_status(tinkoff_invest_types::InstrumentStatus::All);
        let shares = client.shares(request).await?.into_inner().instruments;
        Ok(shares.iter().map(|v| v.into()).collect())
    }

    pub async fn share<T>(&mut self, share: T) -> Result<Option<types::Share>, Box<dyn Error>>
    where
        T: TryInto<types::Share>,
        <T as TryInto<types::Share>>::Error: 'static + std::error::Error,
    {
        let share: types::Share = share.try_into()?;
        let client = self
            .instruments_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::InstrumentsServiceClientNotInit)?;
        let mut request = InstrumentRequest::default();
        request.set_id_type(InstrumentIdType::Figi);
        request.id = share.figi.into();
        let share = client.share_by(request).await?.into_inner().instrument;
        Ok(share.as_ref().map(|x| x.into()))
    }

    pub async fn currencies(&mut self) -> Result<Vec<types::Currency>, Box<dyn Error>> {
        let client = self
            .instruments_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::InstrumentsServiceClientNotInit)?;
        let mut request = InstrumentsRequest::default();
        request.set_instrument_status(tinkoff_invest_types::InstrumentStatus::All);
        let currencies = client.currencies(request).await?.into_inner().instruments;
        Ok(currencies.iter().map(|v| v.into()).collect())
    }

    pub async fn currency<T>(
        &mut self,
        currency: T,
    ) -> Result<Option<types::Currency>, Box<dyn Error>>
    where
        T: TryInto<types::Currency>,
        <T as TryInto<types::Currency>>::Error: 'static + std::error::Error,
    {
        let currency: types::Currency = currency.try_into()?;
        let client = self
            .instruments_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::InstrumentsServiceClientNotInit)?;
        let mut request = InstrumentRequest::default();
        request.set_id_type(InstrumentIdType::Figi);
        request.id = currency.figi.into();
        let currency = client.currency_by(request).await?.into_inner().instrument;
        Ok(currency.as_ref().map(|x| x.into()))
    }

    pub async fn trading_status<T>(
        &mut self,
        instrument: T,
    ) -> Result<enums::TradingStatus, Box<dyn Error>>
    where
        T: Into<types::Figi>,
    {
        let figi = instrument.into();
        let client = self
            .market_data_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::MarketDataServiceClientNotInit)?;
        let request = GetTradingStatusRequest { figi: figi.into() };
        Ok(client
            .get_trading_status(request)
            .await?
            .into_inner()
            .trading_status()
            .into())
    }

    pub async fn candlesticks(
        &mut self,
        figi: types::Figi,
        from: Option<types::DateTime>,
        to: Option<types::DateTime>,
        interval: enums::CandlestickInterval,
    ) -> Result<Vec<types::Candlestick>, Box<dyn Error>> {
        let mut request = GetCandlesRequest::default();
        request.figi = figi.into();
        request.from = from.map(|x| x.into());
        request.to = to.map(|x| x.into());
        request.set_interval(interval.into());
        let client = self
            .market_data_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::MarketDataServiceClientNotInit)?;
        let candlesticks = client.get_candles(request).await?.into_inner().candles;
        Ok(candlesticks.iter().map(|v| v.into()).collect())
    }

    #[inline]
    pub async fn operations_on_account(
        &mut self,
        account: &types::Account,
        figi: types::Figi,
        state: enums::OperationState,
        from: Option<types::DateTime>,
        to: Option<types::DateTime>,
    ) -> Result<Vec<types::Operation>, Box<dyn Error>> {
        let figi = figi.into();
        let from: Option<tinkoff_invest_types::extra::Timestamp> = from.map(|x| x.into());
        let to: Option<tinkoff_invest_types::extra::Timestamp> = to.map(|x| x.into());
        let client = self
            .operations_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::OperationsServiceClientNotInit)?;
        let mut request = OperationsRequest {
            account_id: account.id.clone(),
            figi,
            state: 0,
            from,
            to,
        };
        request.set_state(state.into());
        let operations = client
            .get_operations(request)
            .await?
            .into_inner()
            .operations;
        Ok(operations.iter().map(|v| v.into()).collect())
    }

    pub async fn operations(
        &mut self,
        figi: types::Figi,
        state: enums::OperationState,
        from: Option<types::DateTime>,
        to: Option<types::DateTime>,
    ) -> Result<Vec<types::Operation>, Box<dyn Error>> {
        let account = self
            .account
            .as_ref()
            .ok_or(TinkoffInvestError::AccountNotSet)?
            .clone();
        self.operations_on_account(&account, figi, state, from, to)
            .await
    }

    #[inline]
    pub async fn limit_order_on_account(
        &mut self,
        account: &types::Account,
        figi: types::Figi,
        direction: enums::OrderDirection,
        quantity: u64,
        price: types::MoneyValue,
        order_id: Option<String>,
    ) -> Result<types::Order, Box<dyn Error>> {
        let order_id = order_id.unwrap_or_else(|| Uuid::new_v4().to_string());
        let mut request = PostOrderRequest::default();
        request.order_id = order_id;
        request.account_id = account.id.clone();
        request.figi = figi.into();
        request.quantity = quantity as i64;
        request.price = Some(price.into());
        request.set_direction(direction.into());
        request.set_order_type(tinkoff_invest_types::OrderType::Limit);
        let client = self
            .orders_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::OrdersServiceClientNotInit)?;
        Ok(client.post_order(request).await?.into_inner().into())
    }

    pub async fn limit_order(
        &mut self,
        figi: types::Figi,
        direction: enums::OrderDirection,
        quantity: u64,
        price: types::MoneyValue,
        order_id: Option<String>,
    ) -> Result<types::Order, Box<dyn Error>> {
        let account = self
            .account
            .as_ref()
            .ok_or(TinkoffInvestError::AccountNotSet)?
            .clone();
        self.limit_order_on_account(&account, figi, direction, quantity, price, order_id)
            .await
    }
}