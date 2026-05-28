use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes};

use super::header::GfhHeader;
use super::{Gfh, GfhError};
use crate::error::{Error, Result};
use crate::gfh::header::GfhType;
use crate::traits::TryRead;
use crate::{get_bit, set_or_clear_bit, set_or_clear_val};

#[derive(Debug, Immutable, IntoBytes, TryFromBytes, KnownLayout)]
#[repr(C)]
pub struct GfhBromCfgV3 {
    header: GfhHeader,
    attr: u32,
    usbdl_by_auto_detect_timeout_ms: u32,
    usbdl_abnormal_timeout_log: [u8; 64],
    usbdl_v1_method: u8,
    usbdl_drv_type: u8,
    usbdl_method: u8,
    brom_cmd_via_uart1_disable_magic: u8,
    brom_cmd_via_usb_disable_magic: u8,
    jump_bl_magic: u8,
    usbdl_hs_en: u8,
    pll_option: u8,
    usbdl_by_kcol0_timeout_ms: u32,
    usbdl_by_flag_timeout_ms: u32,
    usbdl_vid: u16,
    usbdl_pid: u16,
}

impl GfhBromCfgV3 {
    const SIZE: usize = size_of::<Self>();

    pub const fn header(&self) -> &GfhHeader {
        &self.header
    }

    pub const fn attr(&self) -> u32 {
        self.attr
    }

    pub const fn usbdl_by_auto_detect_timeout_ms(&self) -> u32 {
        self.usbdl_by_auto_detect_timeout_ms
    }

    pub const fn usbdl_abnormal_timeout_log(&self) -> &[u8; 64] {
        &self.usbdl_abnormal_timeout_log
    }

    pub const fn usbdl_v1_method(&self) -> u8 {
        self.usbdl_v1_method
    }

    pub const fn usbdl_drv_type(&self) -> u8 {
        self.usbdl_drv_type
    }

    pub const fn usbdl_method(&self) -> u8 {
        self.usbdl_method
    }

    pub const fn brom_cmd_via_uart1_disable_magic(&self) -> u8 {
        self.brom_cmd_via_uart1_disable_magic
    }

    pub const fn brom_cmd_via_usb_disable_magic(&self) -> u8 {
        self.brom_cmd_via_usb_disable_magic
    }

    pub const fn jump_bl_magic(&self) -> u8 {
        self.jump_bl_magic
    }

    pub const fn usbdl_hs_en(&self) -> u8 {
        self.usbdl_hs_en
    }

    pub const fn pll_option(&self) -> u8 {
        self.pll_option
    }

    pub const fn usbdl_by_kcol0_timeout_ms(&self) -> u32 {
        self.usbdl_by_kcol0_timeout_ms
    }

    pub const fn usbdl_by_flag_timeout_ms(&self) -> u32 {
        self.usbdl_by_flag_timeout_ms
    }

    pub const fn usbdl_vid(&self) -> u16 {
        self.usbdl_vid
    }

    pub const fn usbdl_pid(&self) -> u16 {
        self.usbdl_pid
    }

    pub const fn get_uart1_log_disabled(&self) -> bool {
        get_bit!(self.attr, 0) == 1
    }

    pub const fn get_abnormal_timeout_log_disabled(&self) -> bool {
        get_bit!(self.attr, 2) == 1
    }

    pub const fn get_abnormal_timeout_log_cust(&self) -> bool {
        get_bit!(self.attr, 3) == 1
    }

    pub const fn get_usbdl_auto_detect_disabled(&self) -> bool {
        get_bit!(self.attr, 4) == 1
    }

    pub const fn get_auto_detect_timeout_enabled(&self) -> bool {
        get_bit!(self.attr, 1) == 1
    }

    pub const fn get_cmd_via_uart_disabled(&self) -> bool {
        self.brom_cmd_via_uart1_disable_magic == 0x52
    }

    pub const fn get_cmd_via_usb_disabled(&self) -> bool {
        self.brom_cmd_via_usb_disable_magic == 0x55
    }

    pub const fn get_kcol0_timeout_enabled(&self) -> bool {
        get_bit!(self.attr, 7) == 1
    }

