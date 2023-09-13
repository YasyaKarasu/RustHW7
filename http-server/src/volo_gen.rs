pub mod volo_gen {
    #![allow(warnings, clippy::all)]

    pub mod mini {

        pub mod redis {

            #[::async_trait::async_trait]
            pub trait MiniRedisService {
                async fn get_value(
                    &self,
                    request: GetValueRequest,
                ) -> ::core::result::Result<GetValueResponse, ::volo_thrift::AnyhowError>;
                async fn set_value(
                    &self,
                    request: SetValueRequest,
                ) -> ::core::result::Result<SetValueResponse, ::volo_thrift::AnyhowError>;
                async fn delete_value(
                    &self,
                    request: DeleteValueRequest,
                ) -> ::core::result::Result<DeleteValueResponse, ::volo_thrift::AnyhowError>;
                async fn ping(
                    &self,
                ) -> ::core::result::Result<PingResponse, ::volo_thrift::AnyhowError>;
            }
            pub struct MiniRedisServiceServer<S> {
                inner: S, // handler
            }

            pub struct MkMiniRedisServiceGenericClient;

            pub type MiniRedisServiceClient = MiniRedisServiceGenericClient<
                ::volo::service::BoxCloneService<
                    ::volo_thrift::context::ClientContext,
                    MiniRedisServiceRequestSend,
                    ::std::option::Option<MiniRedisServiceResponseRecv>,
                    ::volo_thrift::Error,
                >,
            >;

            impl<S> ::volo::client::MkClient<::volo_thrift::Client<S>> for MkMiniRedisServiceGenericClient {
                type Target = MiniRedisServiceGenericClient<S>;
                fn mk_client(&self, service: ::volo_thrift::Client<S>) -> Self::Target {
                    MiniRedisServiceGenericClient(service)
                }
            }

            #[derive(Clone)]
            pub struct MiniRedisServiceGenericClient<S>(pub ::volo_thrift::Client<S>);

            pub struct MiniRedisServiceOneShotClient<S>(pub ::volo_thrift::Client<S>);

            impl<
                    S: ::volo::service::Service<
                            ::volo_thrift::context::ClientContext,
                            MiniRedisServiceRequestSend,
                            Response = ::std::option::Option<MiniRedisServiceResponseRecv>,
                            Error = ::volo_thrift::Error,
                        > + Send
                        + Sync
                        + 'static,
                > MiniRedisServiceGenericClient<S>
            {
                pub fn with_callopt<
                    Opt: ::volo::client::Apply<::volo_thrift::context::ClientContext>,
                >(
                    self,
                    opt: Opt,
                ) -> MiniRedisServiceOneShotClient<::volo::client::WithOptService<S, Opt>>
                {
                    MiniRedisServiceOneShotClient(self.0.with_opt(opt))
                }

                pub async fn get_value(
                    &self,
                    request: GetValueRequest,
                ) -> ::std::result::Result<
                    GetValueResponse,
                    ::volo_thrift::error::ResponseError<std::convert::Infallible>,
                > {
                    let req =
                        MiniRedisServiceRequestSend::GetValue(MiniRedisServiceGetValueArgsSend {
                            request,
                        });
                    let mut cx = self.0.make_cx("GetValue", false);
                    #[allow(unreachable_patterns)]
                    let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
                        Some(MiniRedisServiceResponseRecv::GetValue(
                            MiniRedisServiceGetValueResultRecv::Ok(resp),
                        )) => Ok(resp),
                        None => unreachable!(),
                        _ => unreachable!(),
                    };
                    ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
                        let mut cache = cache.borrow_mut();
                        if cache.len() < cache.capacity() {
                            cache.push(cx);
                        }
                    });
                    resp
                }
                pub async fn set_value(
                    &self,
                    request: SetValueRequest,
                ) -> ::std::result::Result<
                    SetValueResponse,
                    ::volo_thrift::error::ResponseError<std::convert::Infallible>,
                > {
                    let req =
                        MiniRedisServiceRequestSend::SetValue(MiniRedisServiceSetValueArgsSend {
                            request,
                        });
                    let mut cx = self.0.make_cx("SetValue", false);
                    #[allow(unreachable_patterns)]
                    let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
                        Some(MiniRedisServiceResponseRecv::SetValue(
                            MiniRedisServiceSetValueResultRecv::Ok(resp),
                        )) => Ok(resp),
                        None => unreachable!(),
                        _ => unreachable!(),
                    };
                    ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
                        let mut cache = cache.borrow_mut();
                        if cache.len() < cache.capacity() {
                            cache.push(cx);
                        }
                    });
                    resp
                }
                pub async fn delete_value(
                    &self,
                    request: DeleteValueRequest,
                ) -> ::std::result::Result<
                    DeleteValueResponse,
                    ::volo_thrift::error::ResponseError<std::convert::Infallible>,
                > {
                    let req = MiniRedisServiceRequestSend::DeleteValue(
                        MiniRedisServiceDeleteValueArgsSend { request },
                    );
                    let mut cx = self.0.make_cx("DeleteValue", false);
                    #[allow(unreachable_patterns)]
                    let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
                        Some(MiniRedisServiceResponseRecv::DeleteValue(
                            MiniRedisServiceDeleteValueResultRecv::Ok(resp),
                        )) => Ok(resp),
                        None => unreachable!(),
                        _ => unreachable!(),
                    };
                    ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
                        let mut cache = cache.borrow_mut();
                        if cache.len() < cache.capacity() {
                            cache.push(cx);
                        }
                    });
                    resp
                }
                pub async fn ping(
                    &self,
                ) -> ::std::result::Result<
                    PingResponse,
                    ::volo_thrift::error::ResponseError<std::convert::Infallible>,
                > {
                    let req = MiniRedisServiceRequestSend::Ping(MiniRedisServicePingArgsSend {});
                    let mut cx = self.0.make_cx("Ping", false);
                    #[allow(unreachable_patterns)]
                    let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
                        Some(MiniRedisServiceResponseRecv::Ping(
                            MiniRedisServicePingResultRecv::Ok(resp),
                        )) => Ok(resp),
                        None => unreachable!(),
                        _ => unreachable!(),
                    };
                    ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
                        let mut cache = cache.borrow_mut();
                        if cache.len() < cache.capacity() {
                            cache.push(cx);
                        }
                    });
                    resp
                }
            }

            impl<
                    S: ::volo::client::OneShotService<
                            ::volo_thrift::context::ClientContext,
                            MiniRedisServiceRequestSend,
                            Response = ::std::option::Option<MiniRedisServiceResponseRecv>,
                            Error = ::volo_thrift::Error,
                        > + Send
                        + Sync
                        + 'static,
                > MiniRedisServiceOneShotClient<S>
            {
                pub async fn get_value(
                    self,
                    request: GetValueRequest,
                ) -> ::std::result::Result<
                    GetValueResponse,
                    ::volo_thrift::error::ResponseError<std::convert::Infallible>,
                > {
                    let req =
                        MiniRedisServiceRequestSend::GetValue(MiniRedisServiceGetValueArgsSend {
                            request,
                        });
                    let mut cx = self.0.make_cx("GetValue", false);
                    #[allow(unreachable_patterns)]
                    let resp =
                        match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
                            Some(MiniRedisServiceResponseRecv::GetValue(
                                MiniRedisServiceGetValueResultRecv::Ok(resp),
                            )) => Ok(resp),
                            None => unreachable!(),
                            _ => unreachable!(),
                        };
                    ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
                        let mut cache = cache.borrow_mut();
                        if cache.len() < cache.capacity() {
                            cache.push(cx);
                        }
                    });
                    resp
                }
                pub async fn set_value(
                    self,
                    request: SetValueRequest,
                ) -> ::std::result::Result<
                    SetValueResponse,
                    ::volo_thrift::error::ResponseError<std::convert::Infallible>,
                > {
                    let req =
                        MiniRedisServiceRequestSend::SetValue(MiniRedisServiceSetValueArgsSend {
                            request,
                        });
                    let mut cx = self.0.make_cx("SetValue", false);
                    #[allow(unreachable_patterns)]
                    let resp =
                        match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
                            Some(MiniRedisServiceResponseRecv::SetValue(
                                MiniRedisServiceSetValueResultRecv::Ok(resp),
                            )) => Ok(resp),
                            None => unreachable!(),
                            _ => unreachable!(),
                        };
                    ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
                        let mut cache = cache.borrow_mut();
                        if cache.len() < cache.capacity() {
                            cache.push(cx);
                        }
                    });
                    resp
                }
                pub async fn delete_value(
                    self,
                    request: DeleteValueRequest,
                ) -> ::std::result::Result<
                    DeleteValueResponse,
                    ::volo_thrift::error::ResponseError<std::convert::Infallible>,
                > {
                    let req = MiniRedisServiceRequestSend::DeleteValue(
                        MiniRedisServiceDeleteValueArgsSend { request },
                    );
                    let mut cx = self.0.make_cx("DeleteValue", false);
                    #[allow(unreachable_patterns)]
                    let resp =
                        match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
                            Some(MiniRedisServiceResponseRecv::DeleteValue(
                                MiniRedisServiceDeleteValueResultRecv::Ok(resp),
                            )) => Ok(resp),
                            None => unreachable!(),
                            _ => unreachable!(),
                        };
                    ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
                        let mut cache = cache.borrow_mut();
                        if cache.len() < cache.capacity() {
                            cache.push(cx);
                        }
                    });
                    resp
                }
                pub async fn ping(
                    self,
                ) -> ::std::result::Result<
                    PingResponse,
                    ::volo_thrift::error::ResponseError<std::convert::Infallible>,
                > {
                    let req = MiniRedisServiceRequestSend::Ping(MiniRedisServicePingArgsSend {});
                    let mut cx = self.0.make_cx("Ping", false);
                    #[allow(unreachable_patterns)]
                    let resp =
                        match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
                            Some(MiniRedisServiceResponseRecv::Ping(
                                MiniRedisServicePingResultRecv::Ok(resp),
                            )) => Ok(resp),
                            None => unreachable!(),
                            _ => unreachable!(),
                        };
                    ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
                        let mut cache = cache.borrow_mut();
                        if cache.len() < cache.capacity() {
                            cache.push(cx);
                        }
                    });
                    resp
                }
            }

            pub struct MiniRedisServiceClientBuilder {}

            impl MiniRedisServiceClientBuilder {
                pub fn new(
                    service_name: impl AsRef<str>,
                ) -> ::volo_thrift::client::ClientBuilder<
                    ::volo::layer::Identity,
                    ::volo::layer::Identity,
                    MkMiniRedisServiceGenericClient,
                    MiniRedisServiceRequestSend,
                    MiniRedisServiceResponseRecv,
                    ::volo::net::dial::DefaultMakeTransport,
                    ::volo_thrift::codec::default::DefaultMakeCodec<
                        ::volo_thrift::codec::default::ttheader::MakeTTHeaderCodec<
                            ::volo_thrift::codec::default::framed::MakeFramedCodec<
                                ::volo_thrift::codec::default::thrift::MakeThriftCodec,
                            >,
                        >,
                    >,
                    ::volo::loadbalance::LbConfig<
                        ::volo::loadbalance::random::WeightedRandomBalance<()>,
                        ::volo::discovery::DummyDiscover,
                    >,
                > {
                    ::volo_thrift::client::ClientBuilder::new(
                        service_name,
                        MkMiniRedisServiceGenericClient,
                    )
                }
            }

            impl<S> MiniRedisServiceServer<S>
            where
                S: MiniRedisService + ::core::marker::Send + ::core::marker::Sync + 'static,
            {
                pub fn new(
                    inner: S,
                ) -> ::volo_thrift::server::Server<
                    Self,
                    ::volo::layer::Identity,
                    MiniRedisServiceRequestRecv,
                    ::volo_thrift::codec::default::DefaultMakeCodec<
                        ::volo_thrift::codec::default::ttheader::MakeTTHeaderCodec<
                            ::volo_thrift::codec::default::framed::MakeFramedCodec<
                                ::volo_thrift::codec::default::thrift::MakeThriftCodec,
                            >,
                        >,
                    >,
                    ::volo_thrift::tracing::DefaultProvider,
                > {
                    ::volo_thrift::server::Server::new(Self { inner })
                }
            }

            impl<T>
                ::volo::service::Service<
                    ::volo_thrift::context::ServerContext,
                    MiniRedisServiceRequestRecv,
                > for MiniRedisServiceServer<T>
            where
                T: MiniRedisService + Send + Sync + 'static,
            {
                type Response = MiniRedisServiceResponseSend;
                type Error = ::anyhow::Error;

                type Future<'cx> = impl ::std::future::Future<
                        Output = ::std::result::Result<Self::Response, Self::Error>,
                    > + 'cx;

                fn call<'cx, 's>(
                    &'s self,
                    _cx: &'cx mut ::volo_thrift::context::ServerContext,
                    req: MiniRedisServiceRequestRecv,
                ) -> Self::Future<'cx>
                where
                    's: 'cx,
                {
                    async move {
                        match req {
                            MiniRedisServiceRequestRecv::GetValue(args) => {
                                Ok(MiniRedisServiceResponseSend::GetValue(
                                    match self.inner.get_value(args.request).await {
                                        Ok(resp) => MiniRedisServiceGetValueResultSend::Ok(resp),
                                        Err(err) => return Err(err),
                                    },
                                ))
                            }
                            MiniRedisServiceRequestRecv::SetValue(args) => {
                                Ok(MiniRedisServiceResponseSend::SetValue(
                                    match self.inner.set_value(args.request).await {
                                        Ok(resp) => MiniRedisServiceSetValueResultSend::Ok(resp),
                                        Err(err) => return Err(err),
                                    },
                                ))
                            }
                            MiniRedisServiceRequestRecv::DeleteValue(args) => {
                                Ok(MiniRedisServiceResponseSend::DeleteValue(
                                    match self.inner.delete_value(args.request).await {
                                        Ok(resp) => MiniRedisServiceDeleteValueResultSend::Ok(resp),
                                        Err(err) => return Err(err),
                                    },
                                ))
                            }
                            MiniRedisServiceRequestRecv::Ping(args) => Ok(
                                MiniRedisServiceResponseSend::Ping(match self.inner.ping().await {
                                    Ok(resp) => MiniRedisServicePingResultSend::Ok(resp),
                                    Err(err) => return Err(err),
                                }),
                            ),
                        }
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub enum MiniRedisServiceRequestRecv {
                GetValue(MiniRedisServiceGetValueArgsRecv),
                SetValue(MiniRedisServiceSetValueArgsRecv),
                DeleteValue(MiniRedisServiceDeleteValueArgsRecv),
                Ping(MiniRedisServicePingArgsRecv),
            }

            #[derive(Debug, Clone)]
            pub enum MiniRedisServiceRequestSend {
                GetValue(MiniRedisServiceGetValueArgsSend),
                SetValue(MiniRedisServiceSetValueArgsSend),
                DeleteValue(MiniRedisServiceDeleteValueArgsSend),
                Ping(MiniRedisServicePingArgsSend),
            }

            #[derive(Debug, Clone)]
            pub enum MiniRedisServiceResponseRecv {
                GetValue(MiniRedisServiceGetValueResultRecv),
                SetValue(MiniRedisServiceSetValueResultRecv),
                DeleteValue(MiniRedisServiceDeleteValueResultRecv),
                Ping(MiniRedisServicePingResultRecv),
            }

            #[derive(Debug, Clone)]
            pub enum MiniRedisServiceResponseSend {
                GetValue(MiniRedisServiceGetValueResultSend),
                SetValue(MiniRedisServiceSetValueResultSend),
                DeleteValue(MiniRedisServiceDeleteValueResultSend),
                Ping(MiniRedisServicePingResultSend),
            }

            #[::async_trait::async_trait]
            impl ::volo_thrift::EntryMessage for MiniRedisServiceRequestRecv {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::core::result::Result<(), ::pilota::thrift::EncodeError> {
                    match self {
                        Self::GetValue(value) => ::pilota::thrift::Message::encode(value, protocol)
                            .map_err(|err| err.into()),
                        Self::SetValue(value) => ::pilota::thrift::Message::encode(value, protocol)
                            .map_err(|err| err.into()),
                        Self::DeleteValue(value) => {
                            ::pilota::thrift::Message::encode(value, protocol)
                                .map_err(|err| err.into())
                        }
                        Self::Ping(value) => ::pilota::thrift::Message::encode(value, protocol)
                            .map_err(|err| err.into()),
                    }
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                    msg_ident: &::pilota::thrift::TMessageIdentifier,
                ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                    Ok(match &*msg_ident.name {
                        "GetValue" => Self::GetValue(::pilota::thrift::Message::decode(protocol)?),
                        "SetValue" => Self::SetValue(::pilota::thrift::Message::decode(protocol)?),
                        "DeleteValue" => {
                            Self::DeleteValue(::pilota::thrift::Message::decode(protocol)?)
                        }
                        "Ping" => Self::Ping(::pilota::thrift::Message::decode(protocol)?),
                        _ => {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                                format!("unknown method {}", msg_ident.name),
                            ));
                        }
                    })
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                    msg_ident: &::pilota::thrift::TMessageIdentifier,
                ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                    Ok(match &*msg_ident.name {
                        "GetValue" => {
                            Self::GetValue(::pilota::thrift::Message::decode_async(protocol).await?)
                        }
                        "SetValue" => {
                            Self::SetValue(::pilota::thrift::Message::decode_async(protocol).await?)
                        }
                        "DeleteValue" => Self::DeleteValue(
                            ::pilota::thrift::Message::decode_async(protocol).await?,
                        ),
                        "Ping" => {
                            Self::Ping(::pilota::thrift::Message::decode_async(protocol).await?)
                        }
                        _ => {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                                format!("unknown method {}", msg_ident.name),
                            ));
                        }
                    })
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    match self {
                        Self::GetValue(value) => ::volo_thrift::Message::size(value, protocol),
                        Self::SetValue(value) => ::volo_thrift::Message::size(value, protocol),
                        Self::DeleteValue(value) => ::volo_thrift::Message::size(value, protocol),
                        Self::Ping(value) => ::volo_thrift::Message::size(value, protocol),
                    }
                }
            }

            #[::async_trait::async_trait]
            impl ::volo_thrift::EntryMessage for MiniRedisServiceRequestSend {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::core::result::Result<(), ::pilota::thrift::EncodeError> {
                    match self {
                        Self::GetValue(value) => ::pilota::thrift::Message::encode(value, protocol)
                            .map_err(|err| err.into()),
                        Self::SetValue(value) => ::pilota::thrift::Message::encode(value, protocol)
                            .map_err(|err| err.into()),
                        Self::DeleteValue(value) => {
                            ::pilota::thrift::Message::encode(value, protocol)
                                .map_err(|err| err.into())
                        }
                        Self::Ping(value) => ::pilota::thrift::Message::encode(value, protocol)
                            .map_err(|err| err.into()),
                    }
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                    msg_ident: &::pilota::thrift::TMessageIdentifier,
                ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                    Ok(match &*msg_ident.name {
                        "GetValue" => Self::GetValue(::pilota::thrift::Message::decode(protocol)?),
                        "SetValue" => Self::SetValue(::pilota::thrift::Message::decode(protocol)?),
                        "DeleteValue" => {
                            Self::DeleteValue(::pilota::thrift::Message::decode(protocol)?)
                        }
                        "Ping" => Self::Ping(::pilota::thrift::Message::decode(protocol)?),
                        _ => {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                                format!("unknown method {}", msg_ident.name),
                            ));
                        }
                    })
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                    msg_ident: &::pilota::thrift::TMessageIdentifier,
                ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                    Ok(match &*msg_ident.name {
                        "GetValue" => {
                            Self::GetValue(::pilota::thrift::Message::decode_async(protocol).await?)
                        }
                        "SetValue" => {
                            Self::SetValue(::pilota::thrift::Message::decode_async(protocol).await?)
                        }
                        "DeleteValue" => Self::DeleteValue(
                            ::pilota::thrift::Message::decode_async(protocol).await?,
                        ),
                        "Ping" => {
                            Self::Ping(::pilota::thrift::Message::decode_async(protocol).await?)
                        }
                        _ => {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                                format!("unknown method {}", msg_ident.name),
                            ));
                        }
                    })
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    match self {
                        Self::GetValue(value) => ::volo_thrift::Message::size(value, protocol),
                        Self::SetValue(value) => ::volo_thrift::Message::size(value, protocol),
                        Self::DeleteValue(value) => ::volo_thrift::Message::size(value, protocol),
                        Self::Ping(value) => ::volo_thrift::Message::size(value, protocol),
                    }
                }
            }
            #[::async_trait::async_trait]
            impl ::volo_thrift::EntryMessage for MiniRedisServiceResponseRecv {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::core::result::Result<(), ::pilota::thrift::EncodeError> {
                    match self {
                        Self::GetValue(value) => ::pilota::thrift::Message::encode(value, protocol)
                            .map_err(|err| err.into()),
                        Self::SetValue(value) => ::pilota::thrift::Message::encode(value, protocol)
                            .map_err(|err| err.into()),
                        Self::DeleteValue(value) => {
                            ::pilota::thrift::Message::encode(value, protocol)
                                .map_err(|err| err.into())
                        }
                        Self::Ping(value) => ::pilota::thrift::Message::encode(value, protocol)
                            .map_err(|err| err.into()),
                    }
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                    msg_ident: &::pilota::thrift::TMessageIdentifier,
                ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                    Ok(match &*msg_ident.name {
                        "GetValue" => Self::GetValue(::pilota::thrift::Message::decode(protocol)?),
                        "SetValue" => Self::SetValue(::pilota::thrift::Message::decode(protocol)?),
                        "DeleteValue" => {
                            Self::DeleteValue(::pilota::thrift::Message::decode(protocol)?)
                        }
                        "Ping" => Self::Ping(::pilota::thrift::Message::decode(protocol)?),
                        _ => {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                                format!("unknown method {}", msg_ident.name),
                            ));
                        }
                    })
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                    msg_ident: &::pilota::thrift::TMessageIdentifier,
                ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                    Ok(match &*msg_ident.name {
                        "GetValue" => {
                            Self::GetValue(::pilota::thrift::Message::decode_async(protocol).await?)
                        }
                        "SetValue" => {
                            Self::SetValue(::pilota::thrift::Message::decode_async(protocol).await?)
                        }
                        "DeleteValue" => Self::DeleteValue(
                            ::pilota::thrift::Message::decode_async(protocol).await?,
                        ),
                        "Ping" => {
                            Self::Ping(::pilota::thrift::Message::decode_async(protocol).await?)
                        }
                        _ => {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                                format!("unknown method {}", msg_ident.name),
                            ));
                        }
                    })
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    match self {
                        Self::GetValue(value) => ::volo_thrift::Message::size(value, protocol),
                        Self::SetValue(value) => ::volo_thrift::Message::size(value, protocol),
                        Self::DeleteValue(value) => ::volo_thrift::Message::size(value, protocol),
                        Self::Ping(value) => ::volo_thrift::Message::size(value, protocol),
                    }
                }
            }

            #[::async_trait::async_trait]
            impl ::volo_thrift::EntryMessage for MiniRedisServiceResponseSend {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::core::result::Result<(), ::pilota::thrift::EncodeError> {
                    match self {
                        Self::GetValue(value) => ::pilota::thrift::Message::encode(value, protocol)
                            .map_err(|err| err.into()),
                        Self::SetValue(value) => ::pilota::thrift::Message::encode(value, protocol)
                            .map_err(|err| err.into()),
                        Self::DeleteValue(value) => {
                            ::pilota::thrift::Message::encode(value, protocol)
                                .map_err(|err| err.into())
                        }
                        Self::Ping(value) => ::pilota::thrift::Message::encode(value, protocol)
                            .map_err(|err| err.into()),
                    }
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                    msg_ident: &::pilota::thrift::TMessageIdentifier,
                ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                    Ok(match &*msg_ident.name {
                        "GetValue" => Self::GetValue(::pilota::thrift::Message::decode(protocol)?),
                        "SetValue" => Self::SetValue(::pilota::thrift::Message::decode(protocol)?),
                        "DeleteValue" => {
                            Self::DeleteValue(::pilota::thrift::Message::decode(protocol)?)
                        }
                        "Ping" => Self::Ping(::pilota::thrift::Message::decode(protocol)?),
                        _ => {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                                format!("unknown method {}", msg_ident.name),
                            ));
                        }
                    })
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                    msg_ident: &::pilota::thrift::TMessageIdentifier,
                ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                    Ok(match &*msg_ident.name {
                        "GetValue" => {
                            Self::GetValue(::pilota::thrift::Message::decode_async(protocol).await?)
                        }
                        "SetValue" => {
                            Self::SetValue(::pilota::thrift::Message::decode_async(protocol).await?)
                        }
                        "DeleteValue" => Self::DeleteValue(
                            ::pilota::thrift::Message::decode_async(protocol).await?,
                        ),
                        "Ping" => {
                            Self::Ping(::pilota::thrift::Message::decode_async(protocol).await?)
                        }
                        _ => {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                                format!("unknown method {}", msg_ident.name),
                            ));
                        }
                    })
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    match self {
                        Self::GetValue(value) => ::volo_thrift::Message::size(value, protocol),
                        Self::SetValue(value) => ::volo_thrift::Message::size(value, protocol),
                        Self::DeleteValue(value) => ::volo_thrift::Message::size(value, protocol),
                        Self::Ping(value) => ::volo_thrift::Message::size(value, protocol),
                    }
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
            #[derivative(Default)]
            #[derive(Clone, PartialEq)]

            pub enum MiniRedisServicePingResultRecv {
                #[derivative(Default)]
                Ok(PingResponse),
            }

            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for MiniRedisServicePingResultRecv {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServicePingResultRecv",
                    })?;
                    match self {
                        MiniRedisServicePingResultRecv::Ok(ref value) => {
                            protocol.write_struct_field(
                                0,
                                value,
                                ::pilota::thrift::TType::Struct,
                            )?;
                        }
                    }
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};
                    let mut ret = None;
                    protocol.read_struct_begin()?;
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                    protocol.struct_len(&field_ident);
                                    ret = Some(MiniRedisServicePingResultRecv::Ok(field_ident));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new(
                                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }
                    }
                    protocol.read_field_end()?;
                    protocol.read_struct_end()?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut ret = None;
                    protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident =
                                        ::pilota::thrift::Message::decode_async(protocol).await?;

                                    ret = Some(MiniRedisServicePingResultRecv::Ok(field_ident));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new(
                                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    protocol.read_field_end().await?;
                    protocol.read_struct_end().await?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServicePingResultRecv",
                    }) + match self {
                        MiniRedisServicePingResultRecv::Ok(ref value) => {
                            protocol.struct_field_len(Some(0), value)
                        }
                    } + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
            #[derivative(Default)]
            #[derive(Clone, PartialEq)]

            pub enum MiniRedisServiceDeleteValueResultRecv {
                #[derivative(Default)]
                Ok(DeleteValueResponse),
            }

            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for MiniRedisServiceDeleteValueResultRecv {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceDeleteValueResultRecv",
                    })?;
                    match self {
                        MiniRedisServiceDeleteValueResultRecv::Ok(ref value) => {
                            protocol.write_struct_field(
                                0,
                                value,
                                ::pilota::thrift::TType::Struct,
                            )?;
                        }
                    }
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};
                    let mut ret = None;
                    protocol.read_struct_begin()?;
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                    protocol.struct_len(&field_ident);
                                    ret = Some(MiniRedisServiceDeleteValueResultRecv::Ok(
                                        field_ident,
                                    ));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new(
                                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }
                    }
                    protocol.read_field_end()?;
                    protocol.read_struct_end()?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut ret = None;
                    protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident =
                                        ::pilota::thrift::Message::decode_async(protocol).await?;

                                    ret = Some(MiniRedisServiceDeleteValueResultRecv::Ok(
                                        field_ident,
                                    ));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new(
                                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    protocol.read_field_end().await?;
                    protocol.read_struct_end().await?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceDeleteValueResultRecv",
                    }) + match self {
                        MiniRedisServiceDeleteValueResultRecv::Ok(ref value) => {
                            protocol.struct_field_len(Some(0), value)
                        }
                    } + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
            #[derivative(Default)]
            #[derive(Clone, PartialEq)]

            pub enum MiniRedisServiceSetValueResultRecv {
                #[derivative(Default)]
                Ok(SetValueResponse),
            }

            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for MiniRedisServiceSetValueResultRecv {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceSetValueResultRecv",
                    })?;
                    match self {
                        MiniRedisServiceSetValueResultRecv::Ok(ref value) => {
                            protocol.write_struct_field(
                                0,
                                value,
                                ::pilota::thrift::TType::Struct,
                            )?;
                        }
                    }
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};
                    let mut ret = None;
                    protocol.read_struct_begin()?;
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                    protocol.struct_len(&field_ident);
                                    ret = Some(MiniRedisServiceSetValueResultRecv::Ok(field_ident));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new(
                                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }
                    }
                    protocol.read_field_end()?;
                    protocol.read_struct_end()?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut ret = None;
                    protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident =
                                        ::pilota::thrift::Message::decode_async(protocol).await?;

                                    ret = Some(MiniRedisServiceSetValueResultRecv::Ok(field_ident));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new(
                                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    protocol.read_field_end().await?;
                    protocol.read_struct_end().await?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceSetValueResultRecv",
                    }) + match self {
                        MiniRedisServiceSetValueResultRecv::Ok(ref value) => {
                            protocol.struct_field_len(Some(0), value)
                        }
                    } + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
            #[derivative(Default)]
            #[derive(Clone, PartialEq)]

            pub enum MiniRedisServiceGetValueResultRecv {
                #[derivative(Default)]
                Ok(GetValueResponse),
            }

            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for MiniRedisServiceGetValueResultRecv {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceGetValueResultRecv",
                    })?;
                    match self {
                        MiniRedisServiceGetValueResultRecv::Ok(ref value) => {
                            protocol.write_struct_field(
                                0,
                                value,
                                ::pilota::thrift::TType::Struct,
                            )?;
                        }
                    }
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};
                    let mut ret = None;
                    protocol.read_struct_begin()?;
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                    protocol.struct_len(&field_ident);
                                    ret = Some(MiniRedisServiceGetValueResultRecv::Ok(field_ident));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new(
                                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }
                    }
                    protocol.read_field_end()?;
                    protocol.read_struct_end()?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut ret = None;
                    protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident =
                                        ::pilota::thrift::Message::decode_async(protocol).await?;

                                    ret = Some(MiniRedisServiceGetValueResultRecv::Ok(field_ident));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new(
                                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    protocol.read_field_end().await?;
                    protocol.read_struct_end().await?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceGetValueResultRecv",
                    }) + match self {
                        MiniRedisServiceGetValueResultRecv::Ok(ref value) => {
                            protocol.struct_field_len(Some(0), value)
                        }
                    } + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct DeleteValueRequest {
                pub key: ::pilota::FastStr,
            }
            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for DeleteValueRequest {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "DeleteValueRequest",
                    };

                    protocol.write_struct_begin(&struct_ident)?;
                    protocol.write_faststr_field(1, (&self.key).clone())?;
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};

                    let mut key = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin()?;
                    if let Err(err) = (|| {
                        loop {
                            let field_ident = protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                protocol.field_stop_len();
                                break;
                            } else {
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    key = Some(protocol.read_faststr()?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type)?;
                                }
                            }

                            protocol.read_field_end()?;
                            protocol.field_end_len();
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    })() {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `DeleteValueRequest` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end()?;

                    let Some(key) = key else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field key is required".to_string(),
                        ));
                    };

                    let data = Self { key };
                    Ok(data)
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut key = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    key = Some(protocol.read_faststr().await?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `DeleteValueRequest` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let Some(key) = key else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field key is required".to_string(),
                        ));
                    };

                    let data = Self { key };
                    Ok(data)
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "DeleteValueRequest",
                    }) + protocol.faststr_field_len(Some(1), &self.key)
                        + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct GetValueResponse {
                pub value: ::pilota::FastStr,

                pub error: ::pilota::FastStr,
            }
            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for GetValueResponse {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "GetValueResponse",
                    };

                    protocol.write_struct_begin(&struct_ident)?;
                    protocol.write_faststr_field(1, (&self.value).clone())?;
                    protocol.write_faststr_field(2, (&self.error).clone())?;
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};

                    let mut value = None;
                    let mut error = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin()?;
                    if let Err(err) = (|| {
                        loop {
                            let field_ident = protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                protocol.field_stop_len();
                                break;
                            } else {
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    value = Some(protocol.read_faststr()?);
                                }
                                Some(2)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    error = Some(protocol.read_faststr()?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type)?;
                                }
                            }

                            protocol.read_field_end()?;
                            protocol.field_end_len();
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    })() {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `GetValueResponse` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end()?;

                    let Some(value) = value else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field value is required".to_string(),
                        ));
                    };
                    let Some(error) = error else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field error is required".to_string(),
                        ));
                    };

                    let data = Self { value, error };
                    Ok(data)
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut value = None;
                    let mut error = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    value = Some(protocol.read_faststr().await?);
                                }
                                Some(2)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    error = Some(protocol.read_faststr().await?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `GetValueResponse` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let Some(value) = value else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field value is required".to_string(),
                        ));
                    };
                    let Some(error) = error else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field error is required".to_string(),
                        ));
                    };

                    let data = Self { value, error };
                    Ok(data)
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "GetValueResponse",
                    }) + protocol.faststr_field_len(Some(1), &self.value)
                        + protocol.faststr_field_len(Some(2), &self.error)
                        + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct MiniRedisServicePingArgsRecv {}
            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for MiniRedisServicePingArgsRecv {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServicePingArgsRecv",
                    };

                    protocol.write_struct_begin(&struct_ident)?;

                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin()?;
                    if let Err(err) = (|| {
                        loop {
                            let field_ident = protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                protocol.field_stop_len();
                                break;
                            } else {
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                _ => {
                                    protocol.skip(field_ident.field_type)?;
                                }
                            }

                            protocol.read_field_end()?;
                            protocol.field_end_len();
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    })() {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                        format!("decode struct `MiniRedisServicePingArgsRecv` field(#{}) failed", field_id),
                    ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end()?;

                    let data = Self {};
                    Ok(data)
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                        format!("decode struct `MiniRedisServicePingArgsRecv` field(#{}) failed", field_id),
                    ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let data = Self {};
                    Ok(data)
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServicePingArgsRecv",
                    }) + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
            #[derivative(Default)]
            #[derive(Clone, PartialEq)]

            pub enum MiniRedisServicePingResultSend {
                #[derivative(Default)]
                Ok(PingResponse),
            }

            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for MiniRedisServicePingResultSend {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServicePingResultSend",
                    })?;
                    match self {
                        MiniRedisServicePingResultSend::Ok(ref value) => {
                            protocol.write_struct_field(
                                0,
                                value,
                                ::pilota::thrift::TType::Struct,
                            )?;
                        }
                    }
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};
                    let mut ret = None;
                    protocol.read_struct_begin()?;
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                    protocol.struct_len(&field_ident);
                                    ret = Some(MiniRedisServicePingResultSend::Ok(field_ident));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new(
                                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }
                    }
                    protocol.read_field_end()?;
                    protocol.read_struct_end()?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut ret = None;
                    protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident =
                                        ::pilota::thrift::Message::decode_async(protocol).await?;

                                    ret = Some(MiniRedisServicePingResultSend::Ok(field_ident));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new(
                                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    protocol.read_field_end().await?;
                    protocol.read_struct_end().await?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServicePingResultSend",
                    }) + match self {
                        MiniRedisServicePingResultSend::Ok(ref value) => {
                            protocol.struct_field_len(Some(0), value)
                        }
                    } + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct MiniRedisServiceDeleteValueArgsRecv {
                pub request: DeleteValueRequest,
            }
            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for MiniRedisServiceDeleteValueArgsRecv {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceDeleteValueArgsRecv",
                    };

                    protocol.write_struct_begin(&struct_ident)?;
                    protocol.write_struct_field(
                        1,
                        &self.request,
                        ::pilota::thrift::TType::Struct,
                    )?;
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};

                    let mut request = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin()?;
                    if let Err(err) = (|| {
                        loop {
                            let field_ident = protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                protocol.field_stop_len();
                                break;
                            } else {
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    request = Some(::pilota::thrift::Message::decode(protocol)?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type)?;
                                }
                            }

                            protocol.read_field_end()?;
                            protocol.field_end_len();
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    })() {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                        format!("decode struct `MiniRedisServiceDeleteValueArgsRecv` field(#{}) failed", field_id),
                    ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end()?;

                    let Some(request) = request else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field request is required".to_string(),
                        ));
                    };

                    let data = Self { request };
                    Ok(data)
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut request = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    request = Some(
                                        ::pilota::thrift::Message::decode_async(protocol).await?,
                                    );
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                        format!("decode struct `MiniRedisServiceDeleteValueArgsRecv` field(#{}) failed", field_id),
                    ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let Some(request) = request else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field request is required".to_string(),
                        ));
                    };

                    let data = Self { request };
                    Ok(data)
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceDeleteValueArgsRecv",
                    }) + protocol.struct_field_len(Some(1), &self.request)
                        + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
            #[derivative(Default)]
            #[derive(Clone, PartialEq)]

            pub enum MiniRedisServiceDeleteValueResultSend {
                #[derivative(Default)]
                Ok(DeleteValueResponse),
            }

            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for MiniRedisServiceDeleteValueResultSend {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceDeleteValueResultSend",
                    })?;
                    match self {
                        MiniRedisServiceDeleteValueResultSend::Ok(ref value) => {
                            protocol.write_struct_field(
                                0,
                                value,
                                ::pilota::thrift::TType::Struct,
                            )?;
                        }
                    }
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};
                    let mut ret = None;
                    protocol.read_struct_begin()?;
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                    protocol.struct_len(&field_ident);
                                    ret = Some(MiniRedisServiceDeleteValueResultSend::Ok(
                                        field_ident,
                                    ));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new(
                                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }
                    }
                    protocol.read_field_end()?;
                    protocol.read_struct_end()?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut ret = None;
                    protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident =
                                        ::pilota::thrift::Message::decode_async(protocol).await?;

                                    ret = Some(MiniRedisServiceDeleteValueResultSend::Ok(
                                        field_ident,
                                    ));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new(
                                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    protocol.read_field_end().await?;
                    protocol.read_struct_end().await?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceDeleteValueResultSend",
                    }) + match self {
                        MiniRedisServiceDeleteValueResultSend::Ok(ref value) => {
                            protocol.struct_field_len(Some(0), value)
                        }
                    } + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct MiniRedisServiceSetValueArgsRecv {
                pub request: SetValueRequest,
            }
            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for MiniRedisServiceSetValueArgsRecv {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceSetValueArgsRecv",
                    };

                    protocol.write_struct_begin(&struct_ident)?;
                    protocol.write_struct_field(
                        1,
                        &self.request,
                        ::pilota::thrift::TType::Struct,
                    )?;
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};

                    let mut request = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin()?;
                    if let Err(err) = (|| {
                        loop {
                            let field_ident = protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                protocol.field_stop_len();
                                break;
                            } else {
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    request = Some(::pilota::thrift::Message::decode(protocol)?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type)?;
                                }
                            }

                            protocol.read_field_end()?;
                            protocol.field_end_len();
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    })() {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                        format!("decode struct `MiniRedisServiceSetValueArgsRecv` field(#{}) failed", field_id),
                    ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end()?;

                    let Some(request) = request else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field request is required".to_string(),
                        ));
                    };

                    let data = Self { request };
                    Ok(data)
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut request = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    request = Some(
                                        ::pilota::thrift::Message::decode_async(protocol).await?,
                                    );
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                        format!("decode struct `MiniRedisServiceSetValueArgsRecv` field(#{}) failed", field_id),
                    ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let Some(request) = request else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field request is required".to_string(),
                        ));
                    };

                    let data = Self { request };
                    Ok(data)
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceSetValueArgsRecv",
                    }) + protocol.struct_field_len(Some(1), &self.request)
                        + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
            #[derivative(Default)]
            #[derive(Clone, PartialEq)]

            pub enum MiniRedisServiceSetValueResultSend {
                #[derivative(Default)]
                Ok(SetValueResponse),
            }

            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for MiniRedisServiceSetValueResultSend {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceSetValueResultSend",
                    })?;
                    match self {
                        MiniRedisServiceSetValueResultSend::Ok(ref value) => {
                            protocol.write_struct_field(
                                0,
                                value,
                                ::pilota::thrift::TType::Struct,
                            )?;
                        }
                    }
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};
                    let mut ret = None;
                    protocol.read_struct_begin()?;
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                    protocol.struct_len(&field_ident);
                                    ret = Some(MiniRedisServiceSetValueResultSend::Ok(field_ident));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new(
                                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }
                    }
                    protocol.read_field_end()?;
                    protocol.read_struct_end()?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut ret = None;
                    protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident =
                                        ::pilota::thrift::Message::decode_async(protocol).await?;

                                    ret = Some(MiniRedisServiceSetValueResultSend::Ok(field_ident));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new(
                                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    protocol.read_field_end().await?;
                    protocol.read_struct_end().await?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceSetValueResultSend",
                    }) + match self {
                        MiniRedisServiceSetValueResultSend::Ok(ref value) => {
                            protocol.struct_field_len(Some(0), value)
                        }
                    } + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct MiniRedisServiceGetValueArgsRecv {
                pub request: GetValueRequest,
            }
            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for MiniRedisServiceGetValueArgsRecv {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceGetValueArgsRecv",
                    };

                    protocol.write_struct_begin(&struct_ident)?;
                    protocol.write_struct_field(
                        1,
                        &self.request,
                        ::pilota::thrift::TType::Struct,
                    )?;
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};

                    let mut request = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin()?;
                    if let Err(err) = (|| {
                        loop {
                            let field_ident = protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                protocol.field_stop_len();
                                break;
                            } else {
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    request = Some(::pilota::thrift::Message::decode(protocol)?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type)?;
                                }
                            }

                            protocol.read_field_end()?;
                            protocol.field_end_len();
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    })() {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                        format!("decode struct `MiniRedisServiceGetValueArgsRecv` field(#{}) failed", field_id),
                    ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end()?;

                    let Some(request) = request else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field request is required".to_string(),
                        ));
                    };

                    let data = Self { request };
                    Ok(data)
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut request = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    request = Some(
                                        ::pilota::thrift::Message::decode_async(protocol).await?,
                                    );
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                        format!("decode struct `MiniRedisServiceGetValueArgsRecv` field(#{}) failed", field_id),
                    ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let Some(request) = request else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field request is required".to_string(),
                        ));
                    };

                    let data = Self { request };
                    Ok(data)
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceGetValueArgsRecv",
                    }) + protocol.struct_field_len(Some(1), &self.request)
                        + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
            #[derivative(Default)]
            #[derive(Clone, PartialEq)]

            pub enum MiniRedisServiceGetValueResultSend {
                #[derivative(Default)]
                Ok(GetValueResponse),
            }

            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for MiniRedisServiceGetValueResultSend {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceGetValueResultSend",
                    })?;
                    match self {
                        MiniRedisServiceGetValueResultSend::Ok(ref value) => {
                            protocol.write_struct_field(
                                0,
                                value,
                                ::pilota::thrift::TType::Struct,
                            )?;
                        }
                    }
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};
                    let mut ret = None;
                    protocol.read_struct_begin()?;
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                    protocol.struct_len(&field_ident);
                                    ret = Some(MiniRedisServiceGetValueResultSend::Ok(field_ident));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new(
                                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }
                    }
                    protocol.read_field_end()?;
                    protocol.read_struct_end()?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut ret = None;
                    protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident =
                                        ::pilota::thrift::Message::decode_async(protocol).await?;

                                    ret = Some(MiniRedisServiceGetValueResultSend::Ok(field_ident));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new(
                                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    protocol.read_field_end().await?;
                    protocol.read_struct_end().await?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceGetValueResultSend",
                    }) + match self {
                        MiniRedisServiceGetValueResultSend::Ok(ref value) => {
                            protocol.struct_field_len(Some(0), value)
                        }
                    } + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct PingResponse {
                pub pong: ::pilota::FastStr,
            }
            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for PingResponse {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "PingResponse",
                    };

                    protocol.write_struct_begin(&struct_ident)?;
                    protocol.write_faststr_field(1, (&self.pong).clone())?;
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};

                    let mut pong = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin()?;
                    if let Err(err) = (|| {
                        loop {
                            let field_ident = protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                protocol.field_stop_len();
                                break;
                            } else {
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    pong = Some(protocol.read_faststr()?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type)?;
                                }
                            }

                            protocol.read_field_end()?;
                            protocol.field_end_len();
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    })() {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!("decode struct `PingResponse` field(#{}) failed", field_id),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end()?;

                    let Some(pong) = pong else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field pong is required".to_string(),
                        ));
                    };

                    let data = Self { pong };
                    Ok(data)
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut pong = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    pong = Some(protocol.read_faststr().await?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!("decode struct `PingResponse` field(#{}) failed", field_id),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let Some(pong) = pong else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field pong is required".to_string(),
                        ));
                    };

                    let data = Self { pong };
                    Ok(data)
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "PingResponse",
                    }) + protocol.faststr_field_len(Some(1), &self.pong)
                        + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct SetValueResponse {
                pub error: ::pilota::FastStr,
            }
            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for SetValueResponse {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "SetValueResponse",
                    };

                    protocol.write_struct_begin(&struct_ident)?;
                    protocol.write_faststr_field(1, (&self.error).clone())?;
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};

                    let mut error = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin()?;
                    if let Err(err) = (|| {
                        loop {
                            let field_ident = protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                protocol.field_stop_len();
                                break;
                            } else {
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    error = Some(protocol.read_faststr()?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type)?;
                                }
                            }

                            protocol.read_field_end()?;
                            protocol.field_end_len();
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    })() {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `SetValueResponse` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end()?;

                    let Some(error) = error else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field error is required".to_string(),
                        ));
                    };

                    let data = Self { error };
                    Ok(data)
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut error = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    error = Some(protocol.read_faststr().await?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `SetValueResponse` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let Some(error) = error else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field error is required".to_string(),
                        ));
                    };

                    let data = Self { error };
                    Ok(data)
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "SetValueResponse",
                    }) + protocol.faststr_field_len(Some(1), &self.error)
                        + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct GetValueRequest {
                pub key: ::pilota::FastStr,
            }
            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for GetValueRequest {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "GetValueRequest",
                    };

                    protocol.write_struct_begin(&struct_ident)?;
                    protocol.write_faststr_field(1, (&self.key).clone())?;
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};

                    let mut key = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin()?;
                    if let Err(err) = (|| {
                        loop {
                            let field_ident = protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                protocol.field_stop_len();
                                break;
                            } else {
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    key = Some(protocol.read_faststr()?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type)?;
                                }
                            }

                            protocol.read_field_end()?;
                            protocol.field_end_len();
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    })() {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `GetValueRequest` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end()?;

                    let Some(key) = key else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field key is required".to_string(),
                        ));
                    };

                    let data = Self { key };
                    Ok(data)
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut key = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    key = Some(protocol.read_faststr().await?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `GetValueRequest` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let Some(key) = key else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field key is required".to_string(),
                        ));
                    };

                    let data = Self { key };
                    Ok(data)
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "GetValueRequest",
                    }) + protocol.faststr_field_len(Some(1), &self.key)
                        + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct MiniRedisServicePingArgsSend {}
            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for MiniRedisServicePingArgsSend {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServicePingArgsSend",
                    };

                    protocol.write_struct_begin(&struct_ident)?;

                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin()?;
                    if let Err(err) = (|| {
                        loop {
                            let field_ident = protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                protocol.field_stop_len();
                                break;
                            } else {
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                _ => {
                                    protocol.skip(field_ident.field_type)?;
                                }
                            }

                            protocol.read_field_end()?;
                            protocol.field_end_len();
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    })() {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                        format!("decode struct `MiniRedisServicePingArgsSend` field(#{}) failed", field_id),
                    ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end()?;

                    let data = Self {};
                    Ok(data)
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                        format!("decode struct `MiniRedisServicePingArgsSend` field(#{}) failed", field_id),
                    ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let data = Self {};
                    Ok(data)
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServicePingArgsSend",
                    }) + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct MiniRedisServiceDeleteValueArgsSend {
                pub request: DeleteValueRequest,
            }
            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for MiniRedisServiceDeleteValueArgsSend {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceDeleteValueArgsSend",
                    };

                    protocol.write_struct_begin(&struct_ident)?;
                    protocol.write_struct_field(
                        1,
                        &self.request,
                        ::pilota::thrift::TType::Struct,
                    )?;
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};

                    let mut request = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin()?;
                    if let Err(err) = (|| {
                        loop {
                            let field_ident = protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                protocol.field_stop_len();
                                break;
                            } else {
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    request = Some(::pilota::thrift::Message::decode(protocol)?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type)?;
                                }
                            }

                            protocol.read_field_end()?;
                            protocol.field_end_len();
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    })() {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                        format!("decode struct `MiniRedisServiceDeleteValueArgsSend` field(#{}) failed", field_id),
                    ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end()?;

                    let Some(request) = request else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field request is required".to_string(),
                        ));
                    };

                    let data = Self { request };
                    Ok(data)
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut request = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    request = Some(
                                        ::pilota::thrift::Message::decode_async(protocol).await?,
                                    );
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                        format!("decode struct `MiniRedisServiceDeleteValueArgsSend` field(#{}) failed", field_id),
                    ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let Some(request) = request else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field request is required".to_string(),
                        ));
                    };

                    let data = Self { request };
                    Ok(data)
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceDeleteValueArgsSend",
                    }) + protocol.struct_field_len(Some(1), &self.request)
                        + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct MiniRedisServiceSetValueArgsSend {
                pub request: SetValueRequest,
            }
            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for MiniRedisServiceSetValueArgsSend {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceSetValueArgsSend",
                    };

                    protocol.write_struct_begin(&struct_ident)?;
                    protocol.write_struct_field(
                        1,
                        &self.request,
                        ::pilota::thrift::TType::Struct,
                    )?;
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};

                    let mut request = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin()?;
                    if let Err(err) = (|| {
                        loop {
                            let field_ident = protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                protocol.field_stop_len();
                                break;
                            } else {
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    request = Some(::pilota::thrift::Message::decode(protocol)?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type)?;
                                }
                            }

                            protocol.read_field_end()?;
                            protocol.field_end_len();
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    })() {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                        format!("decode struct `MiniRedisServiceSetValueArgsSend` field(#{}) failed", field_id),
                    ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end()?;

                    let Some(request) = request else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field request is required".to_string(),
                        ));
                    };

                    let data = Self { request };
                    Ok(data)
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut request = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    request = Some(
                                        ::pilota::thrift::Message::decode_async(protocol).await?,
                                    );
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                        format!("decode struct `MiniRedisServiceSetValueArgsSend` field(#{}) failed", field_id),
                    ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let Some(request) = request else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field request is required".to_string(),
                        ));
                    };

                    let data = Self { request };
                    Ok(data)
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceSetValueArgsSend",
                    }) + protocol.struct_field_len(Some(1), &self.request)
                        + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct MiniRedisServiceGetValueArgsSend {
                pub request: GetValueRequest,
            }
            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for MiniRedisServiceGetValueArgsSend {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceGetValueArgsSend",
                    };

                    protocol.write_struct_begin(&struct_ident)?;
                    protocol.write_struct_field(
                        1,
                        &self.request,
                        ::pilota::thrift::TType::Struct,
                    )?;
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};

                    let mut request = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin()?;
                    if let Err(err) = (|| {
                        loop {
                            let field_ident = protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                protocol.field_stop_len();
                                break;
                            } else {
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    request = Some(::pilota::thrift::Message::decode(protocol)?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type)?;
                                }
                            }

                            protocol.read_field_end()?;
                            protocol.field_end_len();
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    })() {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                        format!("decode struct `MiniRedisServiceGetValueArgsSend` field(#{}) failed", field_id),
                    ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end()?;

                    let Some(request) = request else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field request is required".to_string(),
                        ));
                    };

                    let data = Self { request };
                    Ok(data)
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut request = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    request = Some(
                                        ::pilota::thrift::Message::decode_async(protocol).await?,
                                    );
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                        format!("decode struct `MiniRedisServiceGetValueArgsSend` field(#{}) failed", field_id),
                    ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let Some(request) = request else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field request is required".to_string(),
                        ));
                    };

                    let data = Self { request };
                    Ok(data)
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "MiniRedisServiceGetValueArgsSend",
                    }) + protocol.struct_field_len(Some(1), &self.request)
                        + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct DeleteValueResponse {
                pub error: ::pilota::FastStr,
            }
            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for DeleteValueResponse {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "DeleteValueResponse",
                    };

                    protocol.write_struct_begin(&struct_ident)?;
                    protocol.write_faststr_field(1, (&self.error).clone())?;
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};

                    let mut error = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin()?;
                    if let Err(err) = (|| {
                        loop {
                            let field_ident = protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                protocol.field_stop_len();
                                break;
                            } else {
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    error = Some(protocol.read_faststr()?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type)?;
                                }
                            }

                            protocol.read_field_end()?;
                            protocol.field_end_len();
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    })() {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `DeleteValueResponse` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end()?;

                    let Some(error) = error else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field error is required".to_string(),
                        ));
                    };

                    let data = Self { error };
                    Ok(data)
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut error = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    error = Some(protocol.read_faststr().await?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `DeleteValueResponse` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let Some(error) = error else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field error is required".to_string(),
                        ));
                    };

                    let data = Self { error };
                    Ok(data)
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "DeleteValueResponse",
                    }) + protocol.faststr_field_len(Some(1), &self.error)
                        + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct SetValueRequest {
                pub key: ::pilota::FastStr,

                pub value: ::pilota::FastStr,

                pub expire_seconds: ::std::option::Option<i32>,
            }
            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for SetValueRequest {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "SetValueRequest",
                    };

                    protocol.write_struct_begin(&struct_ident)?;
                    protocol.write_faststr_field(1, (&self.key).clone())?;
                    protocol.write_faststr_field(2, (&self.value).clone())?;
                    if let Some(value) = self.expire_seconds.as_ref() {
                        protocol.write_i32_field(3, *value)?;
                    }
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};

                    let mut key = None;
                    let mut value = None;
                    let mut expire_seconds = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin()?;
                    if let Err(err) = (|| {
                        loop {
                            let field_ident = protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                protocol.field_stop_len();
                                break;
                            } else {
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    key = Some(protocol.read_faststr()?);
                                }
                                Some(2)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    value = Some(protocol.read_faststr()?);
                                }
                                Some(3)
                                    if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                                {
                                    expire_seconds = Some(protocol.read_i32()?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type)?;
                                }
                            }

                            protocol.read_field_end()?;
                            protocol.field_end_len();
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    })() {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `SetValueRequest` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end()?;

                    let Some(key) = key else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field key is required".to_string(),
                        ));
                    };
                    let Some(value) = value else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field value is required".to_string(),
                        ));
                    };

                    let data = Self {
                        key,
                        value,
                        expire_seconds,
                    };
                    Ok(data)
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut key = None;
                    let mut value = None;
                    let mut expire_seconds = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    key = Some(protocol.read_faststr().await?);
                                }
                                Some(2)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    value = Some(protocol.read_faststr().await?);
                                }
                                Some(3)
                                    if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                                {
                                    expire_seconds = Some(protocol.read_i32().await?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `SetValueRequest` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let Some(key) = key else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field key is required".to_string(),
                        ));
                    };
                    let Some(value) = value else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field value is required".to_string(),
                        ));
                    };

                    let data = Self {
                        key,
                        value,
                        expire_seconds,
                    };
                    Ok(data)
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "SetValueRequest",
                    }) + protocol.faststr_field_len(Some(1), &self.key)
                        + protocol.faststr_field_len(Some(2), &self.value)
                        + self
                            .expire_seconds
                            .as_ref()
                            .map_or(0, |value| protocol.i32_field_len(Some(3), *value))
                        + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
        }
    }
}
