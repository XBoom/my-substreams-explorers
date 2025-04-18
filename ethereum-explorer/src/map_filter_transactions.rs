use crate::pb::eth::transaction::v1::{Transaction, Transactions}; // 引入交易相关结构体
use crate::util; // 引入自定义工具模块
use anyhow::anyhow; // 用于错误处理
use serde::Deserialize; // 用于反序列化参数
use substreams::Hex; // 用于十六进制编码和解码
use substreams_ethereum::pb::eth::v2::{Block, TransactionTrace, TransactionTraceStatus}; // 引入以太坊区块和交易追踪结构体

// 交易过滤参数结构体，支持 from 和 to 地址过滤
#[derive(Deserialize)]
struct TransactionFilterParams {
    to: Option<String>,   // 目标地址，可选
    from: Option<String>, // 来源地址，可选
}

// Substreams 映射处理器，根据过滤参数筛选区块中的交易
#[substreams::handlers::map]
fn map_filter_transactions(params: String, blk: Block) -> Result<Transactions, Vec<substreams::errors::Error>> {
    let filters = parse_filters_from_params(params)?; // 解析并校验过滤参数

    let transactions: Vec<Transaction> = blk
        .transactions() // 获取区块中的所有交易
        .filter(|trans| apply_filter(&trans, &filters)) // 应用过滤条件
        .map(|trans| Transaction {
            from: Hex::encode(&trans.from), // 发送方地址（十六进制字符串）
            to: Hex::encode(&trans.to),     // 接收方地址（十六进制字符串）
            hash: Hex::encode(&trans.hash), // 交易哈希
        })
        .collect();

    Ok(Transactions { transactions }) // 返回符合条件的交易集合
}

// 解析过滤参数并校验有效性
fn parse_filters_from_params(params: String) -> Result<TransactionFilterParams, Vec<substreams::errors::Error>> {
    let parsed_result = serde_qs::from_str(&params); // 解析查询字符串为结构体
    if parsed_result.is_err() {
        return Err(Vec::from([anyhow!("Unpexcted error while parsing parameters")])); // 解析失败返回错误
    }

    let filters = parsed_result.unwrap();
    verify_filters(&filters)?; // 校验过滤参数

    Ok(filters)
}

// 校验过滤参数中的地址是否合法
fn verify_filters(params: &TransactionFilterParams) -> Result<(), Vec<substreams::errors::Error>> {
    let mut errors: Vec<substreams::errors::Error> = Vec::new();

    // 校验 from 地址
    if params.from.is_some() && !util::is_address_valid(&params.from.as_ref().unwrap()) {
        let from = params.from.as_ref().unwrap();

        if !util::is_address_valid(from) {
            errors.push(anyhow!("'from' address ({}) is not valid", from));
        }
    }

    // 校验 to 地址
    if params.to.is_some() && !util::is_address_valid(&params.to.as_ref().unwrap()) {
        let to = params.to.as_ref().unwrap();

        if !util::is_address_valid(to) {
            errors.push(anyhow!("'to' address ({}) is not valid", to));
        }
    }

    if errors.len() > 0 {
        return Err(errors); // 有错误则返回
    }

    Ok(())
}

// 判断单笔交易是否符合过滤条件
fn apply_filter(transaction: &TransactionTrace, filters: &TransactionFilterParams) -> bool {
    if !filter_by_parameter(&filters.from, &transaction.from)
        || !filter_by_parameter(&filters.to, &transaction.to)
        || transaction.status != (TransactionTraceStatus::Succeeded as i32) // 只保留成功的交易
    {
        return false;
    }
    true
}

// 判断交易字段是否与过滤参数匹配
fn filter_by_parameter(parameter: &Option<String>, transaction_field: &Vec<u8>) -> bool {
    if parameter.is_none() {
        return true; // 未指定过滤条件则通过
    }

    let parameter_as_vec = &Hex::decode(parameter.as_ref().unwrap()).expect("already verified");
    if transaction_field == parameter_as_vec {
        return true; // 匹配则通过
    }

    false // 不匹配则过滤掉
}