    pub const fn get_flag_timeout_enabled(&self) -> bool {
        get_bit!(self.attr, 8) == 1
    }

    pub const fn get_usbdl_custom_vid_pid_enabled(&self) -> bool {
        get_bit!(self.attr, 6) == 1
    }

    pub const fn get_jump_bl_aarch64_enabled(&self) -> bool {
        get_bit!(self.attr, 12) == 1
    }

    pub const fn set_uart1_log_disabled(&mut self, disabled: bool) {
        set_or_clear_bit!(self.attr, 0, disabled);
    }

    pub const fn set_abnormal_timeout_log_disabled(&mut self, disabled: bool) {
        set_or_clear_bit!(self.attr, 2, disabled);
    }

    pub const fn set_abnormal_timeout_log_cust(&mut self, enabled: bool) {
        set_or_clear_bit!(self.attr, 3, enabled);
    }

    pub const fn set_usbdl_auto_detect_disabled(&mut self, disabled: bool) {
        set_or_clear_bit!(self.attr, 4, disabled);
    }

    pub const fn set_speed_config(&mut self, config: u8) {
        set_or_clear_val!(self.attr, config as u32, config != 0);
    }

    pub const fn set_auto_detect_timeout_ms(&mut self, timeout: u32) {
        self.usbdl_by_auto_detect_timeout_ms = timeout;
        set_or_clear_bit!(self.attr, 1, timeout != 0);
    }

    pub fn set_abnormal_timeout_string(&mut self, log: &str) {
        let bytes = log.as_bytes();
        let len = bytes.len().min(64);
        self.usbdl_abnormal_timeout_log[..len].copy_from_slice(&bytes[..len]);
    }

    pub const fn set_cmd_via_uart_disabled(&mut self, disabled: bool) {
        self.brom_cmd_via_uart1_disable_magic = if disabled { 0x52 } else { 0 };
    }

    pub const fn set_cmd_via_usb_disabled(&mut self, disabled: bool) {
        self.brom_cmd_via_usb_disable_magic = if disabled { 0x55 } else { 0 };
    }

    pub const fn set_usbdl_hs_enabled(&mut self, enabled: bool) {
        self.usbdl_hs_en = enabled as u8;
    }

    pub const fn set_kcol0_timeout_ms(&mut self, timeout: u32) {
        self.usbdl_by_kcol0_timeout_ms = timeout;
        set_or_clear_bit!(self.attr, 7, timeout != 0);
    }

    pub const fn set_flag_timeout_ms(&mut self, timeout: u32) {
        self.usbdl_by_flag_timeout_ms = timeout;
        set_or_clear_bit!(self.attr, 8, timeout != 0);
    }

    pub const fn set_usbdl_vid(&mut self, vid: u16) {
        self.usbdl_vid = vid;
        set_or_clear_bit!(self.attr, 6, vid != 0);
    }

    pub const fn set_usbdl_pid(&mut self, pid: u16) {
        self.usbdl_pid = pid;
        set_or_clear_bit!(self.attr, 6, pid != 0);
    }

    pub const fn set_jump_bl_aarch64(&mut self, enabled: bool) {
        self.jump_bl_magic = if enabled { 0x64 } else { 0 };
        set_or_clear_bit!(self.attr, 12, enabled)
    }
}

impl Gfh for GfhBromCfgV3 {
    fn header(&self) -> &GfhHeader {
        &self.header
    }

    fn validate(&self) -> Result<()> {
        self.header().validate()?;

        if self.header().gfh_type() != GfhType::BromCfg {
            return Err(Error::Gfh(GfhError::InvalidType(
                GfhType::BromCfg,
                self.header().gfh_type(),
            )));
        }
        Ok(())
    }
}

impl<'a> TryRead<'a> for GfhBromCfgV3 {
    fn try_read(data: &[u8]) -> Result<Self> {
        if data.len() < Self::SIZE {
            return Err(Error::Gfh(GfhError::TooShort(data.len(), Self::SIZE)));
        }

        let gfh = Self::try_read_from_bytes(&data[..Self::SIZE]).map_err(|_| Error::Zerocopy)?;

        gfh.validate()?;

        Ok(gfh)
    }
}

