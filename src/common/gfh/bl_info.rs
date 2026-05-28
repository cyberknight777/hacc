use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes};

use super::header::GfhHeader;
use super::{Gfh, GfhError};
use crate::error::{Error, Result};
use crate::gfh::header::GfhType;
use crate::traits::TryRead;

#[derive(Debug, Immutable, IntoBytes, TryFromBytes, KnownLayout)]
#[repr(C)]
pub struct GfhBlInfo {
    header: GfhHeader,
    attr: u32,
}

impl GfhBlInfo {
    const SIZE: usize = size_of::<Self>();

    pub const fn attr(&self) -> u32 {
        self.attr
    }

    pub const fn set_attr(&mut self, attr: u32) {
        self.attr = attr;
    }
}

impl Gfh for GfhBlInfo {
    fn header(&self) -> &GfhHeader {
        &self.header
    }

    fn validate(&self) -> Result<()> {
        self.header().validate()?;

        if self.header().gfh_type() != GfhType::BlInfo {
            return Err(Error::Gfh(GfhError::InvalidType(
                GfhType::BlInfo,
                self.header().gfh_type(),
            )));
        }
        Ok(())
    }
}

impl<'a> TryRead<'a> for GfhBlInfo {
    fn try_read(data: &[u8]) -> Result<Self> {
        if data.len() < Self::SIZE {
            return Err(Error::Gfh(GfhError::TooShort(data.len(), Self::SIZE)));
        }

        let gfh = Self::try_read_from_bytes(&data[..Self::SIZE]).map_err(|_| Error::Zerocopy)?;

        gfh.validate()?;

        Ok(gfh)
    }
}
