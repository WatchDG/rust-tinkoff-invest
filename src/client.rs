use std::error::Error;
use std::time::Duration;
use uuid::Uuid;

use crate::traits::{ToAccountId, ToOrderId};
use crate::{
    TinkoffInvestCallContext, TinkoffInvestError, TinkoffInvestInterceptor, enums, traits, types,
};
use tinkoff_invest_types::{
    self, CancelOrderRequest, GetAccountsRequest, GetCandlesRequest, GetOrderBookRequest,
    GetTradingStatusRequest, InstrumentIdType, InstrumentRequest, InstrumentsRequest,
    OperationsRequest, OrderIdType, PortfolioRequest, PositionsRequest, PostOrderRequest,
    instruments_service_client::InstrumentsServiceClient,
    market_data_service_client::MarketDataServiceClient,
    operations_service_client::OperationsServiceClient, orders_service_client::OrdersServiceClient,
    portfolio_request::CurrencyRequest, users_service_client::UsersServiceClient,
};
use tonic::{
    Request as TonicRequest,
    codec::CompressionEncoding,
    codegen::InterceptedService,
    service::Interceptor,
    transport::{Channel, ClientTlsConfig, Endpoint},
};

/// Флаги для включения сервисных клиентов в TinkoffInvestBuilder
#[derive(Clone, Copy, Default)]
pub struct TinkoffInvestBuilderFlags(u8);

impl TinkoffInvestBuilderFlags {
    const USERS: u8 = 1 << 0;
    const INSTRUMENTS: u8 = 1 << 1;
    const MARKET_DATA: u8 = 1 << 2;
    const OPERATIONS: u8 = 1 << 3;
    const ORDERS: u8 = 1 << 4;

    #[inline]
    pub fn new() -> Self {
        Self(0)
    }

    #[inline]
    pub fn enable(&mut self, flag: u8) {
        self.0 |= flag;
    }

    #[inline]
    pub fn disable(&mut self, flag: u8) {
        self.0 &= !flag;
    }

    #[inline]
    pub fn set(&mut self, flag: u8, value: bool) {
        if value {
            self.enable(flag);
        } else {
            self.disable(flag);
        }
    }

    #[inline]
    pub fn is_enabled(&self, flag: u8) -> bool {
        (self.0 & flag) != 0
    }

    #[inline]
    pub fn users_enabled(&self) -> bool {
        self.is_enabled(Self::USERS)
    }

    #[inline]
    pub fn instruments_enabled(&self) -> bool {
        self.is_enabled(Self::INSTRUMENTS)
    }

    #[inline]
    pub fn market_data_enabled(&self) -> bool {
        self.is_enabled(Self::MARKET_DATA)
    }

    #[inline]
    pub fn operations_enabled(&self) -> bool {
        self.is_enabled(Self::OPERATIONS)
    }

    #[inline]
    pub fn orders_enabled(&self) -> bool {
        self.is_enabled(Self::ORDERS)
    }
}

macro_rules! create_service_client {
    ($channel:expr, $interceptor:expr, $enabled:expr, $factory:expr, $max_size:expr) => {
        if $enabled {
            let mut client = $factory($channel.clone(), $interceptor.clone());
            client = client.send_compressed(CompressionEncoding::Gzip);
            client = client.accept_compressed(CompressionEncoding::Gzip);
            client = client.max_decoding_message_size($max_size);
            Some(client)
        } else {
            None
        }
    };
}

pub struct TinkoffInvestBuilder<I>
where
    I: Interceptor + Clone,
{
    endpoint: Option<Endpoint>,
    interceptor: Option<I>,
    flags: TinkoffInvestBuilderFlags,
}