#[derive(Immutable, IntoBytes, TryFromBytes, KnownLayout)]
#[repr(C)]
pub struct GfhBromCfgV4 {
    header: GfhHeader,
    attr: u32,
    usbdl_by_auto_detect_timeout_ms: u32,
    usbdl_abnormal_timeout_log: [u8; 64],
    usbdl_v1_method: u8,
    usbdl_drv_type: u8,
    usbdl_method: u8,
    brom_cmd_via_uart1_disable_magic: u8,
    brom_cmd_via_usb_disable_magic: u8,
    jump_bl_magic: u8,
    usbdl_hs_en: u8,
    pll_option: u8,
    usbdl_by_kcol0_timeout_ms: u32,
    usbdl_by_flag_timeout_ms: u32,
    usbdl_vid: u16,
    usbdl_pid: u16,
    pciedl_by_auto_detect_timeout_ms: u32,
    pciedl_abnormal_timeout_log: [u8; 64],
    pciedl_by_kcol0_timeout_ms: u32,
    pciedl_by_flag_timeout_ms: u32,
    pciedl_link_timeout_ms: u32,
    pciedl_method: u8,
    brom_cmd_via_pcie_disable_magic: u8,
    pciedl_pll_option: u8,
    pciedl_rid: u8,
    pciedl_vid: u16,
    pciedl_did: u16,
    pciedl_svid: u16,
    pciedl_sdid: u16,
    pciedl_cid: u32,
    pciedl_sku_low: u32,
    pciedl_sku_high: u32,
}

impl GfhBromCfgV4 {
    const SIZE: usize = size_of::<Self>();

    pub const fn header(&self) -> &GfhHeader {
        &self.header
    }

    pub const fn attr(&self) -> u32 {
        self.attr
    }

    pub const fn usbdl_by_auto_detect_timeout_ms(&self) -> u32 {
        self.usbdl_by_auto_detect_timeout_ms
    }

    pub const fn usbdl_abnormal_timeout_log(&self) -> &[u8; 64] {
        &self.usbdl_abnormal_timeout_log
    }

    pub const fn usbdl_v1_method(&self) -> u8 {
        self.usbdl_v1_method
    }

    pub const fn usbdl_drv_type(&self) -> u8 {
        self.usbdl_drv_type
    }

    pub const fn usbdl_method(&self) -> u8 {
        self.usbdl_method
    }

    pub const fn brom_cmd_via_uart1_disable_magic(&self) -> u8 {
        self.brom_cmd_via_uart1_disable_magic
    }

    pub const fn brom_cmd_via_usb_disable_magic(&self) -> u8 {
        self.brom_cmd_via_usb_disable_magic
    }

    pub const fn jump_bl_magic(&self) -> u8 {
        self.jump_bl_magic
    }

    pub const fn usbdl_hs_en(&self) -> u8 {
        self.usbdl_hs_en
    }

    pub const fn pll_option(&self) -> u8 {
        self.pll_option
    }

    pub const fn usbdl_by_kcol0_timeout_ms(&self) -> u32 {
        self.usbdl_by_kcol0_timeout_ms
    }

    pub const fn usbdl_by_flag_timeout_ms(&self) -> u32 {
        self.usbdl_by_flag_timeout_ms
    }

    pub const fn usbdl_vid(&self) -> u16 {
        self.usbdl_vid
    }

    pub const fn usbdl_pid(&self) -> u16 {
        self.usbdl_pid
    }

    pub const fn pciedl_by_auto_detect_timeout_ms(&self) -> u32 {
        self.pciedl_by_auto_detect_timeout_ms
    }

    pub const fn pciedl_abnormal_timeout_log(&self) -> &[u8; 64] {
        &self.pciedl_abnormal_timeout_log
    }

    pub const fn pciedl_by_kcol0_timeout_ms(&self) -> u32 {
        self.pciedl_by_kcol0_timeout_ms
    }

