use std::error::Error;
use std::time::Duration;
use uuid::Uuid;

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
            .tls_config(ClientTlsConfig::new().with_native_roots())
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
            client = client.max_decoding_message_size(256 * 1024 * 1024);
            Some(client)
        } else {
            None
        };
        let instruments_service_client = if self.enable_instruments_service_client {
            let mut client =
                InstrumentsServiceClient::with_interceptor(channel.clone(), interceptor.clone());
            client = client.send_compressed(CompressionEncoding::Gzip);
            client = client.accept_compressed(CompressionEncoding::Gzip);
            client = client.max_decoding_message_size(256 * 1024 * 1024);
            Some(client)
        } else {
            None
        };
        let market_data_service_client = if self.enable_market_data_service_client {
            let mut client =
                MarketDataServiceClient::with_interceptor(channel.clone(), interceptor.clone());
            client = client.send_compressed(CompressionEncoding::Gzip);
            client = client.accept_compressed(CompressionEncoding::Gzip);
            client = client.max_decoding_message_size(256 * 1024 * 1024);
            Some(client)
        } else {
            None
        };
        let operations_service_client = if self.enable_operations_service_client {
            let mut client =
                OperationsServiceClient::with_interceptor(channel.clone(), interceptor.clone());
            client = client.send_compressed(CompressionEncoding::Gzip);
            client = client.accept_compressed(CompressionEncoding::Gzip);
            client = client.max_decoding_message_size(256 * 1024 * 1024);
            Some(client)
        } else {
            None
        };
        let orders_service_client = if self.enable_orders_service_client {
            let mut client =
                OrdersServiceClient::with_interceptor(channel.clone(), interceptor.clone());
            client = client.send_compressed(CompressionEncoding::Gzip);
            client = client.accept_compressed(CompressionEncoding::Gzip);
            client = client.max_decoding_message_size(256 * 1024 * 1024);
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

    /// Создает Request с установленным x-tracking-id из TinkoffInvestCallContext
    fn create_request_with_context<T>(
        message: T,
        ctx: &TinkoffInvestCallContext,
    ) -> TonicRequest<T> {
        let mut request = TonicRequest::new(message);
        request
            .metadata_mut()
            .insert("x-tracking-id", ctx.request_id.parse().unwrap());
        request
    }

    pub async fn accounts(
        &mut self,
        ctx: Option<&TinkoffInvestCallContext>,
    ) -> Result<Vec<types::Account>, Box<dyn Error>> {
        let client = self
            .users_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::UsersServiceClientNotInit)?;
        let ctx = ctx
            .map(|c| c.clone())
            .unwrap_or_else(|| TinkoffInvestCallContext::new(None));
        let message = GetAccountsRequest {
            ..Default::default()
        };
        let request = Self::create_request_with_context(message, &ctx);
        let accounts = client.get_accounts(request).await?.into_inner().accounts;
        Ok(accounts.iter().map(|v| v.clone().into()).collect())
    }

    pub async fn market_instruments(
        &mut self,
        ctx: Option<&TinkoffInvestCallContext>,
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
        ctx: Option<&TinkoffInvestCallContext>,
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
        ctx: Option<&TinkoffInvestCallContext>,
    ) -> Result<Vec<types::MarketInstrument>, Box<dyn Error>> {
        let client = self
            .instruments_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::InstrumentsServiceClientNotInit)?;
        let ctx = ctx
            .map(|c| c.clone())
            .unwrap_or_else(|| TinkoffInvestCallContext::new(None));
        let mut message = InstrumentsRequest::default();
        message.set_instrument_status(tinkoff_invest_types::InstrumentStatus::All);
        let request = Self::create_request_with_context(message, &ctx);
        let shares = client.shares(request).await?.into_inner().instruments;
        Ok(shares.into_iter().map(|x| x.into()).collect())
    }

    pub async fn share<T>(
        &mut self,
        ctx: Option<&TinkoffInvestCallContext>,
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
        let ctx = ctx
            .map(|c| c.clone())
            .unwrap_or_else(|| TinkoffInvestCallContext::new(None));
        let request = Self::create_request_with_context(message, &ctx);
        let share = client.share_by(request).await?.into_inner().instrument;
        Ok(share.as_ref().map(|x| x.clone().into()))
    }

    pub async fn currencies(
        &mut self,
        ctx: Option<&TinkoffInvestCallContext>,
    ) -> Result<Vec<types::MarketInstrument>, Box<dyn Error>> {
        let client = self
            .instruments_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::InstrumentsServiceClientNotInit)?;
        let mut message = InstrumentsRequest::default();
        message.set_instrument_status(tinkoff_invest_types::InstrumentStatus::All);
        let ctx = ctx
            .map(|c| c.clone())
            .unwrap_or_else(|| TinkoffInvestCallContext::new(None));
        let request = Self::create_request_with_context(message, &ctx);
        let currencies = client.currencies(request).await?.into_inner().instruments;
        Ok(currencies.into_iter().map(|v| v.into()).collect())
    }

    pub async fn currency<T>(
        &mut self,
        ctx: Option<&TinkoffInvestCallContext>,
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
        let ctx = ctx
            .map(|c| c.clone())
            .unwrap_or_else(|| TinkoffInvestCallContext::new(None));
        let request = Self::create_request_with_context(message, &ctx);
        let currency = client.currency_by(request).await?.into_inner().instrument;
        Ok(currency.as_ref().map(|x| x.clone().into()))
    }

    pub async fn futures(
        &mut self,
        ctx: Option<&TinkoffInvestCallContext>,
    ) -> Result<Vec<types::MarketInstrument>, Box<dyn Error>> {
        let client = self
            .instruments_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::InstrumentsServiceClientNotInit)?;
        let mut message = InstrumentsRequest::default();
        message.set_instrument_status(tinkoff_invest_types::InstrumentStatus::All);
        let ctx = ctx
            .map(|c| c.clone())
            .unwrap_or_else(|| TinkoffInvestCallContext::new(None));
        let request = Self::create_request_with_context(message, &ctx);
        let futures = client.futures(request).await?.into_inner().instruments;
        Ok(futures.into_iter().map(|v| v.into()).collect())
    }

    pub async fn future<T>(
        &mut self,
        ctx: Option<&TinkoffInvestCallContext>,
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
        let ctx = ctx
            .map(|c| c.clone())
            .unwrap_or_else(|| TinkoffInvestCallContext::new(None));
        let request = Self::create_request_with_context(message, &ctx);
        let future = client.future_by(request).await?.into_inner().instrument;
        Ok(future.as_ref().map(|x| x.clone().into()))
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
        ctx: Option<&TinkoffInvestCallContext>,
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
        let ctx = ctx
            .map(|c| c.clone())
            .unwrap_or_else(|| TinkoffInvestCallContext::new(None));
        let request = Self::create_request_with_context(message, &ctx);
        Ok(client
            .get_trading_status(request)
            .await?
            .into_inner()
            .trading_status()
            .into())
    }

    pub async fn candlesticks<T>(
        &mut self,
        ctx: Option<&TinkoffInvestCallContext>,
        instrument: T,
        interval: enums::CandlestickInterval,
        from: types::DateTime,
        to: types::DateTime,
    ) -> Result<Vec<types::Candlestick>, Box<dyn Error>>
    where
        T: traits::ToUid,
    {
        let uid = instrument.to_uid();
        let mut message = GetCandlesRequest {
            instrument_id: Some(uid.clone().into()),
            from: Some(from.into()),
            to: Some(to.into()),
            ..Default::default()
        };
        message.set_interval(interval.clone().into());
        let client = self
            .market_data_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::MarketDataServiceClientNotInit)?;
        let ctx = ctx
            .map(|c| c.clone())
            .unwrap_or_else(|| TinkoffInvestCallContext::new(None));
        let request = Self::create_request_with_context(message, &ctx);
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
        ctx: Option<&TinkoffInvestCallContext>,
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
        let ctx = ctx
            .map(|c| c.clone())
            .unwrap_or_else(|| TinkoffInvestCallContext::new(None));
        let request = Self::create_request_with_context(message, &ctx);
        Ok(client.get_order_book(request).await?.into_inner().into())
    }

    pub async fn order_on_account<T>(
        &mut self,
        ctx: Option<&TinkoffInvestCallContext>,
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
        let message = tinkoff_invest_types::GetOrderStateRequest {
            account_id: account.to_account_id().into(),
            order_id: order_id.into(),
            ..Default::default()
        };
        let ctx = ctx
            .map(|c| c.clone())
            .unwrap_or_else(|| TinkoffInvestCallContext::new(None));
        let request = Self::create_request_with_context(message, &ctx);
        let order_state = client.get_order_state(request).await?.into_inner();
        Ok(types::Order::from(order_state))
    }

    pub async fn order<T>(
        &mut self,
        ctx: Option<&TinkoffInvestCallContext>,
        order_id: types::OrderId,
    ) -> Result<types::Order, Box<dyn Error>> {
        let account = self
            .account
            .as_ref()
            .ok_or(TinkoffInvestError::AccountNotSet)?
            .clone();
        self.order_on_account(ctx, &account, order_id).await
    }

    #[inline]
    pub async fn operations_on_account<T, K>(
        &mut self,
        ctx: Option<&TinkoffInvestCallContext>,
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
        let mut message = OperationsRequest {
            account_id: account.to_account_id().into(),
            figi: Some(instrument.to_figi().into()),
            state: Some(0),
            from,
            to,
        };
        message.set_state(state.into());
        let ctx = ctx
            .map(|c| c.clone())
            .unwrap_or_else(|| TinkoffInvestCallContext::new(None));
        let request = Self::create_request_with_context(message, &ctx);
        let operations = client
            .get_operations(request)
            .await?
            .into_inner()
            .operations;
        Ok(operations.into_iter().map(|x| x.into()).collect())
    }

    pub async fn operations<T>(
        &mut self,
        ctx: Option<&TinkoffInvestCallContext>,
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
        self.operations_on_account(ctx, &account, instrument, state, from, to)
            .await
    }

    pub async fn portfolio_on_account<T>(
        &mut self,
        ctx: Option<&TinkoffInvestCallContext>,
        account: T,
    ) -> Result<Vec<types::PortfolioPosition>, Box<dyn Error>>
    where
        T: traits::ToAccountId,
    {
        let mut message = PortfolioRequest {
            account_id: account.to_account_id().into(),
            ..Default::default()
        };
        message.set_currency(CurrencyRequest::Rub);
        let client = self
            .operations_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::OperationsServiceClientNotInit)?;
        let ctx = ctx
            .map(|c| c.clone())
            .unwrap_or_else(|| TinkoffInvestCallContext::new(None));
        let request = Self::create_request_with_context(message, &ctx);
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

    pub async fn portfolio(
        &mut self,
        ctx: Option<&TinkoffInvestCallContext>,
    ) -> Result<Vec<types::PortfolioPosition>, Box<dyn Error>> {
        let account = self
            .account
            .as_ref()
            .ok_or(TinkoffInvestError::AccountNotSet)?
            .clone();
        self.portfolio_on_account(ctx, &account).await
    }

    pub async fn positions_on_account<T>(
        &mut self,
        ctx: Option<&TinkoffInvestCallContext>,
        account: T,
    ) -> Result<types::Positions, Box<dyn Error>>
    where
        T: traits::ToAccountId,
    {
        let message = PositionsRequest {
            account_id: account.to_account_id().into(),
        };
        let client = self
            .operations_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::OperationsServiceClientNotInit)?;
        let ctx = ctx
            .map(|c| c.clone())
            .unwrap_or_else(|| TinkoffInvestCallContext::new(None));
        let request = Self::create_request_with_context(message, &ctx);
        let response = client.get_positions(request).await?;
        let positions = response.into_inner().into();
        Ok(positions)
    }

    pub async fn positions(
        &mut self,
        ctx: Option<&TinkoffInvestCallContext>,
    ) -> Result<types::Positions, Box<dyn Error>> {
        let account = self
            .account
            .as_ref()
            .ok_or(TinkoffInvestError::AccountNotSet)?
            .clone();
        self.positions_on_account(ctx, &account).await
    }

    #[inline]
    pub async fn limit_order_on_account<T, K>(
        &mut self,
        ctx: Option<&TinkoffInvestCallContext>,
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
        let order_id = order_id.unwrap_or_else(|| Uuid::now_v7().to_string());
        let mut message = PostOrderRequest {
            order_id,
            account_id: account.to_account_id().into(),
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
        let ctx = ctx
            .map(|c| c.clone())
            .unwrap_or_else(|| TinkoffInvestCallContext::new(None));
        let request = Self::create_request_with_context(message, &ctx);
        Ok(client.post_order(request).await?.into_inner().into())
    }

    pub async fn limit_order<T>(
        &mut self,
        ctx: Option<&TinkoffInvestCallContext>,
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
        self.limit_order_on_account(
            ctx, &account, instrument, direction, quantity, price, order_id,
        )
        .await
    }

    #[inline]
    pub async fn cancel_order_on_account<T, K>(
        &mut self,
        ctx: Option<&TinkoffInvestCallContext>,
        account: T,
        order: K,
    ) -> Result<Option<types::DateTime>, Box<dyn Error>>
    where
        T: traits::ToAccountId,
        K: traits::ToOrderId,
    {
        let mut message = CancelOrderRequest {
            account_id: account.to_account_id().into(),
            order_id: order.to_order_id().into(),
            ..Default::default()
        };
        message.set_order_id_type(OrderIdType::Exchange);
        let client = self
            .orders_service_client
            .as_mut()
            .ok_or(TinkoffInvestError::OrdersServiceClientNotInit)?;
        let ctx = ctx
            .map(|c| c.clone())
            .unwrap_or_else(|| TinkoffInvestCallContext::new(None));
        let request = Self::create_request_with_context(message, &ctx);
        Ok(client
            .cancel_order(request)
            .await?
            .into_inner()
            .time
            .map(|x| x.into()))
    }

    pub async fn cancel_order<T>(
        &mut self,
        ctx: Option<&TinkoffInvestCallContext>,
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
        self.cancel_order_on_account(ctx, &account, order).await
    }
}