impl<I> TinkoffInvestBuilder<I>
where
    I: Interceptor + Clone,
{
    /// URL эндпоинта Tinkoff Invest API по умолчанию
    const DEFAULT_ENDPOINT: &'static str = "https://invest-public-api.tinkoff.ru";

    /// Таймаут подключения по умолчанию (10 секунд)
    const DEFAULT_TIMEOUT: Duration = Duration::from_millis(10000);

    /// Максимальный размер декодируемого сообщения (256 MB)
    const MAX_DECODING_MESSAGE_SIZE: usize = 256 * 1024 * 1024;

    #[inline]
    pub fn new() -> Self {
        Self {
            endpoint: None,
            interceptor: None,
            flags: TinkoffInvestBuilderFlags::new(),
        }
    }

    #[inline]
    pub fn set_endpoint(mut self, endpoint: Option<Endpoint>) -> Self {
        self.endpoint = endpoint;
        self
    }

    #[inline]
    pub fn set_interceptor(mut self, interceptor: Option<I>) -> Self {
        self.interceptor = interceptor;
        self
    }

    #[inline]
    pub fn enable_users_service_client(mut self, value: bool) -> Self {
        self.flags.set(TinkoffInvestBuilderFlags::USERS, value);
        self
    }

    #[inline]
    pub fn enable_instruments_service_client(mut self, value: bool) -> Self {
        self.flags
            .set(TinkoffInvestBuilderFlags::INSTRUMENTS, value);
        self
    }

    #[inline]
    pub fn enable_market_data_service_client(mut self, value: bool) -> Self {
        self.flags
            .set(TinkoffInvestBuilderFlags::MARKET_DATA, value);
        self
    }

    #[inline]
    pub fn enable_operations_service_client(mut self, value: bool) -> Self {
        self.flags.set(TinkoffInvestBuilderFlags::OPERATIONS, value);
        self
    }

    #[inline]
    pub fn enable_orders_service_client(mut self, value: bool) -> Self {
        self.flags.set(TinkoffInvestBuilderFlags::ORDERS, value);
        self
    }

    #[inline]
    pub async fn build(self) -> Result<TinkoffInvest<I>, Box<dyn Error>> {
        let endpoint = self.endpoint.unwrap_or_else(|| {
            Channel::from_static(Self::DEFAULT_ENDPOINT)
                .tls_config(ClientTlsConfig::new().with_native_roots())
                .unwrap()
                .timeout(Self::DEFAULT_TIMEOUT)
        });
        let channel = endpoint.connect().await?;
        let interceptor = self
            .interceptor
            .ok_or(TinkoffInvestError::InterceptorNotSet)?;

        let users_service_client = create_service_client!(
            &channel,
            &interceptor,
            self.flags.users_enabled(),
            UsersServiceClient::with_interceptor,
            Self::MAX_DECODING_MESSAGE_SIZE
        );

        let instruments_service_client = create_service_client!(
            &channel,
            &interceptor,
            self.flags.instruments_enabled(),
            InstrumentsServiceClient::with_interceptor,
            Self::MAX_DECODING_MESSAGE_SIZE
        );

        let market_data_service_client = create_service_client!(
            &channel,
            &interceptor,
            self.flags.market_data_enabled(),
            MarketDataServiceClient::with_interceptor,
            Self::MAX_DECODING_MESSAGE_SIZE
        );

        let operations_service_client = create_service_client!(
            &channel,
            &interceptor,
            self.flags.operations_enabled(),
            OperationsServiceClient::with_interceptor,
            Self::MAX_DECODING_MESSAGE_SIZE
        );

        let orders_service_client = create_service_client!(
            &channel,
            &interceptor,
            self.flags.orders_enabled(),
            OrdersServiceClient::with_interceptor,
            Self::MAX_DECODING_MESSAGE_SIZE
        );

        Ok(TinkoffInvest {
            endpoint,
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
        TinkoffInvestBuilder::new()
            .set_interceptor(Some(interceptor))
            .enable_users_service_client(true)
            .enable_instruments_service_client(true)
            .enable_market_data_service_client(true)
            .enable_operations_service_client(true)
            .enable_orders_service_client(true)
            .build()
            .await
    }
}

impl<I> TinkoffInvest<I>
where
    I: Interceptor,
{
    /// Создает Request с установленным x-tracking-id из TinkoffInvestCallContext
    fn create_request_with_context<T>(
        message: T,
        ctx: &TinkoffInvestCallContext,
    ) -> TonicRequest<T> {
        let mut request = TonicRequest::new(message);
        let request_id_string = ctx
            .request_id
            .as_deref()
            .map(|s| s.to_string())
            .unwrap_or_else(|| Uuid::now_v7().to_string());
        request
            .metadata_mut()
            .insert("x-tracking-id", request_id_string.parse().unwrap());
        request
    }

    pub async fn accounts(
        &mut self,
        ctx: &TinkoffInvestCallContext,
    ) -> Result<Vec<types::Account>, Box<dyn Error>> {
        let client = self
            .users_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::UsersServiceClientNotInit)?;
        let message = GetAccountsRequest {
            ..Default::default()
        };
        let request = Self::create_request_with_context(message, ctx);
        let accounts = client.get_accounts(request).await?.into_inner().accounts;
        Ok(accounts.into_iter().map(|v| v.into()).collect())
    }

    pub async fn market_instruments(
        &mut self,
        ctx: &TinkoffInvestCallContext,
        instrument_type: enums::InstrumentType,
    ) -> Result<Vec<types::MarketInstrument>, Box<dyn Error>> {
        match instrument_type {
            enums::InstrumentType::Share => self.shares(ctx).await,
            enums::InstrumentType::Currency => self.currencies(ctx).await,
            enums::InstrumentType::Future => self.futures(ctx).await,
            // enums::InstrumentType::Option => self.options(ctx).await,
        }
    }

    pub async fn market_instrument<T>(
        &mut self,
        ctx: &TinkoffInvestCallContext,
        instrument: T,
    ) -> Result<Option<types::MarketInstrument>, Box<dyn Error>>
    where
        T: traits::ToInstrumentType + traits::ToFigi,
    {
        match instrument.to_instrument_type() {
            enums::InstrumentType::Share => self.share(ctx, instrument).await,
            enums::InstrumentType::Currency => self.currency(ctx, instrument).await,
            enums::InstrumentType::Future => self.future(ctx, instrument).await,
            // enums::InstrumentType::Option => self.option(ctx, instrument).await,
        }
    }

    pub async fn shares(
        &mut self,
        ctx: &TinkoffInvestCallContext,
    ) -> Result<Vec<types::MarketInstrument>, Box<dyn Error>> {
        let client = self
            .instruments_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::InstrumentsServiceClientNotInit)?;
        let mut message = InstrumentsRequest::default();
        message.set_instrument_status(tinkoff_invest_types::InstrumentStatus::All);
        let request = Self::create_request_with_context(message, ctx);
        let shares = client.shares(request).await?.into_inner().instruments;
        Ok(shares.into_iter().map(|x| x.into()).collect())
    }

    pub async fn share<T>(
        &mut self,
        ctx: &TinkoffInvestCallContext,
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
        let mut message = InstrumentRequest {
            id: instrument.to_figi().into(),
            ..Default::default()
        };
        message.set_id_type(InstrumentIdType::Figi);
        let request = Self::create_request_with_context(message, ctx);
        let share = client.share_by(request).await?.into_inner().instrument;
        Ok(share.map(|x| x.into()))
    }

    pub async fn currencies(
        &mut self,
        ctx: &TinkoffInvestCallContext,
    ) -> Result<Vec<types::MarketInstrument>, Box<dyn Error>> {
        let client = self
            .instruments_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::InstrumentsServiceClientNotInit)?;
        let mut message = InstrumentsRequest::default();
        message.set_instrument_status(tinkoff_invest_types::InstrumentStatus::All);
        let request = Self::create_request_with_context(message, ctx);
        let currencies = client.currencies(request).await?.into_inner().instruments;
        Ok(currencies.into_iter().map(|v| v.into()).collect())
    }

    pub async fn currency<T>(
        &mut self,
        ctx: &TinkoffInvestCallContext,
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
        let mut message = InstrumentRequest {
            id: instrument.to_figi().into(),
            ..Default::default()
        };
        message.set_id_type(InstrumentIdType::Figi);
        let request = Self::create_request_with_context(message, ctx);
        let currency = client.currency_by(request).await?.into_inner().instrument;
        Ok(currency.map(|x| x.into()))
    }

    pub async fn futures(
        &mut self,
        ctx: &TinkoffInvestCallContext,
    ) -> Result<Vec<types::MarketInstrument>, Box<dyn Error>> {
        let client = self
            .instruments_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::InstrumentsServiceClientNotInit)?;
        let mut message = InstrumentsRequest::default();
        message.set_instrument_status(tinkoff_invest_types::InstrumentStatus::All);
        let request = Self::create_request_with_context(message, ctx);
        let futures = client.futures(request).await?.into_inner().instruments;
        Ok(futures.into_iter().map(|v| v.into()).collect())
    }

    pub async fn future<T>(
        &mut self,
        ctx: &TinkoffInvestCallContext,
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
        let mut message = InstrumentRequest {
            id: instrument.to_figi().into(),
            ..Default::default()
        };
        message.set_id_type(InstrumentIdType::Figi);
        let request = Self::create_request_with_context(message, ctx);
        let future = client.future_by(request).await?.into_inner().instrument;
        Ok(future.map(|x| x.into()))
    }

    // pub async fn options(&mut self) -> Result<Vec<types::MarketInstrument>, Box<dyn Error>> {
    //     let client = self
    //         .instruments_service_client
    //         .as_mut()
    //         .ok_or(TinkoffInvestError::InstrumentsServiceClientNotInit)?;
    //     let mut request = InstrumentsRequest::default();
    //     request.set_instrument_status(tinkoff_invest_types::InstrumentStatus::All);
    //     let futures = client.options(request).await?.into_inner().instruments;
    //     Ok(futures.into_iter().map(|v| v.into()).collect())
    // }

    // pub async fn option<T>(
    //     &mut self,
    //     instrument: T,
    // ) -> Result<Option<types::MarketInstrument>, Box<dyn Error>>
    // where
    //     T: traits::ToInstrumentType + traits::ToFigi,
    // {
    //     if instrument.to_instrument_type() != enums::InstrumentType::Future {
    //         return Err(TinkoffInvestError::MarketInstrumentTypeNotFuture.into());
    //     }
    //     let client = self
    //         .instruments_service_client
    //         .as_mut()
    //         .ok_or(TinkoffInvestError::InstrumentsServiceClientNotInit)?;
    //     let mut request = InstrumentRequest {
    //         id: instrument.to_figi().into(),
    //         ..Default::default()
    //     };
    //     request.set_id_type(InstrumentIdType::Figi);
    //     let future = client.option_by(request).await?.into_inner().instrument;
    //     Ok(future.as_ref().map(|x| x.clone().into()))
    // }

    pub async fn trading_status<T>(
        &mut self,
        ctx: &TinkoffInvestCallContext,
        instrument: T,
    ) -> Result<enums::TradingStatus, Box<dyn Error>>
    where
        T: traits::ToUid,
    {
        let client = self
            .market_data_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::MarketDataServiceClientNotInit)?;
        let message = GetTradingStatusRequest {
            instrument_id: Some(instrument.to_uid().into()),
            ..Default::default()
        };
        let request = Self::create_request_with_context(message, ctx);
        Ok(client
            .get_trading_status(request)
            .await?
            .into_inner()
            .trading_status()
            .into())
    }

    pub async fn candlesticks<T>(
        &mut self,
        ctx: &TinkoffInvestCallContext,
        instrument: T,
        interval: enums::CandlestickInterval,
        from: types::DateTime,
        to: types::DateTime,
    ) -> Result<Vec<types::Candlestick>, Box<dyn Error>>
    where
        T: traits::ToUid,
    {
        let uid = instrument.to_uid();
        let uid_clone = uid.clone();
        let interval_clone = interval.clone();
        let mut message = GetCandlesRequest {
            instrument_id: Some(uid.into()),
            from: Some(from.into()),
            to: Some(to.into()),
            ..Default::default()
        };
        message.set_interval(interval_clone.clone().into());
        let client = self
            .market_data_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::MarketDataServiceClientNotInit)?;
        let request = Self::create_request_with_context(message, ctx);
        let candlesticks = client.get_candles(request).await?.into_inner().candles;
        Ok(candlesticks
            .into_iter()
            .map(|x| {
                let mut candlestick = types::Candlestick::from(x);
                candlestick.uid = Some(uid_clone.clone());
                candlestick.interval = Some(interval_clone.clone());
                candlestick
            })
            .collect())
    }

    pub async fn orderbook<T>(
        &mut self,
        ctx: &TinkoffInvestCallContext,
        instrument: T,
        depth: usize,
    ) -> Result<types::OrderBook, Box<dyn Error>>
    where
        T: traits::ToUid,
    {
        let message = GetOrderBookRequest {
            depth: depth as i32,
            instrument_id: Some(instrument.to_uid().into()),
            ..Default::default()
        };
        let client = self
            .market_data_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::MarketDataServiceClientNotInit)?;
        let request = Self::create_request_with_context(message, ctx);
        Ok(client.get_order_book(request).await?.into_inner().into())
    }

    pub async fn order(
        &mut self,
        ctx: &TinkoffInvestCallContext,
    ) -> Result<types::Order, Box<dyn Error>> {
        let client = self
            .orders_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::OrdersServiceClientNotInit)?;
        let message = tinkoff_invest_types::GetOrderStateRequest {
            account_id: ctx.to_account_id().into(),
            order_id: ctx.to_order_id().into(),
            ..Default::default()
        };
        let request = Self::create_request_with_context(message, ctx);
        let order_state = client.get_order_state(request).await?.into_inner();
        Ok(types::Order::from(order_state))
    }

    #[inline]
    pub async fn operations<K>(
        &mut self,
        ctx: &TinkoffInvestCallContext,
        instrument: K,
        state: enums::OperationState,
        from: types::DateTime,
        to: types::DateTime,
    ) -> Result<Vec<types::Operation>, Box<dyn Error>>
    where
        K: traits::ToFigi,
    {
        let from = Some(from.into());
        let to = Some(to.into());
        let client = self
            .operations_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::OperationsServiceClientNotInit)?;
        let mut message = OperationsRequest {
            account_id: ctx.to_account_id().into(),
            figi: Some(instrument.to_figi().into()),
            state: Some(0),
            from,
            to,
        };
        message.set_state(state.into());
        let request = Self::create_request_with_context(message, ctx);
        let response = client.get_operations(request).await?;
        let operations = response.into_inner().operations;
        Ok(operations.into_iter().map(|x| x.into()).collect())
    }

    pub async fn portfolio(
        &mut self,
        ctx: &TinkoffInvestCallContext,
    ) -> Result<Vec<types::PortfolioPosition>, Box<dyn Error>> {
        let mut message = PortfolioRequest {
            account_id: ctx.to_account_id().into(),
            ..Default::default()
        };
        message.set_currency(CurrencyRequest::Rub);
        let client = self
            .operations_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::OperationsServiceClientNotInit)?;
        let request = Self::create_request_with_context(message, ctx);
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

    pub async fn positions(
        &mut self,
        ctx: &TinkoffInvestCallContext,
    ) -> Result<types::Positions, Box<dyn Error>> {
        let message = PositionsRequest {
            account_id: ctx.to_account_id().into(),
        };
        let client = self
            .operations_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::OperationsServiceClientNotInit)?;
        let request = Self::create_request_with_context(message, ctx);
        let response = client.get_positions(request).await?;
        let positions = response.into_inner().into();
        Ok(positions)
    }

    #[inline]
    pub async fn limit_order(
        &mut self,
        ctx: &TinkoffInvestCallContext,
        instrument: impl traits::ToUid,
        direction: enums::OrderDirection,
        quantity: u64,
        price: types::MoneyValue,
    ) -> Result<types::Order, Box<dyn Error>> {
        let mut message = PostOrderRequest {
            order_id: ctx.to_order_id().into(),
            account_id: ctx.to_account_id().into(),
            instrument_id: instrument.to_uid().into(),
            quantity: quantity as i64,
            price: Some(price.into()),
            ..Default::default()
        };
        message.set_direction(direction.into());
        message.set_order_type(tinkoff_invest_types::OrderType::Limit);
        let client = self
            .orders_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::OrdersServiceClientNotInit)?;
        let request = Self::create_request_with_context(message, ctx);
        let response = client.post_order(request).await?;
        let order = response.into_inner().into();
        Ok(order)
    }

    #[inline]
    pub async fn cancel_order(
        &mut self,
        ctx: &TinkoffInvestCallContext,
    ) -> Result<Option<types::DateTime>, Box<dyn Error>> {
        let mut message = CancelOrderRequest {
            account_id: ctx.to_account_id().into(),
            order_id: ctx.to_order_id().into(),
            ..Default::default()
        };
        message.set_order_id_type(OrderIdType::Exchange);
        let client = self
            .orders_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::OrdersServiceClientNotInit)?;
        let request = Self::create_request_with_context(message, ctx);
        let response = client.cancel_order(request).await?;
        Ok(response.into_inner().time.map(|x| x.into()))
    }
}
