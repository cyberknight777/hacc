use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes};

use super::header::GfhHeader;
use super::{Gfh, GfhError};
use crate::error::{Error, Result};
use crate::gfh::header::GfhType;
use crate::traits::TryRead;

#[derive(Debug, Immutable, IntoBytes, TryFromBytes, KnownLayout)]
#[repr(C)]
pub struct GfhBlExtCfg {
    header: GfhHeader,
    attr: u32,
    bl_attr: u32,
    offset: u32,
}

impl GfhBlExtCfg {
    pub const fn attr(&self) -> u32 {
        self.attr
    }

    pub const fn bl_attr(&self) -> u32 {
        self.bl_attr
    }

    pub const fn offset(&self) -> u32 {
        self.offset
    }

    pub const fn set_offset(&mut self, offset: u32) {
        self.offset = offset;
    }

    pub const fn set_attr(&mut self, attr: u32) {
        self.attr = attr;
    }

    pub const fn set_bl_attr(&mut self, bl_attr: u32) {
        self.bl_attr = bl_attr;
    }
}

impl Gfh for GfhBlExtCfg {
    fn header(&self) -> &GfhHeader {
        &self.header
    }

    fn validate(&self) -> Result<()> {
        self.header().validate()?;

        if self.header().gfh_type() != GfhType::BlExtCfg {
            return Err(Error::Gfh(GfhError::InvalidType(
                GfhType::BlExtCfg,
                self.header().gfh_type(),
            )));
        }
        Ok(())
    }
}

impl<'a> TryRead<'a> for GfhBlExtCfg {
    fn try_read(data: &[u8]) -> Result<Self> {
        if data.len() < size_of::<Self>() {
            return Err(Error::Gfh(GfhError::TooShort(data.len(), size_of::<Self>())));
        }

        let gfh =
            Self::try_read_from_bytes(&data[..size_of::<Self>()]).map_err(|_| Error::Zerocopy)?;

        gfh.validate()?;

        Ok(gfh)
    }
}
