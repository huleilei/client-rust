#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CpuTimeRecord {
    #[prost(bytes = "vec", tag = "1")]
    pub resource_group_tag: ::prost::alloc::vec::Vec<u8>,
    /// UNIX timestamp in second.
    #[prost(uint64, repeated, tag = "2")]
    pub record_list_timestamp_sec: ::prost::alloc::vec::Vec<u64>,
    /// The value can be greater than 1000ms if the requests are running parallelly.
    #[prost(uint32, repeated, tag = "3")]
    pub record_list_cpu_time_ms: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceUsageRecord {
    #[prost(bytes = "vec", tag = "1")]
    pub resource_group_tag: ::prost::alloc::vec::Vec<u8>,
    /// UNIX timestamp in second.
    #[prost(uint64, repeated, tag = "2")]
    pub record_list_timestamp_sec: ::prost::alloc::vec::Vec<u64>,
    /// The value can be greater than 1000ms if the requests are running parallelly.
    #[prost(uint32, repeated, tag = "3")]
    pub record_list_cpu_time_ms: ::prost::alloc::vec::Vec<u32>,
    /// The number of reads of keys associated with resource_group_tag.
    #[prost(uint32, repeated, tag = "4")]
    pub record_list_read_keys: ::prost::alloc::vec::Vec<u32>,
    /// The number of writes of keys associated with resource_group_tag.
    #[prost(uint32, repeated, tag = "5")]
    pub record_list_write_keys: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceMeteringRequest {}
/// Generated client implementations.
pub mod resource_usage_agent_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// ResourceUsageAgent is the service for storing resource usage records.
    #[derive(Debug, Clone)]
    pub struct ResourceUsageAgentClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ResourceUsageAgentClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ResourceUsageAgentClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ResourceUsageAgentClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ResourceUsageAgentClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// DEPRECATED: We now use `Report` to report not only CPU time.
        ///
        /// Report the CPU time records. By default, the records with the same
        /// resource group tag will be batched by minute.
        pub async fn report_cpu_time(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::CpuTimeRecord>,
        ) -> std::result::Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/resource_usage_agent.ResourceUsageAgent/ReportCPUTime",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "resource_usage_agent.ResourceUsageAgent",
                        "ReportCPUTime",
                    ),
                );
            self.inner.client_streaming(req, path, codec).await
        }
        /// Report the resource usage records. By default, the records with the same
        /// resource group tag will be batched by minute.
        pub async fn report(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::ResourceUsageRecord,
            >,
        ) -> std::result::Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/resource_usage_agent.ResourceUsageAgent/Report",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("resource_usage_agent.ResourceUsageAgent", "Report"),
                );
            self.inner.client_streaming(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod resource_metering_pub_sub_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// TiKV implements ResourceMeteringPubSub service for clients to subscribe to resource metering records.
    #[derive(Debug, Clone)]
    pub struct ResourceMeteringPubSubClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ResourceMeteringPubSubClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ResourceMeteringPubSubClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ResourceMeteringPubSubClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ResourceMeteringPubSubClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Clients subscribe to resource metering records through this RPC, and TiKV periodically (e.g. per minute)
        /// publishes resource metering records to clients via gRPC stream.
        pub async fn subscribe(
            &mut self,
            request: impl tonic::IntoRequest<super::ResourceMeteringRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ResourceUsageRecord>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/resource_usage_agent.ResourceMeteringPubSub/Subscribe",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "resource_usage_agent.ResourceMeteringPubSub",
                        "Subscribe",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