    pub const fn pciedl_by_flag_timeout_ms(&self) -> u32 {
        self.pciedl_by_flag_timeout_ms
    }

    pub const fn pciedl_link_timeout_ms(&self) -> u32 {
        self.pciedl_link_timeout_ms
    }

    pub const fn pciedl_method(&self) -> u8 {
        self.pciedl_method
    }

    pub const fn brom_cmd_via_pcie_disable_magic(&self) -> u8 {
        self.brom_cmd_via_pcie_disable_magic
    }

    pub const fn pciedl_pll_option(&self) -> u8 {
        self.pciedl_pll_option
    }

    pub const fn pciedl_rid(&self) -> u8 {
        self.pciedl_rid
    }

    pub const fn pciedl_vid(&self) -> u16 {
        self.pciedl_vid
    }

    pub const fn pciedl_did(&self) -> u16 {
        self.pciedl_did
    }

    pub const fn pciedl_svid(&self) -> u16 {
        self.pciedl_svid
    }

    pub const fn pciedl_sdid(&self) -> u16 {
        self.pciedl_sdid
    }

    pub const fn pciedl_cid(&self) -> u32 {
        self.pciedl_cid
    }

    pub const fn pciedl_sku_low(&self) -> u32 {
        self.pciedl_sku_low
    }

    pub const fn pciedl_sku_high(&self) -> u32 {
        self.pciedl_sku_high
    }

    pub const fn get_uart1_log_disabled(&self) -> bool {
        get_bit!(self.attr, 0) == 1
    }

    pub const fn get_abnormal_timeout_log_disabled(&self) -> bool {
        get_bit!(self.attr, 2) == 1
    }

    pub const fn get_abnormal_timeout_log_cust(&self) -> bool {
        get_bit!(self.attr, 3) == 1
    }

    pub const fn get_usbdl_auto_detect_disabled(&self) -> bool {
        get_bit!(self.attr, 4) == 1
    }

    pub const fn get_auto_detect_timeout_enabled(&self) -> bool {
        get_bit!(self.attr, 1) == 1
    }

    pub const fn get_cmd_via_uart_disabled(&self) -> bool {
        self.brom_cmd_via_uart1_disable_magic == 0x52
    }

    pub const fn get_cmd_via_usb_disabled(&self) -> bool {
        self.brom_cmd_via_usb_disable_magic == 0x55
    }

    pub const fn get_kcol0_timeout_enabled(&self) -> bool {
        get_bit!(self.attr, 7) == 1
    }

    pub const fn get_flag_timeout_enabled(&self) -> bool {
        get_bit!(self.attr, 8) == 1
    }

    pub const fn get_usbdl_custom_vid_pid_enabled(&self) -> bool {
        get_bit!(self.attr, 6) == 1
    }

    pub const fn get_jump_bl_aarch64_enabled(&self) -> bool {
        get_bit!(self.attr, 12) == 1
    }

    pub const fn get_speed_config(&self) -> u8 {
        ((self.attr >> 10) & 0x1) as u8
    }

    pub const fn get_usbdl_speed_config(&self) -> u8 {
        ((self.attr >> 10) & 0x1) as u8
    }

    pub const fn get_pciedl_abnormal_timeout_log_disabled(&self) -> bool {
        get_bit!(self.attr, 17) == 1
    }

    pub const fn get_pciedl_abnormal_timeout_log_cust(&self) -> bool {
        get_bit!(self.attr, 18) == 1
    }

    pub const fn get_pciedl_auto_detect_disabled(&self) -> bool {
        get_bit!(self.attr, 19) == 1
    }

    pub const fn get_pciedl_set_flag_enabled(&self) -> bool {
        get_bit!(self.attr, 28) == 1
    }

    pub const fn get_pciedl_auto_detect_timeout_enabled(&self) -> bool {
        get_bit!(self.attr, 16) == 1
    }

    pub const fn get_pciedl_kcol0_timeout_enabled(&self) -> bool {
        get_bit!(self.attr, 24) == 1
    }

    pub const fn get_pciedl_flag_timeout_enabled(&self) -> bool {
        get_bit!(self.attr, 25) == 1
    }

