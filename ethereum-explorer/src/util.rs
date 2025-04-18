pub fn is_address_valid(address: &String) -> bool {
    // 以太坊地址总是 40 个十六进制字符（不带 0x 前缀）或 42 个字符（带 0x 前缀）
    if address.len() != 40 && address.len() != 42 {
        return false; // 长度不符则判定为无效地址
    }

    true // 长度符合则判定为有效地址（未进一步校验字符内容）
}
