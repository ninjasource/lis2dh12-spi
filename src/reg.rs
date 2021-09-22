#![allow(non_upper_case_globals)]

/// Register mapping
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone)]
pub enum Register {
    STATUS_REG_AUX = 0x07,
    OUT_TEMP_L = 0x0C,
    OUT_TEMP_H = 0x0D,
    WHO_AM_I = 0x0F,
    CTRL_REG0 = 0x1E,
    TEMP_CFG_REG = 0x1F,
    CTRL_REG1 = 0x20,
    CTRL_REG2 = 0x21,
    CTRL_REG3 = 0x22,
    CTRL_REG4 = 0x23,
    CTRL_REG5 = 0x24,
    CTRL_REG6 = 0x25,
    REFERENCE = 0x26,
    STATUS_REG = 0x27,
    OUT_X_L = 0x28,
    OUT_X_H = 0x29,
    OUT_Y_L = 0x2A,
    OUT_Y_H = 0x2B,
    OUT_Z_L = 0x2C,
    OUT_Z_H = 0x2D,
    FIFO_CTRL_REG = 0x2E,
    FIFO_SRC_REG = 0x2F,
    INT1_CFG = 0x30,
    INT1_SRC = 0x31,
    INT1_THS = 0x32,
    INT1_DURATION = 0x33,
    INT2_CFG = 0x34,
    INT2_SRC = 0x35,
    INT2_THS = 0x36,
    INT2_DURATION = 0x37,
    CLICK_CFG = 0x38,
    CLICK_SRC = 0x39,
    CLICK_THS = 0x3A,
    TIME_LIMIT = 0x3B,
    TIME_LATENCY = 0x3C,
    TIME_WINDOW = 0x3D,
    ACT_THS = 0x3E,
    ACT_DUR = 0x3F,
}

/// AOI-6D Interrupt mode
#[derive(Copy, Clone)]
pub enum Aoi6d {
    /// OR combination of interrupt events
    Or = 0b00,
    /// 6-direction movement recognition
    Movement6D = 0b01,
    /// AND combination of interrupt events
    And = 0b10,
    /// 6-direction position recognition
    Position6D = 0b11,
}

// === INT1_CFG (30h), INT2_CFG (34h) ===

pub const AOI_6D_MASK: u8 = 0b1100_0000;

pub const INT1_CFG_ENABLE_CLICK: u8 = 0b0011_1111;

impl Register {
    pub fn addr(self) -> u8 {
        self as u8
    }
}

// === CTRL_REG5 (24h) ===

pub const BOOT: u8 = 0b1000_0000;
pub const FIFO_EN: u8 = 0b0100_0000;
pub const LIR_INT1: u8 = 0b0000_1000;
pub const D4D_INT1: u8 = 0b0000_0100;
pub const LIR_INT2: u8 = 0b0000_0010;
pub const D4D_INT2: u8 = 0b0000_0001;

// === CTRL_REG6 (25h) ===

pub const I2_CLICK: u8 = 0b1000_0000;
pub const I2_IA1: u8 = 0b0100_0000;
pub const I2_IA2: u8 = 0b0010_0000;
pub const I2_BOOT: u8 = 0b0001_0000;
pub const I2_ACT: u8 = 0b0000_1000;
pub const INT_POLARITY: u8 = 0b0000_0010;

// === INT1_THS (32h), INT2_THS (36h) ===

pub const THS_MASK: u8 = 0b0111_1111;

// === CLICK_CFG (38h) ===

pub const ZD: u8 = 0b0010_0000;
pub const ZS: u8 = 0b0001_0000;
pub const YD: u8 = 0b0000_1000;
pub const YS: u8 = 0b0000_0100;
pub const XD: u8 = 0b0000_0010;
pub const XS: u8 = 0b0000_0001;

/// Output Data Rate
#[derive(Copy, Clone)]
#[cfg_attr(feature = "out_f32", derive(FromPrimitive))]
pub enum OutputDataRate {
    /// Power-down mode
    PowerDown = 0b0000,
    /// 1 Hz
    Hz1 = 0b0001,
    /// 10 Hz
    Hz10 = 0b0010,
    /// 25 Hz
    Hz25 = 0b0011,
    /// 50 Hz
    Hz50 = 0b0100,
    /// 100 Hz
    Hz100 = 0b0101,
    /// 200 Hz
    Hz200 = 0b0110,
    /// 400 Hz
    Hz400 = 0b0111,
    /// Low-power mode (1.620 kHz)
    HighRate0 = 0b1000,
    /// High-resolution / Normal (1.344 kHz),
    /// Low-power (5.376 kHz)
    HighRate1 = 0b1001,
}

/// WHO_AM_I device identification register
pub const DEVICE_ID: u8 = 0b0011_0011; // 51 (decimal)

// === CTRL_REG1 (20h) ===
pub const ODR_MASK: u8 = 0b1111_0000;

pub const LPen: u8 = 0b0000_1000;
pub const Zen: u8 = 0b0000_0100;
pub const Yen: u8 = 0b0000_0010;
pub const Xen: u8 = 0b0000_0001;

pub const HR: u8 = 0b0000_1000;

// === TEMP_CFG_REG (1Fh) ===

pub const TEMP_EN: u8 = 0b1100_0000;

// === CTRL_REG3 (22h) ===

pub const I1_CLICK: u8 = 0b1000_0000;
pub const I1_IA1: u8 = 0b0100_0000;
pub const I1_IA2: u8 = 0b0010_0000;
pub const I1_ZYXDA: u8 = 0b0001_0000;
pub const I1_WTM: u8 = 0b0000_0100;
pub const I1_OVERRUN: u8 = 0b0000_0010;

// === CTRL_REG4 (23h) ===

pub const BDU: u8 = 0b1000_0000;

pub const FS_MASK: u8 = 0b0011_0000;

#[derive(Copy, Clone)]
pub enum FullScaleSelection {
    PlusMinus2G = 0x00, // default
    PlusMinus4G = 0x01,
    PlusMinus8G = 0x02,
    PlusMinus16G = 0x03,
}

/// Data status structure,
/// decoded from STATUS_REG register
#[derive(Debug)]
pub struct DataStatus {
    /// ZYXOR bit
    pub zyxor: bool,
    /// (XOR, YOR, ZOR) bits
    pub xyzor: (bool, bool, bool),
    /// ZYXDA bit
    pub zyxda: bool,
    /// (XDA, YDA, ZDA) bits
    pub xyzda: (bool, bool, bool),
}

/// Operating mode
pub enum OperatingMode {
    /// High-resolution mode (12-bit data output)
    HighResolution,
    /// Normal mode (10-bit data output)
    Normal,
    /// Low-power mode (8-bit data output)
    LowPower,
}

// === STATUS_REG (27h) ===

pub const ZYXOR: u8 = 0b1000_0000;
pub const ZOR: u8 = 0b0100_0000;
pub const YOR: u8 = 0b0010_0000;
pub const XOR: u8 = 0b0001_0000;
pub const ZYXDA: u8 = 0b0000_1000;
pub const ZDA: u8 = 0b0000_0100;
pub const YDA: u8 = 0b0000_0010;
pub const XDA: u8 = 0b0000_0001;
