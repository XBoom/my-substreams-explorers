use crate::pb::eth::event::v1::Event; // 引入 Event 结构体，表示单个以太坊事件
use crate::pb::eth::event::v1::Events; // 引入 Events 结构体，表示事件集合
use crate::util; // 引入自定义工具模块
use anyhow::anyhow; // 用于错误处理
use anyhow::Ok; // 用于返回 Ok 结果
use substreams::errors::Error; // 引入 Substreams 错误类型
use substreams::Hex; // 用于十六进制编码和解码
use substreams_ethereum::pb::eth::v2::Block; // 引入以太坊区块结构体

// Substreams 映射处理器，提取指定合约地址的所有事件日志
#[substreams::handlers::map]
fn map_contract_events(contract_address: String, blk: Block) -> Result<Events, Error> {
    verify_parameter(&contract_address)?; // 校验合约地址是否合法

    let events: Vec<Event> = blk
        .logs() // 获取区块中的所有日志
        .filter(|log| log.address().to_vec() == Hex::decode(&contract_address).expect("already validated")) // 过滤出指定合约地址的日志
        .map(|log| Event {
            address: Hex::encode(log.address()), // 事件发生的合约地址（十六进制字符串）
            topics: log.topics().into_iter().map(Hex::encode).collect(), // 事件主题数组（十六进制字符串）
            tx_hash: Hex::encode(&log.receipt.transaction.hash), // 事件所属交易哈希
        })
        .collect();

    Ok(Events { events }) // 返回事件集合
}

// 校验合约地址是否合法
fn verify_parameter(address: &String) -> Result<(), Error> {
    if !util::is_address_valid(&address) {
        return Err(anyhow!("Contract address ({}) is not valid", address)); // 地址不合法则返回错误
    }

    Ok(())
}
