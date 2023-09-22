use std::error::Error;
use std::time::Duration;
use uuid::Uuid;

use crate::{enums, traits, types, TinkoffInvestError, TinkoffInvestInterceptor};
use tinkoff_invest_types::{
    self, instruments_service_client::InstrumentsServiceClient,
    market_data_service_client::MarketDataServiceClient,
    operations_service_client::OperationsServiceClient, orders_service_client::OrdersServiceClient,
    portfolio_request::CurrencyRequest, users_service_client::UsersServiceClient,
    CancelOrderRequest, GetAccountsRequest, GetCandlesRequest, GetOrderBookRequest,
    GetTradingStatusRequest, InstrumentIdType, InstrumentRequest, InstrumentsRequest,
    OperationsRequest, PortfolioRequest, PositionsRequest, PostOrderRequest,
};
use tonic::{
    codec::CompressionEncoding,
    codegen::InterceptedService,
    service::Interceptor,
    transport::{Channel, ClientTlsConfig, Endpoint},
};

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
            .unwrap()
            .timeout(Duration::from_millis(10000));
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
    pub async fn build(self) -> Result<TinkoffInvest<I>, Box<dyn Error>> {
        let channel = self.endpoint.clone().connect().await?;
        let interceptor = self
            .interceptor
            .ok_or(TinkoffInvestError::InterceptorNotSet)?;
        let users_service_client = if self.enable_users_service_client {
            let mut client =
                UsersServiceClient::with_interceptor(channel.clone(), interceptor.clone());
            client = client.send_compressed(CompressionEncoding::Gzip);
            client = client.accept_compressed(CompressionEncoding::Gzip);
            Some(client)
        } else {
            None
        };
        let instruments_service_client = if self.enable_instruments_service_client {
            let mut client =
                InstrumentsServiceClient::with_interceptor(channel.clone(), interceptor.clone());
            client = client.send_compressed(CompressionEncoding::Gzip);
            client = client.accept_compressed(CompressionEncoding::Gzip);
            Some(client)
        } else {
            None
        };
        let market_data_service_client = if self.enable_market_data_service_client {
            let mut client =
                MarketDataServiceClient::with_interceptor(channel.clone(), interceptor.clone());
            client = client.send_compressed(CompressionEncoding::Gzip);
            client = client.accept_compressed(CompressionEncoding::Gzip);
            Some(client)
        } else {
            None
        };
        let operations_service_client = if self.enable_operations_service_client {
            let mut client =
                OperationsServiceClient::with_interceptor(channel.clone(), interceptor.clone());
            client = client.send_compressed(CompressionEncoding::Gzip);
            client = client.accept_compressed(CompressionEncoding::Gzip);
            Some(client)
        } else {
            None
        };
        let orders_service_client = if self.enable_orders_service_client {
            let mut client =
                OrdersServiceClient::with_interceptor(channel.clone(), interceptor.clone());
            client = client.send_compressed(CompressionEncoding::Gzip);
            client = client.accept_compressed(CompressionEncoding::Gzip);
            Some(client)
        } else {
            None
        };
        Ok(TinkoffInvest {
            account: None,
            endpoint: self.endpoint,
            channel,
            interceptor,
            users_service_client,
            instruments_service_client,
            market_data_service_client,
            operations_service_client,
            orders_service_client,
        })
    }
}

impl<I> Default for TinkoffInvestBuilder<I>
where
    I: Interceptor + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct TinkoffInvest<I>
where
    I: Interceptor,
{
    account: Option<types::Account>,
    pub(crate) endpoint: Endpoint,
    pub(crate) channel: Channel,
    pub(crate) interceptor: I,
    users_service_client: Option<UsersServiceClient<InterceptedService<Channel, I>>>,
    instruments_service_client: Option<InstrumentsServiceClient<InterceptedService<Channel, I>>>,
    market_data_service_client: Option<MarketDataServiceClient<InterceptedService<Channel, I>>>,
    operations_service_client: Option<OperationsServiceClient<InterceptedService<Channel, I>>>,
    orders_service_client: Option<OrdersServiceClient<InterceptedService<Channel, I>>>,
}

impl TinkoffInvest<TinkoffInvestInterceptor> {
    pub async fn new(token: String) -> Result<Self, Box<dyn Error>> {
        let interceptor = TinkoffInvestInterceptor::new(token);
        let mut builder = TinkoffInvestBuilder::new();
        builder.interceptor(Some(interceptor));
        builder.enable_users_service_client(true);
        builder.enable_instruments_service_client(true);
        builder.enable_market_data_service_client(true);
        builder.enable_operations_service_client(true);
        builder.enable_orders_service_client(true);
        builder.build().await
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
        Ok(accounts.iter().map(|v| v.clone().into()).collect())
    }

