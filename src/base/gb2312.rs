use heapless::Vec;

static _ZH_CHARS: &str = "欢迎光临祝你一路顺风车位已满年月日时分秒今日温湿度剩余个";
static _ZH_GB2312_BYTES: [[u8; 2]; 28] = [
    [0xBB, 0xB6],
    [0xD3, 0xAD],
    [0xB9, 0xE2],
    [0xC1, 0xD9],
    [0xD7, 0xA3],
    [0xC4, 0xE3],
    [0xD2, 0xBB],
    [0xC2, 0xB7],
    [0xCB, 0xB3],
    [0xB7, 0xE7],
    [0xB3, 0xB5],
    [0xCE, 0xBB],
    [0xD2, 0xD1],
    [0xC2, 0xFA],
    [0xC4, 0xEA],
    [0xD4, 0xC2],
    [0xC8, 0xD5],
    [0xCA, 0xB1],
    [0xB7, 0xD6],
    [0xC3, 0xEB],
    [0xBD, 0xF1],
    [0xC8, 0xD5],
    [0xCE, 0xC2],
    [0xCA, 0xAA],
    [0xB6, 0xC8],
    [0xCA, 0xA3],
    [0xD3, 0xE0],
    [0xB8, 0xF6],
];

// 字符串转gb2312字符
pub fn str_to_gb2312(s: &str) -> Vec<u8, 16> {
    let mut buffer = Vec::<u8, 16>::new();

    for c in s.chars() {
        let mut exists = false;

        for (i, z) in _ZH_CHARS.chars().enumerate() {
            if c == z {
                let bytes = _ZH_GB2312_BYTES[i];
                buffer.push(bytes[0]).ok();
                buffer.push(bytes[1]).ok();
                exists = true;
                break;
            }
        }

        // 不存在对应字符时采用默认编码
        if !exists {
            buffer.push(c as u8).ok();
        }
    }

    buffer
}
