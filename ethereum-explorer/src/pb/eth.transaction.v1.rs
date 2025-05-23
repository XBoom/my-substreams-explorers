// @generated
// This file is @generated by prost-build.
/// Transactions 消息，表示一组以太坊交易
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transactions {
    /// 包含多个 Transaction，每个 Transaction 表示一笔交易
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
/// Transaction 消息，定义单个以太坊交易的结构
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    /// 交易发送方地址
    #[prost(string, tag="1")]
    pub from: ::prost::alloc::string::String,
    /// 交易接收方地址
    #[prost(string, tag="2")]
    pub to: ::prost::alloc::string::String,
    /// 交易哈希值
    #[prost(string, tag="3")]
    pub hash: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
