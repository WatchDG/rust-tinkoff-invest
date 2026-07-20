use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

use crate::interceptor::TInterceptor;
use crate::traits::{RequestId, ToAccountIdRef, ToOrderIdRef};
use crate::{TError, enums, traits, types};
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

/// Флаги для включения сервисных клиентов в [`TClientBuilder`].
#[derive(Clone, Copy, Default)]
pub struct TClientBuilderFlags(u8);

impl TClientBuilderFlags {
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
    pub fn set(&mut self, flag: u8, value: bool) {
        self.0 = if value { self.0 | flag } else { self.0 & !flag };
    }

    #[inline]
    pub fn is_enabled(&self, flag: u8) -> bool {
        (self.0 & flag) != 0
    }

    #[inline]
    pub fn is_users_enabled(&self) -> bool {
        self.is_enabled(Self::USERS)
    }

    #[inline]
    pub fn is_instruments_enabled(&self) -> bool {
        self.is_enabled(Self::INSTRUMENTS)
    }

    #[inline]
    pub fn is_market_data_enabled(&self) -> bool {
        self.is_enabled(Self::MARKET_DATA)
    }

    #[inline]
    pub fn is_operations_enabled(&self) -> bool {
        self.is_enabled(Self::OPERATIONS)
    }

    #[inline]
    pub fn is_orders_enabled(&self) -> bool {
        self.is_enabled(Self::ORDERS)
    }
}

macro_rules! create_service_client {
    ($enabled:expr, $channel:expr, $interceptor:expr, $factory:expr, $max_size:expr) => {{
        if $enabled {
            let channel_clone = $channel.clone();
            let interceptor_clone = $interceptor.clone();
            let mut client = $factory(channel_clone, interceptor_clone);
            client = client.send_compressed(CompressionEncoding::Gzip);
            client = client.accept_compressed(CompressionEncoding::Gzip);
            client = client.max_decoding_message_size($max_size);
            Some(client)
        } else {
            None
        }
    }};
}

/// Builder для [`TClient`].
///
/// Позволяет выбрать endpoint, interceptor, таймаут и набор gRPC-сервисов.
pub struct TClientBuilder<I>
where
    I: Interceptor + Clone + Send,
{
    endpoint: Option<Endpoint>,
    interceptor: Option<I>,
    flags: TClientBuilderFlags,
    max_decoding_message_size: Option<usize>,
    timeout: Option<Duration>,
}

