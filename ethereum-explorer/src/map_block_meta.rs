use crate::pb::eth::block_meta::v1::BlockMeta; // 引入 BlockMeta 结构体，用于输出区块元数据
use substreams::Hex; // 用于将字节数组编码为十六进制字符串
use substreams_ethereum::pb::eth::v2::Block; // 引入以太坊区块结构体，包含了一个区块的所有关键信息

// Substreams 映射处理器，将以太坊区块映射为 BlockMeta
#[substreams::handlers::map]
fn map_block_meta(blk: Block) -> Result<BlockMeta, substreams::errors::Error> {
    let header = blk.header.as_ref().unwrap(); // 获取区块头部信息

    Ok(BlockMeta {
        number: blk.number, // 区块高度
        hash: Hex::encode(&blk.hash), // 当前区块哈希（十六进制字符串）
        parent_hash: Hex::encode(&header.parent_hash), // 父区块哈希（十六进制字符串）
    })
}