    pub const fn get_pciedl_link_timeout_enabled(&self) -> bool {
        get_bit!(self.attr, 26) == 1
    }

    pub const fn get_cmd_via_pcie_disabled(&self) -> bool {
        self.brom_cmd_via_pcie_disable_magic == 0x50
    }

    pub const fn get_pciedl_custom_vid_did_enabled(&self) -> bool {
        get_bit!(self.attr, 20) == 1
    }

    pub const fn get_pciedl_custom_svid_sdid_enabled(&self) -> bool {
        get_bit!(self.attr, 21) == 1
    }

    pub const fn get_pciedl_custom_rid_cid_enabled(&self) -> bool {
        get_bit!(self.attr, 22) == 1
    }

    pub const fn get_pciedl_custom_sku_enabled(&self) -> bool {
        get_bit!(self.attr, 23) == 1
    }

    pub const fn set_uart1_log_disabled(&mut self, disabled: bool) {
        set_or_clear_bit!(self.attr, 0, disabled);
    }

    pub const fn set_abnormal_timeout_log_disabled(&mut self, disabled: bool) {
        set_or_clear_bit!(self.attr, 2, disabled);
    }

    pub const fn set_abnormal_timeout_log_cust(&mut self, enabled: bool) {
        set_or_clear_bit!(self.attr, 3, enabled);
    }

    pub const fn set_usbdl_auto_detect_disabled(&mut self, disabled: bool) {
        set_or_clear_bit!(self.attr, 4, disabled);
    }

    pub const fn set_speed_config(&mut self, config: u8) {
        set_or_clear_val!(self.attr, config as u32, config != 0);
    }

    pub const fn set_auto_detect_timeout_ms(&mut self, timeout: u32) {
        self.usbdl_by_auto_detect_timeout_ms = timeout;
        set_or_clear_bit!(self.attr, 1, timeout != 0);
    }

    pub fn set_abnormal_timeout_string(&mut self, log: &str) {
        let bytes = log.as_bytes();
        let len = bytes.len().min(64);
        self.usbdl_abnormal_timeout_log[..len].copy_from_slice(&bytes[..len]);
    }

    pub const fn set_cmd_via_uart_disabled(&mut self, disabled: bool) {
        self.brom_cmd_via_uart1_disable_magic = if disabled { 0x52 } else { 0 };
    }

    pub const fn set_cmd_via_usb_disabled(&mut self, disabled: bool) {
        self.brom_cmd_via_usb_disable_magic = if disabled { 0x55 } else { 0 };
    }

    pub const fn set_usbdl_hs_enabled(&mut self, enabled: bool) {
        self.usbdl_hs_en = enabled as u8;
    }

    pub const fn set_kcol0_timeout_ms(&mut self, timeout: u32) {
        self.usbdl_by_kcol0_timeout_ms = timeout;
        set_or_clear_bit!(self.attr, 7, timeout != 0);
    }

    pub const fn set_flag_timeout_ms(&mut self, timeout: u32) {
        self.usbdl_by_flag_timeout_ms = timeout;
        set_or_clear_bit!(self.attr, 8, timeout != 0);
    }

    pub const fn set_usbdl_vid(&mut self, vid: u16) {
        self.usbdl_vid = vid;
        set_or_clear_bit!(self.attr, 6, vid != 0);
    }

    pub const fn set_usbdl_pid(&mut self, pid: u16) {
        self.usbdl_pid = pid;
        set_or_clear_bit!(self.attr, 6, pid != 0);
    }

    pub const fn set_jump_bl_aarch64(&mut self, enabled: bool) {
        self.jump_bl_magic = if enabled { 0x64 } else { 0 };
        set_or_clear_bit!(self.attr, 12, enabled)
    }

    pub const fn set_pciedl_abnormal_timeout_log_disabled(&mut self, disabled: bool) {
        set_or_clear_bit!(self.attr, 17, disabled);
    }

    pub const fn set_pciedl_abnormal_timeout_log_cust(&mut self, enabled: bool) {
        set_or_clear_bit!(self.attr, 18, enabled);
    }