impl<I> TClientBuilder<I>
where
    I: Interceptor + Clone + Send,
{
    /// URL эндпоинта Tinkoff Invest API по умолчанию
    const DEFAULT_ENDPOINT: &'static str = "https://invest-public-api.tinkoff.ru";

    /// Таймаут подключения по умолчанию (10 секунд)
    const DEFAULT_TIMEOUT: Duration = Duration::from_millis(10000);

    /// Максимальный размер декодируемого сообщения (256 MB)
    const DEFAULT_MAX_DECODING_MESSAGE_SIZE: usize = 256 * 1024 * 1024;

    #[inline]
    pub fn new() -> Self {
        Self {
            endpoint: None,
            interceptor: None,
            flags: TClientBuilderFlags::new(),
            max_decoding_message_size: None,
            timeout: None,
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
        self.flags.set(TClientBuilderFlags::USERS, value);
        self
    }

    #[inline]
    pub fn enable_instruments_service_client(mut self, value: bool) -> Self {
        self.flags.set(TClientBuilderFlags::INSTRUMENTS, value);
        self
    }

    #[inline]
    pub fn enable_market_data_service_client(mut self, value: bool) -> Self {
        self.flags.set(TClientBuilderFlags::MARKET_DATA, value);
        self
    }

    #[inline]
    pub fn enable_operations_service_client(mut self, value: bool) -> Self {
        self.flags.set(TClientBuilderFlags::OPERATIONS, value);
        self
    }

    #[inline]
    pub fn enable_orders_service_client(mut self, value: bool) -> Self {
        self.flags.set(TClientBuilderFlags::ORDERS, value);
        self
    }

    #[inline]
    pub fn set_max_decoding_message_size(mut self, size: Option<usize>) -> Self {
        self.max_decoding_message_size = size;
        self
    }

    #[inline]
    pub fn set_timeout(mut self, timeout: Option<Duration>) -> Self {
        self.timeout = timeout;
        self
    }

    #[inline]
    pub async fn build(self) -> Result<TClient<I>, TError> {
        let timeout = self.timeout.unwrap_or(Self::DEFAULT_TIMEOUT);
        let max_decoding_message_size = self
            .max_decoding_message_size
            .unwrap_or(Self::DEFAULT_MAX_DECODING_MESSAGE_SIZE);
        let endpoint = if let Some(endpoint) = self.endpoint {
            endpoint
        } else {
            Channel::from_static(Self::DEFAULT_ENDPOINT)
                .tls_config(ClientTlsConfig::new().with_native_roots())
                .map_err(|e| TError::TlsConfig(e.to_string()))?
                .timeout(timeout)
                .http2_keep_alive_interval(Duration::from_secs(30))
                .keep_alive_timeout(Duration::from_secs(10))
                .keep_alive_while_idle(true)
        };
        let channel = endpoint.connect().await?;
        let interceptor = self.interceptor.ok_or(TError::InterceptorNotSet)?;

        let users_service_client = create_service_client!(
            self.flags.is_users_enabled(),
            &channel,
            &interceptor,
            UsersServiceClient::with_interceptor,
            max_decoding_message_size
        );

        let instruments_service_client = create_service_client!(
            self.flags.is_instruments_enabled(),
            &channel,
            &interceptor,
            InstrumentsServiceClient::with_interceptor,
            max_decoding_message_size
        );

        let market_data_service_client = create_service_client!(
            self.flags.is_market_data_enabled(),
            &channel,
            &interceptor,
            MarketDataServiceClient::with_interceptor,
            max_decoding_message_size
        );

        let operations_service_client = create_service_client!(
            self.flags.is_operations_enabled(),
            &channel,
            &interceptor,
            OperationsServiceClient::with_interceptor,
            max_decoding_message_size
        );

        let orders_service_client = create_service_client!(
            self.flags.is_orders_enabled(),
            &channel,
            &interceptor,
            OrdersServiceClient::with_interceptor,
            max_decoding_message_size
        );

        Ok(TClient {
            users_service_client,
            instruments_service_client,
            market_data_service_client,
            operations_service_client,
            orders_service_client,
        })
    }
}

impl<I> Default for TClientBuilder<I>
where
    I: Interceptor + Clone + Send,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Клиент Tinkoff Invest API.
///
/// Создаётся через [`TClient::new`] или [`TClientBuilder`].
pub struct TClient<I>
where
    I: Interceptor + Clone + Send,
{
    users_service_client: Option<UsersServiceClient<InterceptedService<Channel, I>>>,
    instruments_service_client: Option<InstrumentsServiceClient<InterceptedService<Channel, I>>>,
    market_data_service_client: Option<MarketDataServiceClient<InterceptedService<Channel, I>>>,
    operations_service_client: Option<OperationsServiceClient<InterceptedService<Channel, I>>>,
    orders_service_client: Option<OrdersServiceClient<InterceptedService<Channel, I>>>,
}

impl TClient<TInterceptor> {
    /// Создаёт клиент со всеми сервисами, включёнными через Cargo features.
    pub async fn new(token: String) -> Result<Self, TError> {
        let interceptor = TInterceptor::new(token)?;
        TClientBuilder::new()
            .set_interceptor(Some(interceptor))
            .enable_users_service_client(cfg!(feature = "users"))
            .enable_instruments_service_client(cfg!(feature = "instruments"))
            .enable_market_data_service_client(cfg!(feature = "market-data"))
            .enable_operations_service_client(cfg!(feature = "operations"))
            .enable_orders_service_client(cfg!(feature = "orders"))
            .build()
            .await
    }
}

impl<I> TClient<I>
where
    I: Interceptor + Clone + Send,
{
    /// Создаёт Request с установленным `x-tracking-id` из контекста.
    fn create_request<T, C>(ctx: &C, message: T) -> Result<TonicRequest<T>, TError>
    where
        C: RequestId,
    {
        let mut request = TonicRequest::new(message);
        let request_id_string = ctx
            .request_id()
            .map(|s| s.to_string())
            .unwrap_or_else(|| Uuid::now_v7().to_string());
        let tracking_id = request_id_string
            .parse()
            .map_err(|e| TError::InvalidMetadata(format!("x-tracking-id: {e}")))?;
        request.metadata_mut().insert("x-tracking-id", tracking_id);
        Ok(request)
    }

    pub async fn accounts<C>(&self, ctx: &C) -> Result<Vec<types::Account>, TError>
    where
        C: RequestId,
    {
        let client = self
            .users_service_client
            .as_ref()
            .ok_or(TError::UsersServiceClientNotInit)?;
        let message = GetAccountsRequest::default();
        let request = Self::create_request(ctx, message)?;
        let mut client = client.clone();
        let accounts = client.get_accounts(request).await?.into_inner().accounts;
        let mut result = Vec::with_capacity(accounts.len());
        for v in accounts {
            result.push(v.into());
        }
        Ok(result)
    }

    pub async fn market_instruments<C>(
        &self,
        ctx: &C,
        instrument_type: enums::InstrumentType,
    ) -> Result<Vec<types::MarketInstrument>, TError>
    where
        C: RequestId,
    {
        match instrument_type {
            enums::InstrumentType::Share => self.shares(ctx).await,
            enums::InstrumentType::Currency => self.currencies(ctx).await,
            enums::InstrumentType::Future => self.futures(ctx).await,
        }
    }

    pub async fn market_instrument<T, C>(
        &self,
        ctx: &C,
        instrument: T,
    ) -> Result<Option<types::MarketInstrument>, TError>
    where
        T: traits::ToInstrumentType + traits::ToFigi,
        C: RequestId,
    {
        match instrument.to_instrument_type() {
            enums::InstrumentType::Share => self.share(ctx, instrument).await,
            enums::InstrumentType::Currency => self.currency(ctx, instrument).await,
            enums::InstrumentType::Future => self.future(ctx, instrument).await,
        }
    }

    pub async fn shares<C>(&self, ctx: &C) -> Result<Vec<types::MarketInstrument>, TError>
    where
        C: RequestId,
    {
        let client = self
            .instruments_service_client
            .as_ref()
            .ok_or(TError::InstrumentsServiceClientNotInit)?;
        let mut message = InstrumentsRequest::default();
        message.set_instrument_status(tinkoff_invest_types::InstrumentStatus::All);
        let request = Self::create_request(ctx, message)?;
        let mut client = client.clone();
        let shares = client.shares(request).await?.into_inner().instruments;
        let mut result = Vec::with_capacity(shares.len());
        for x in shares {
            result.push(x.into());
        }
        Ok(result)
    }

    pub async fn share<T, C>(
        &self,
        ctx: &C,
        instrument: T,
    ) -> Result<Option<types::MarketInstrument>, TError>
    where
        T: traits::ToInstrumentType + traits::ToFigi,
        C: RequestId,
    {
        if instrument.to_instrument_type() != enums::InstrumentType::Share {
            return Err(TError::MarketInstrumentTypeNotShare);
        }
        let client = self
            .instruments_service_client
            .as_ref()
            .ok_or(TError::InstrumentsServiceClientNotInit)?;
        let mut message = InstrumentRequest {
            id: instrument.to_figi().into(),
            ..Default::default()
        };
        message.set_id_type(InstrumentIdType::Figi);
        let request = Self::create_request(ctx, message)?;
        let mut client = client.clone();
        let share = client.share_by(request).await?.into_inner().instrument;
        Ok(share.map(|x| x.into()))
    }

    pub async fn currencies<C>(&self, ctx: &C) -> Result<Vec<types::MarketInstrument>, TError>
    where
        C: RequestId,
    {
        let client = self
            .instruments_service_client
            .as_ref()
            .ok_or(TError::InstrumentsServiceClientNotInit)?;
        let mut message = InstrumentsRequest::default();
        message.set_instrument_status(tinkoff_invest_types::InstrumentStatus::All);
        let request = Self::create_request(ctx, message)?;
        let mut client = client.clone();
        let currencies = client.currencies(request).await?.into_inner().instruments;
        let mut result = Vec::with_capacity(currencies.len());
        for v in currencies {
            result.push(v.into());
        }
        Ok(result)
    }

    pub async fn currency<T, C>(
        &self,
        ctx: &C,
        instrument: T,
    ) -> Result<Option<types::MarketInstrument>, TError>
    where
        T: traits::ToInstrumentType + traits::ToFigi,
        C: RequestId,
    {
        if instrument.to_instrument_type() != enums::InstrumentType::Currency {
            return Err(TError::MarketInstrumentTypeNotCurrency);
        }
        let client = self
            .instruments_service_client
            .as_ref()
            .ok_or(TError::InstrumentsServiceClientNotInit)?;
        let mut message = InstrumentRequest {
            id: instrument.to_figi().into(),
            ..Default::default()
        };
        message.set_id_type(InstrumentIdType::Figi);
        let request = Self::create_request(ctx, message)?;
        let mut client = client.clone();
        let currency = client.currency_by(request).await?.into_inner().instrument;
        Ok(currency.map(|x| x.into()))
    }

    pub async fn futures<C>(&self, ctx: &C) -> Result<Vec<types::MarketInstrument>, TError>
    where
        C: RequestId,
    {
        let client = self
            .instruments_service_client
            .as_ref()
            .ok_or(TError::InstrumentsServiceClientNotInit)?;
        let mut message = InstrumentsRequest::default();
        message.set_instrument_status(tinkoff_invest_types::InstrumentStatus::All);
        let request = Self::create_request(ctx, message)?;
        let mut client = client.clone();
        let futures = client.futures(request).await?.into_inner().instruments;
        let mut result = Vec::with_capacity(futures.len());
        for v in futures {
            result.push(v.into());
        }
        Ok(result)
    }

    pub async fn future<T, C>(
        &self,
        ctx: &C,
        instrument: T,
    ) -> Result<Option<types::MarketInstrument>, TError>
    where
        T: traits::ToInstrumentType + traits::ToFigi,
        C: RequestId,
    {
        if instrument.to_instrument_type() != enums::InstrumentType::Future {
            return Err(TError::MarketInstrumentTypeNotFuture);
        }
        let client = self
            .instruments_service_client
            .as_ref()
            .ok_or(TError::InstrumentsServiceClientNotInit)?;
        let mut message = InstrumentRequest {
            id: instrument.to_figi().into(),
            ..Default::default()
        };
        message.set_id_type(InstrumentIdType::Figi);
        let request = Self::create_request(ctx, message)?;
        let mut client = client.clone();
        let future = client.future_by(request).await?.into_inner().instrument;
        Ok(future.map(|x| x.into()))
    }

    pub async fn trading_status<T, C>(
        &self,
        ctx: &C,
        instrument: T,
    ) -> Result<enums::TradingStatus, TError>
    where
        T: traits::ToUid,
        C: RequestId,
    {
        let client = self
            .market_data_service_client
            .as_ref()
            .ok_or(TError::MarketDataServiceClientNotInit)?;
        let message = GetTradingStatusRequest {
            instrument_id: Some(instrument.to_uid().into()),
            ..Default::default()
        };
        let request = Self::create_request(ctx, message)?;
        let mut client = client.clone();
        Ok(client
            .get_trading_status(request)
            .await?
            .into_inner()
            .trading_status()
            .into())
    }

    pub async fn candlesticks<T, C>(
        &self,
        ctx: &C,
        instrument: T,
        interval: enums::CandlestickInterval,
        from: types::DateTime,
        to: types::DateTime,
    ) -> Result<Vec<types::Candlestick>, TError>
    where
        T: traits::ToUid,
        C: RequestId,
    {
        let uid = Arc::new(instrument.to_uid());
        let interval = Arc::new(interval);
        let mut message = GetCandlesRequest {
            instrument_id: Some((*uid).clone().into()),
            from: Some(from.into()),
            to: Some(to.into()),
            ..Default::default()
        };
        message.set_interval((*interval).clone().into());
        let client = self
            .market_data_service_client
            .as_ref()
            .ok_or(TError::MarketDataServiceClientNotInit)?;
        let request = Self::create_request(ctx, message)?;
        let mut client = client.clone();
        let candlesticks = client.get_candles(request).await?.into_inner().candles;
        let mut result = Vec::with_capacity(candlesticks.len());
        for x in candlesticks {
            if let Some(time) = x.time {
                result.push(types::Candlestick {
                    instrument_uid: Arc::clone(&uid),
                    interval: Arc::clone(&interval),
                    open: x.open.map(|v| v.into()),
                    high: x.high.map(|v| v.into()),
                    low: x.low.map(|v| v.into()),
                    close: x.close.map(|v| v.into()),
                    volume: x.volume as u64,
                    datetime: time.into(),
                    is_complete: x.is_complete,
                });
            }
        }
        Ok(result)
    }

    pub async fn orderbook<T, C>(
        &self,
        ctx: &C,
        instrument: T,
        depth: usize,
    ) -> Result<types::OrderBook, TError>
    where
        T: traits::ToUid,
        C: RequestId,
    {
        let message = GetOrderBookRequest {
            depth: depth as i32,
            instrument_id: Some(instrument.to_uid().into()),
            ..Default::default()
        };
        let client = self
            .market_data_service_client
            .as_ref()
            .ok_or(TError::MarketDataServiceClientNotInit)?;
        let request = Self::create_request(ctx, message)?;
        let mut client = client.clone();
        Ok(client.get_order_book(request).await?.into_inner().into())
    }

    pub async fn order<C>(&self, ctx: &C) -> Result<types::Order, TError>
    where
        C: RequestId + ToAccountIdRef + ToOrderIdRef,
    {
        let client = self
            .orders_service_client
            .as_ref()
            .ok_or(TError::OrdersServiceClientNotInit)?;
        let message = tinkoff_invest_types::GetOrderStateRequest {
            account_id: ctx.to_account_id_ref().into(),
            order_id: ctx.to_order_id_ref().into(),
            ..Default::default()
        };
        let request = Self::create_request(ctx, message)?;
        let mut client = client.clone();
        let order_state = client.get_order_state(request).await?.into_inner();
        Ok(types::Order::from(order_state))
    }

    #[inline]
    pub async fn operations<K, C>(
        &self,
        ctx: &C,
        instrument: K,
        state: enums::OperationState,
        from: types::DateTime,
        to: types::DateTime,
    ) -> Result<Vec<types::Operation>, TError>
    where
        K: traits::ToFigi,
        C: RequestId + ToAccountIdRef,
    {
        let from = Some(from.into());
        let to = Some(to.into());
        let client = self
            .operations_service_client
            .as_ref()
            .ok_or(TError::OperationsServiceClientNotInit)?;
        let mut message = OperationsRequest {
            account_id: ctx.to_account_id_ref().into(),
            figi: Some(instrument.to_figi().into()),
            state: Some(0),
            from,
            to,
        };
        message.set_state(state.into());
        let request = Self::create_request(ctx, message)?;
        let mut client = client.clone();
        let response = client.get_operations(request).await?;
        let operations = response.into_inner().operations;
        let mut result = Vec::with_capacity(operations.len());
        for x in operations {
            result.push(x.into());
        }
        Ok(result)
    }

    pub async fn portfolio<C>(&self, ctx: &C) -> Result<Vec<types::PortfolioPosition>, TError>
    where
        C: RequestId + ToAccountIdRef,
    {
        let mut message = PortfolioRequest {
            account_id: ctx.to_account_id_ref().into(),
            ..Default::default()
        };
        message.set_currency(CurrencyRequest::Rub);
        let client = self
            .operations_service_client
            .as_ref()
            .ok_or(TError::OperationsServiceClientNotInit)?;
        let request = Self::create_request(ctx, message)?;
        let mut client = client.clone();
        let positions = client.get_portfolio(request).await?.into_inner().positions;
        let mut result = Vec::with_capacity(positions.len());
        for x in &positions {
            result.push(x.into());
        }
        Ok(result)
    }

    pub async fn positions<C>(&self, ctx: &C) -> Result<types::Positions, TError>
    where
        C: RequestId + ToAccountIdRef,
    {
        let message = PositionsRequest {
            account_id: ctx.to_account_id_ref().into(),
        };
        let client = self
            .operations_service_client
            .as_ref()
            .ok_or(TError::OperationsServiceClientNotInit)?;
        let request = Self::create_request(ctx, message)?;
        let mut client = client.clone();
        let response = client.get_positions(request).await?;
        let positions = response.into_inner().into();
        Ok(positions)
    }

    #[inline]
    pub async fn limit_order<C>(
        &self,
        ctx: &C,
        instrument: impl traits::ToUid,
        direction: enums::OrderDirection,
        quantity: u64,
        price: types::MoneyValue,
    ) -> Result<types::Order, TError>
    where
        C: RequestId + ToAccountIdRef + ToOrderIdRef,
    {
        let mut message = PostOrderRequest {
            order_id: ctx.to_order_id_ref().into(),
            account_id: ctx.to_account_id_ref().into(),
            instrument_id: instrument.to_uid().into(),
            quantity: quantity as i64,
            price: Some(price.into()),
            ..Default::default()
        };
        message.set_direction(direction.into());
        message.set_order_type(tinkoff_invest_types::OrderType::Limit);
        let client = self
            .orders_service_client
            .as_ref()
            .ok_or(TError::OrdersServiceClientNotInit)?;
        let request = Self::create_request(ctx, message)?;
        let mut client = client.clone();
        let response = client.post_order(request).await?;
        let order = response.into_inner().into();
        Ok(order)
    }

    #[inline]
    pub async fn cancel_order<C>(&self, ctx: &C) -> Result<Option<types::DateTime>, TError>
    where
        C: RequestId + ToAccountIdRef + ToOrderIdRef,
    {
        let mut message = CancelOrderRequest {
            account_id: ctx.to_account_id_ref().into(),
            order_id: ctx.to_order_id_ref().into(),
            ..Default::default()
        };
        message.set_order_id_type(OrderIdType::Exchange);
        let client = self
            .orders_service_client
            .as_ref()
            .ok_or(TError::OrdersServiceClientNotInit)?;
        let request = Self::create_request(ctx, message)?;
        let mut client = client.clone();
        let response = client.cancel_order(request).await?;
        Ok(response.into_inner().time.map(|x| x.into()))
    }
}
