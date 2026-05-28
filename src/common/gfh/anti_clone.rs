use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes};

use super::header::GfhHeader;
use super::{Gfh, GfhError};
use crate::error::{Error, Result};
use crate::gfh::header::GfhType;
use crate::traits::TryRead;

#[derive(Debug, Immutable, IntoBytes, TryFromBytes, KnownLayout)]
#[repr(C)]
pub struct GfhAntiClone {
    header: GfhHeader,
    ac_b2k: bool,
    ac_b2c: bool,
    _reserved: [u8; 2],
    ac_offset: u32,
    ac_length: u32,
}

impl GfhAntiClone {
    pub const fn ac_b2k(&self) -> bool {
        self.ac_b2k
    }

    pub const fn ac_b2c(&self) -> bool {
        self.ac_b2c
    }

    pub const fn ac_offset(&self) -> u32 {
        self.ac_offset
    }

    pub const fn ac_length(&self) -> u32 {
        self.ac_length
    }

    pub const fn set_ac_b2k(&mut self, enabled: bool) {
        self.ac_b2k = enabled;
    }

    pub const fn set_ac_b2c(&mut self, enabled: bool) {
        self.ac_b2c = enabled;
    }

    pub const fn set_ac_offset(&mut self, offset: u32) {
        self.ac_offset = offset;
    }

    pub const fn set_ac_length(&mut self, length: u32) {
        self.ac_length = length;
    }
}

impl Gfh for GfhAntiClone {
    fn header(&self) -> &GfhHeader {
        &self.header
    }

    fn validate(&self) -> Result<()> {
        self.header().validate()?;

        if self.header().gfh_type() != GfhType::AntiClone {
            return Err(Error::Gfh(GfhError::InvalidType(
                GfhType::AntiClone,
                self.header().gfh_type(),
            )));
        }
        Ok(())
    }
}

impl<'a> TryRead<'a> for GfhAntiClone {
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