    pub const fn set_pciedl_auto_detect_disabled(&mut self, disabled: bool) {
        set_or_clear_bit!(self.attr, 19, disabled);
    }

    pub const fn set_pciedl_set_flag_enabled(&mut self, enabled: bool) {
        set_or_clear_bit!(self.attr, 28, enabled);
    }

    pub const fn set_pciedl_auto_detect_timeout_ms(&mut self, timeout: u32) {
        self.pciedl_by_auto_detect_timeout_ms = timeout;
        set_or_clear_bit!(self.attr, 16, timeout != 0);
    }

    pub fn set_pciedl_abnormal_timeout_string(&mut self, log: &str) {
        let bytes = log.as_bytes();
        let len = bytes.len().min(64);
        self.pciedl_abnormal_timeout_log[..len].copy_from_slice(&bytes[..len]);
    }

    pub const fn set_pciedl_kcol0_timeout_ms(&mut self, timeout: u32) {
        self.pciedl_by_kcol0_timeout_ms = timeout;
        set_or_clear_bit!(self.attr, 24, timeout != 0);
    }

    pub const fn set_pciedl_flag_timeout_ms(&mut self, timeout: u32) {
        self.pciedl_by_flag_timeout_ms = timeout;
        set_or_clear_bit!(self.attr, 25, timeout != 0);
    }

    pub const fn set_pciedl_link_timeout_ms(&mut self, timeout: u32) {
        self.pciedl_link_timeout_ms = timeout;
        set_or_clear_bit!(self.attr, 26, timeout != 0);
    }

    pub const fn set_cmd_via_pcie_disabled(&mut self, disabled: bool) {
        self.brom_cmd_via_pcie_disable_magic = if disabled { 0x50 } else { 0 };
    }

    pub const fn set_pciedl_vid(&mut self, vid: u16) {
        self.pciedl_vid = vid;
        set_or_clear_bit!(self.attr, 20, vid != 0);
    }

    pub const fn set_pciedl_did(&mut self, did: u16) {
        self.pciedl_did = did;
        set_or_clear_bit!(self.attr, 20, did != 0);
    }

    pub const fn set_pciedl_svid(&mut self, svid: u16) {
        self.pciedl_svid = svid;
        set_or_clear_bit!(self.attr, 21, svid != 0);
    }

    pub const fn set_pciedl_sdid(&mut self, sdid: u16) {
        self.pciedl_sdid = sdid;
        set_or_clear_bit!(self.attr, 21, sdid != 0);
    }

    pub const fn set_pciedl_rid(&mut self, rid: u8) {
        self.pciedl_rid = rid;
        set_or_clear_bit!(self.attr, 22, rid != 0);
    }

    pub const fn set_pciedl_cid(&mut self, cid: u32) {
        self.pciedl_cid = cid;
        set_or_clear_bit!(self.attr, 22, cid != 0);
    }

    pub const fn set_pciedl_sku(&mut self, sku: u64) {
        self.pciedl_sku_low = sku as u32;
        self.pciedl_sku_high = (sku >> 32) as u32;
        set_or_clear_bit!(self.attr, 23, sku != 0);
    }
}

impl Gfh for GfhBromCfgV4 {
    fn header(&self) -> &GfhHeader {
        &self.header
    }

    fn validate(&self) -> Result<()> {
        self.header().validate()?;

        if self.header().gfh_type() != GfhType::BromCfg {
            return Err(Error::Gfh(GfhError::InvalidType(
                GfhType::BromCfg,
                self.header().gfh_type(),
            )));
        }

        Ok(())
    }
}

impl<'a> TryRead<'a> for GfhBromCfgV4 {
    fn try_read(data: &[u8]) -> Result<Self> {
        if data.len() < Self::SIZE {
            return Err(Error::Gfh(GfhError::TooShort(data.len(), Self::SIZE)));
        }

        let gfh = Self::try_read_from_bytes(&data[..Self::SIZE]).map_err(|_| Error::Zerocopy)?;

        gfh.validate()?;

        Ok(gfh)
    }
}
