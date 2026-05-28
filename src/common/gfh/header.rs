use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes};

use super::GfhError;
use crate::error::{Error, Result};
use crate::traits::TryRead;

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Immutable, IntoBytes, TryFromBytes, KnownLayout)]
pub enum GfhType {
    FileInfo = 0x0000,
    BlInfo = 0x0001,
    AntiClone = 0x0002,
    BlSecKey = 0x0003,
    SctrlCert = 0x0004,
    ToolAuth = 0x0005,
    Reserved1 = 0x0006,
    BromCfg = 0x0007,
    BromSecCfg = 0x0008,
    Reserved2 = 0x0009,
    Reserved3 = 0x000A,
    RootCert = 0x000B,
    ExpChk = 0x000C,
    EppParam = 0x000D,
    ChipVer = 0x000E,
    Reserved4 = 0x000F,
    MdSecCfg = 0x0010,
    BlExtCfg = 0x0012,

    MultiSecCfg = 0x0060,

    EppInfo = 0x0100,
    EmiList = 0x0101,
    CmemIdInfo = 0x0102,
    CmemNorInfo = 0x0103,
    DspInfo = 0x0104,

    MauiInfo = 0x0200,
    MauiSec = 0x0201,
    MauiCodeKey = 0x0202,
    MauiSecureRoKey = 0x0203,
    MauiResourceKey = 0x0204,
    SecureRoInfo = 0x0205,
    DlPackageInfo = 0x0206,
    FlashInfo = 0x0207,
    MacrInfo = 0x0208,
    ArmBlInfo = 0x0209,
    EmmcBootingInfo = 0x020A,
    FotaInfo = 0x020B,
    CbrRecordInfo = 0x020C,
    ConfidentialBinInfo = 0x020D,
    CbrInfo = 0x020E,
    MbaInfo = 0x020F,
    BinaryLocation = 0x0210,

    BootCertCtrlContent = 0x0300,

    TypeNum,
    TypeEnd = 0xFFFF,
}

#[derive(Debug, Clone, Immutable, IntoBytes, TryFromBytes, KnownLayout)]
#[repr(C)]
pub struct GfhHeader {
    magic: u32,
    size: u16,
    gfh_type: GfhType,
}

impl GfhHeader {
    pub const MAGIC: u32 = 0x004D4D4D;
    const MAGIC_MASK: u32 = 0x00FFFFFF;

    pub const fn new(gfh_type: GfhType, size: u16) -> Self {
        Self { magic: Self::MAGIC, size, gfh_type }
    }

    pub const fn validate(&self) -> Result<()> {
        if (self.magic & Self::MAGIC_MASK) != Self::MAGIC {
            return Err(Error::Gfh(GfhError::InvalidHeaderMagic(Self::MAGIC, self.magic)));
        }

        Ok(())
    }

    pub const fn version(&self) -> u8 {
        (self.magic >> 24) as u8
    }

    pub const fn size(&self) -> u16 {
        self.size
    }

    pub const fn gfh_type(&self) -> GfhType {
        self.gfh_type
    }
}

impl<'a> TryRead<'a> for GfhHeader {
    fn try_read(data: &[u8]) -> Result<Self> {
        if data.len() < size_of::<Self>() {
            return Err(Error::Gfh(GfhError::TooShort(data.len(), size_of::<Self>())));
        }

        let header =
            Self::try_read_from_bytes(&data[..size_of::<Self>()]).map_err(|_| Error::Zerocopy)?;

        header.validate()?;

        Ok(header)
    }
}
