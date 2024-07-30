const _ZH_CHARS: &str = "欢迎光临祝你一路顺风车位已满年月日时分秒！";
const _ZH_FONTS_BITMAP: [[u8; 32]; 21] = [
    [
        0x04, 0x24, 0x44, 0x84, 0x64, 0x9C, 0x40, 0x30, 0x0F, 0xC8, 0x08, 0x08, 0x28, 0x18, 0x00,
        0x00, 0x10, 0x08, 0x06, 0x01, 0x82, 0x4C, 0x20, 0x18, 0x06, 0x01, 0x06, 0x18, 0x20, 0x40,
        0x80, 0x00,
    ],
    [
        0x40, 0x40, 0x42, 0xCC, 0x00, 0x00, 0xFC, 0x04, 0x02, 0x00, 0xFC, 0x04, 0x04, 0xFC, 0x00,
        0x00, 0x00, 0x40, 0x20, 0x1F, 0x20, 0x40, 0x4F, 0x44, 0x42, 0x40, 0x7F, 0x42, 0x44, 0x43,
        0x40, 0x00,
    ],
    [
        0x40, 0x40, 0x42, 0x44, 0x58, 0xC0, 0x40, 0x7F, 0x40, 0xC0, 0x50, 0x48, 0x46, 0x40, 0x40,
        0x00, 0x80, 0x80, 0x40, 0x20, 0x18, 0x07, 0x00, 0x00, 0x00, 0x3F, 0x40, 0x40, 0x40, 0x40,
        0x78, 0x00,
    ],
    [
        0x00, 0xF8, 0x00, 0x00, 0xFF, 0x40, 0x20, 0x18, 0x0F, 0x18, 0x68, 0x08, 0x08, 0x08, 0x08,
        0x00, 0x00, 0x1F, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x7F, 0x21, 0x21, 0x3F, 0x21, 0x21, 0x7F,
        0x00, 0x00,
    ],
    [
        0x08, 0x08, 0x89, 0xEE, 0x98, 0x00, 0x7E, 0x42, 0xC2, 0x42, 0x42, 0xC2, 0x42, 0x7E, 0x00,
        0x00, 0x02, 0x01, 0x00, 0xFF, 0x80, 0x43, 0x20, 0x18, 0x07, 0x00, 0x00, 0x3F, 0x40, 0x40,
        0x78, 0x00,
    ],
    [
        0x00, 0x80, 0x60, 0xF8, 0x07, 0x40, 0x20, 0x18, 0x0F, 0x08, 0xC8, 0x08, 0x08, 0x28, 0x18,
        0x00, 0x01, 0x00, 0x00, 0xFF, 0x00, 0x10, 0x0C, 0x03, 0x40, 0x80, 0x7F, 0x00, 0x01, 0x06,
        0x18, 0x00,
    ],
    [
        0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ],
    [
        0x00, 0x3E, 0x22, 0xE2, 0x22, 0x3E, 0x00, 0x10, 0x88, 0x57, 0x24, 0x54, 0x8C, 0x00, 0x00,
        0x00, 0x40, 0x7E, 0x40, 0x3F, 0x22, 0x22, 0x00, 0x01, 0xFE, 0x42, 0x42, 0x42, 0xFE, 0x01,
        0x01, 0x00,
    ],
    [
        0x00, 0xFE, 0x00, 0xFC, 0x00, 0xFF, 0x00, 0xF2, 0x12, 0x1A, 0xD6, 0x12, 0x12, 0xF2, 0x02,
        0x00, 0x40, 0x3F, 0x00, 0x3F, 0x00, 0xFF, 0x80, 0x4F, 0x20, 0x18, 0x07, 0x10, 0x20, 0x4F,
        0x80, 0x00,
    ],
    [
        0x00, 0x00, 0xFE, 0x02, 0x12, 0x22, 0xC2, 0x02, 0xC2, 0x32, 0x02, 0xFE, 0x00, 0x00, 0x00,
        0x00, 0x80, 0x60, 0x1F, 0x00, 0x20, 0x10, 0x0C, 0x03, 0x0C, 0x30, 0x00, 0x0F, 0x30, 0x40,
        0xF8, 0x00,
    ],
    [
        0x00, 0x08, 0x88, 0x48, 0x28, 0x18, 0x0F, 0xE8, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x00,
        0x00, 0x08, 0x08, 0x09, 0x09, 0x09, 0x09, 0x09, 0xFF, 0x09, 0x09, 0x09, 0x09, 0x09, 0x08,
        0x08, 0x00,
    ],
    [
        0x00, 0x80, 0x60, 0xF8, 0x07, 0x10, 0x90, 0x10, 0x11, 0x16, 0x10, 0x10, 0xD0, 0x10, 0x00,
        0x00, 0x01, 0x00, 0x00, 0xFF, 0x40, 0x40, 0x41, 0x5E, 0x40, 0x40, 0x70, 0x4E, 0x41, 0x40,
        0x40, 0x00,
    ],
    [
        0x00, 0x00, 0xE2, 0x82, 0x82, 0x82, 0x82, 0x82, 0x82, 0x82, 0x82, 0xFE, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x3F, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x78,
        0x00, 0x00,
    ],
    [
        0x10, 0x60, 0x02, 0x8C, 0x00, 0x24, 0x24, 0x2F, 0xE4, 0x24, 0x24, 0xE4, 0x2F, 0x24, 0x24,
        0x00, 0x04, 0x04, 0x7E, 0x01, 0x00, 0xFF, 0x11, 0x09, 0x07, 0x19, 0x09, 0x07, 0x49, 0x91,
        0x7F, 0x00,
    ],
    [
        0x00, 0x20, 0x18, 0xC7, 0x44, 0x44, 0x44, 0x44, 0xFC, 0x44, 0x44, 0x44, 0x44, 0x04, 0x00,
        0x00, 0x04, 0x04, 0x04, 0x07, 0x04, 0x04, 0x04, 0x04, 0xFF, 0x04, 0x04, 0x04, 0x04, 0x04,
        0x04, 0x00,
    ],
    [
        0x00, 0x00, 0x00, 0xFE, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0xFE, 0x00, 0x00,
        0x00, 0x80, 0x40, 0x30, 0x0F, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x42, 0x82, 0x7F, 0x00,
        0x00, 0x00,
    ],
    [
        0x00, 0x00, 0x00, 0xFE, 0x82, 0x82, 0x82, 0x82, 0x82, 0x82, 0x82, 0xFE, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xFF, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0xFF, 0x00, 0x00,
        0x00, 0x00,
    ],
    [
        0x00, 0xFC, 0x84, 0x84, 0x84, 0xFC, 0x00, 0x10, 0x10, 0x10, 0x10, 0x10, 0xFF, 0x10, 0x10,
        0x00, 0x00, 0x3F, 0x10, 0x10, 0x10, 0x3F, 0x00, 0x00, 0x01, 0x06, 0x40, 0x80, 0x7F, 0x00,
        0x00, 0x00,
    ],
    [
        0x80, 0x40, 0x20, 0x90, 0x88, 0x86, 0x80, 0x80, 0x80, 0x83, 0x8C, 0x10, 0x20, 0x40, 0x80,
        0x00, 0x00, 0x80, 0x40, 0x20, 0x18, 0x07, 0x00, 0x40, 0x80, 0x40, 0x3F, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ],
    [
        0x24, 0x24, 0xA4, 0xFE, 0x23, 0x22, 0x00, 0xC0, 0x38, 0x00, 0xFF, 0x00, 0x08, 0x10, 0x60,
        0x00, 0x08, 0x06, 0x01, 0xFF, 0x01, 0x06, 0x81, 0x80, 0x40, 0x40, 0x27, 0x10, 0x0C, 0x03,
        0x00, 0x00,
    ],
    [
        0x00, 0x00, 0x00, 0xFE, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x33, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ],
];

// 获取中文字体字模
pub fn get_zh_font(c: char) -> Result<[u8; 32], &'static str> {
    for (i, ci) in _ZH_CHARS.chars().enumerate() {
        if i > _ZH_FONTS_BITMAP.len() {
            return Err("Out of index.");
        }
        if ci == c {
            return Ok(_ZH_FONTS_BITMAP[i]);
        }
    }

    Err("Unknown")
}