    pub async fn market_instruments(
        &mut self,
        instrument_type: enums::InstrumentType,
    ) -> Result<Vec<types::MarketInstrument>, Box<dyn Error>> {
        match instrument_type {
            enums::InstrumentType::Share => self.shares().await,
            enums::InstrumentType::Currency => self.currencies().await,
            enums::InstrumentType::Future => self.futures().await,
            enums::InstrumentType::Option => self.options().await,
        }
    }

    pub async fn market_instrument<T>(
        &mut self,
        instrument: T,
    ) -> Result<Option<types::MarketInstrument>, Box<dyn Error>>
    where
        T: traits::ToInstrumentType + traits::ToFigi,
    {
        match instrument.to_instrument_type() {
            enums::InstrumentType::Share => self.share(instrument).await,
            enums::InstrumentType::Currency => self.currency(instrument).await,
            enums::InstrumentType::Future => self.future(instrument).await,
            enums::InstrumentType::Option => self.option(instrument).await,
        }
    }

    pub async fn shares(&mut self) -> Result<Vec<types::MarketInstrument>, Box<dyn Error>> {
        let client = self
            .instruments_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::InstrumentsServiceClientNotInit)?;
        let mut request = InstrumentsRequest::default();
        request.set_instrument_status(tinkoff_invest_types::InstrumentStatus::All);
        let shares = client.shares(request).await?.into_inner().instruments;
        Ok(shares.into_iter().map(|x| x.into()).collect())
    }

    pub async fn share<T>(
        &mut self,
        instrument: T,
    ) -> Result<Option<types::MarketInstrument>, Box<dyn Error>>
    where
        T: traits::ToInstrumentType + traits::ToFigi,
    {
        if instrument.to_instrument_type() != enums::InstrumentType::Share {
            return Err(TinkoffInvestError::MarketInstrumentTypeNotShare.into());
        }
        let client = self
            .instruments_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::InstrumentsServiceClientNotInit)?;
        let mut request = InstrumentRequest {
            id: instrument.to_figi().into(),
            ..Default::default()
        };
        request.set_id_type(InstrumentIdType::Figi);
        let share = client.share_by(request).await?.into_inner().instrument;
        Ok(share.as_ref().map(|x| x.clone().into()))
    }

    pub async fn currencies(&mut self) -> Result<Vec<types::MarketInstrument>, Box<dyn Error>> {
        let client = self
            .instruments_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::InstrumentsServiceClientNotInit)?;
        let mut request = InstrumentsRequest::default();
        request.set_instrument_status(tinkoff_invest_types::InstrumentStatus::All);
        let currencies = client.currencies(request).await?.into_inner().instruments;
        Ok(currencies.into_iter().map(|v| v.into()).collect())
    }

    pub async fn currency<T>(
        &mut self,
        instrument: T,
    ) -> Result<Option<types::MarketInstrument>, Box<dyn Error>>
    where
        T: traits::ToInstrumentType + traits::ToFigi,
    {
        if instrument.to_instrument_type() != enums::InstrumentType::Currency {
            return Err(TinkoffInvestError::MarketInstrumentTypeNotCurrency.into());
        }
        let client = self
            .instruments_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::InstrumentsServiceClientNotInit)?;
        let mut request = InstrumentRequest {
            id: instrument.to_figi().into(),
            ..Default::default()
        };
        request.set_id_type(InstrumentIdType::Figi);
        let currency = client.currency_by(request).await?.into_inner().instrument;
        Ok(currency.as_ref().map(|x| x.clone().into()))
    }

    pub async fn futures(&mut self) -> Result<Vec<types::MarketInstrument>, Box<dyn Error>> {
        let client = self
            .instruments_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::InstrumentsServiceClientNotInit)?;
        let mut request = InstrumentsRequest::default();
        request.set_instrument_status(tinkoff_invest_types::InstrumentStatus::All);
        let futures = client.futures(request).await?.into_inner().instruments;
        Ok(futures.into_iter().map(|v| v.into()).collect())
    }

    pub async fn future<T>(
        &mut self,
        instrument: T,
    ) -> Result<Option<types::MarketInstrument>, Box<dyn Error>>
    where
        T: traits::ToInstrumentType + traits::ToFigi,
    {
        if instrument.to_instrument_type() != enums::InstrumentType::Future {
            return Err(TinkoffInvestError::MarketInstrumentTypeNotFuture.into());
        }
        let client = self
            .instruments_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::InstrumentsServiceClientNotInit)?;
        let mut request = InstrumentRequest {
            id: instrument.to_figi().into(),
            ..Default::default()
        };
        request.set_id_type(InstrumentIdType::Figi);
        let future = client.future_by(request).await?.into_inner().instrument;
        Ok(future.as_ref().map(|x| x.clone().into()))
    }

    pub async fn options(&mut self) -> Result<Vec<types::MarketInstrument>, Box<dyn Error>> {
        let client = self
            .instruments_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::InstrumentsServiceClientNotInit)?;
        let mut request = InstrumentsRequest::default();
        request.set_instrument_status(tinkoff_invest_types::InstrumentStatus::All);
        let futures = client.options(request).await?.into_inner().instruments;
        Ok(futures.into_iter().map(|v| v.into()).collect())
    }

    pub async fn option<T>(
        &mut self,
        instrument: T,
    ) -> Result<Option<types::MarketInstrument>, Box<dyn Error>>
    where
        T: traits::ToInstrumentType + traits::ToFigi,
    {
        if instrument.to_instrument_type() != enums::InstrumentType::Future {
            return Err(TinkoffInvestError::MarketInstrumentTypeNotFuture.into());
        }
        let client = self
            .instruments_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::InstrumentsServiceClientNotInit)?;
        let mut request = InstrumentRequest {
            id: instrument.to_figi().into(),
            ..Default::default()
        };
        request.set_id_type(InstrumentIdType::Figi);
        let future = client.option_by(request).await?.into_inner().instrument;
        Ok(future.as_ref().map(|x| x.clone().into()))
    }

    pub async fn trading_status<T>(
        &mut self,
        instrument: T,
    ) -> Result<enums::TradingStatus, Box<dyn Error>>
    where
        T: traits::ToUid,
    {
        let client = self
            .market_data_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::MarketDataServiceClientNotInit)?;
        let request = GetTradingStatusRequest {
            instrument_id: instrument.to_uid().into(),
            ..Default::default()
        };
        Ok(client
            .get_trading_status(request)
            .await?
            .into_inner()
            .trading_status()
            .into())
    }

    pub async fn candlesticks<T>(
        &mut self,
        instrument: T,
        interval: enums::CandlestickInterval,
        from: types::DateTime,
        to: types::DateTime,
    ) -> Result<Vec<types::Candlestick>, Box<dyn Error>>
    where
        T: traits::ToUid,
    {
        let uid = instrument.to_uid();
        let mut request = GetCandlesRequest {
            instrument_id: uid.clone().into(),
            from: Some(from.into()),
            to: Some(to.into()),
            ..Default::default()
        };
        request.set_interval(interval.clone().into());
        let client = self
            .market_data_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::MarketDataServiceClientNotInit)?;
        let candlesticks = client.get_candles(request).await?.into_inner().candles;
        Ok(candlesticks
            .into_iter()
            .map(|x| {
                let mut candlestick = types::Candlestick::from(x);
                candlestick.uid = Some(uid.clone());
                candlestick.interval = Some(interval.clone());
                candlestick
            })
            .collect())
    }

    pub async fn orderbook<T>(
        &mut self,
        instrument: T,
        depth: usize,
    ) -> Result<types::OrderBook, Box<dyn Error>>
    where
        T: traits::ToUid,
    {
        let request = GetOrderBookRequest {
            depth: depth as i32,
            instrument_id: instrument.to_uid().into(),
            ..Default::default()
        };
        let client = self
            .market_data_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::MarketDataServiceClientNotInit)?;
        Ok(client.get_order_book(request).await?.into_inner().into())
    }

    pub async fn order_on_account<T>(
        &mut self,
        account: T,
        order_id: types::OrderId,
    ) -> Result<types::Order, Box<dyn Error>>
    where
        T: traits::ToAccountId,
    {
        let client = self
            .orders_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::OrdersServiceClientNotInit)?;
        let request = tinkoff_invest_types::GetOrderStateRequest {
            account_id: account.to_account_id().into(),
            order_id: order_id.into(),
        };
        let order_state = client.get_order_state(request).await?.into_inner();
        Ok(types::Order::from(order_state))
    }

    pub async fn order<T>(
        &mut self,
        order_id: types::OrderId,
    ) -> Result<types::Order, Box<dyn Error>> {
        let account = self
            .account
            .as_ref()
            .ok_or(TinkoffInvestError::AccountNotSet)?
            .clone();
        self.order_on_account(&account, order_id).await
    }

    #[inline]
    pub async fn operations_on_account<T, K>(
        &mut self,
        account: T,
        instrument: K,
        state: enums::OperationState,
        from: types::DateTime,
        to: types::DateTime,
    ) -> Result<Vec<types::Operation>, Box<dyn Error>>
    where
        T: traits::ToAccountId,
        K: traits::ToFigi,
    {
        let from = Some(from.into());
        let to = Some(to.into());
        let client = self
            .operations_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::OperationsServiceClientNotInit)?;
        let mut request = OperationsRequest {
            account_id: account.to_account_id().into(),
            figi: instrument.to_figi().into(),
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
        Ok(operations.into_iter().map(|x| x.into()).collect())
    }

    pub async fn operations<T>(
        &mut self,
        instrument: T,
        state: enums::OperationState,
        from: types::DateTime,
        to: types::DateTime,
    ) -> Result<Vec<types::Operation>, Box<dyn Error>>
    where
        T: traits::ToFigi,
    {
        let account = self
            .account
            .as_ref()
            .ok_or(TinkoffInvestError::AccountNotSet)?
            .clone();
        self.operations_on_account(&account, instrument, state, from, to)
            .await
    }

    pub async fn portfolio_on_account<T>(
        &mut self,
        account: T,
    ) -> Result<Vec<types::PortfolioPosition>, Box<dyn Error>>
    where
        T: traits::ToAccountId,
    {
        let mut request = PortfolioRequest {
            account_id: account.to_account_id().into(),
            ..Default::default()
        };
        request.set_currency(CurrencyRequest::Rub);
        let client = self
            .operations_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::OperationsServiceClientNotInit)?;
        let portfolio_positions = client
            .get_portfolio(request)
            .await?
            .into_inner()
            .positions
            .iter()
            .map(|x| x.into())
            .collect();
        Ok(portfolio_positions)
    }

    pub async fn portfolio(&mut self) -> Result<Vec<types::PortfolioPosition>, Box<dyn Error>> {
        let account = self
            .account
            .as_ref()
            .ok_or(TinkoffInvestError::AccountNotSet)?
            .clone();
        self.portfolio_on_account(&account).await
    }

    pub async fn positions_on_account<T>(
        &mut self,
        account: T,
    ) -> Result<types::Positions, Box<dyn Error>>
    where
        T: traits::ToAccountId,
    {
        let request = PositionsRequest {
            account_id: account.to_account_id().into(),
        };
        let client = self
            .operations_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::OperationsServiceClientNotInit)?;
        let response = client.get_positions(request).await?;
        let positions = response.into_inner().into();
        Ok(positions)
    }

    pub async fn positions(&mut self) -> Result<types::Positions, Box<dyn Error>> {
        let account = self
            .account
            .as_ref()
            .ok_or(TinkoffInvestError::AccountNotSet)?
            .clone();
        self.positions_on_account(&account).await
    }

    #[inline]
    pub async fn limit_order_on_account<T, K>(
        &mut self,
        account: T,
        instrument: K,
        direction: enums::OrderDirection,
        quantity: u64,
        price: types::MoneyValue,
        order_id: Option<String>,
    ) -> Result<types::Order, Box<dyn Error>>
    where
        T: traits::ToAccountId,
        K: traits::ToUid,
    {
        let order_id = order_id.unwrap_or_else(|| Uuid::new_v4().to_string());
        let mut request = PostOrderRequest {
            order_id,
            account_id: account.to_account_id().into(),
            instrument_id: instrument.to_uid().into(),
            quantity: quantity as i64,
            price: Some(price.into()),
            ..Default::default()
        };
        request.set_direction(direction.into());
        request.set_order_type(tinkoff_invest_types::OrderType::Limit);
        let client = self
            .orders_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::OrdersServiceClientNotInit)?;
        Ok(client.post_order(request).await?.into_inner().into())
    }

    pub async fn limit_order<T>(
        &mut self,
        instrument: T,
        direction: enums::OrderDirection,
        quantity: u64,
        price: types::MoneyValue,
        order_id: Option<String>,
    ) -> Result<types::Order, Box<dyn Error>>
    where
        T: traits::ToUid,
    {
        let account = self
            .account
            .as_ref()
            .ok_or(TinkoffInvestError::AccountNotSet)?
            .clone();
        self.limit_order_on_account(&account, instrument, direction, quantity, price, order_id)
            .await
    }

    #[inline]
    pub async fn cancel_order_on_account<T, K>(
        &mut self,
        account: T,
        order: K,
    ) -> Result<Option<types::DateTime>, Box<dyn Error>>
    where
        T: traits::ToAccountId,
        K: traits::ToOrderId,
    {
        let request = CancelOrderRequest {
            account_id: account.to_account_id().into(),
            order_id: order.to_order_id().into(),
        };
        let client = self
            .orders_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::OrdersServiceClientNotInit)?;
        Ok(client
            .cancel_order(request)
            .await?
            .into_inner()
            .time
            .map(|x| x.into()))
    }

    pub async fn cancel_order<T>(
        &mut self,
        order: T,
    ) -> Result<Option<types::DateTime>, Box<dyn Error>>
    where
        T: traits::ToOrderId,
    {
        let account = self
            .account
            .as_ref()
            .ok_or(TinkoffInvestError::AccountNotSet)?
            .clone();
        self.cancel_order_on_account(&account, order).await
    }
}
