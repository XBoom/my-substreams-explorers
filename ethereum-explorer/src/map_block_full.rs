use substreams_ethereum::pb::eth::v2::Block; // 引入以太坊区块结构体 Block

// Substreams 映射处理器，直接返回完整的以太坊区块数据
#[substreams::handlers::map]
fn map_block_full(blk: Block) -> Result<Block, substreams::errors::Error> {
    Ok(blk) // 直接返回输入的区块对象
}
