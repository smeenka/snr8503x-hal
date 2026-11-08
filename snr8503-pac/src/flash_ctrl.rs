#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    flash_cfg: FlashCfg,
    flash_addr: FlashAddr,
    flash_wdata: FlashWdata,
    flash_rdata: FlashRdata,
    flash_erase: FlashErase,
    flash_protect: FlashProtect,
    flash_ready: FlashReady,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn flash_cfg(&self) -> &FlashCfg {
        &self.flash_cfg
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn flash_addr(&self) -> &FlashAddr {
        &self.flash_addr
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn flash_wdata(&self) -> &FlashWdata {
        &self.flash_wdata
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn flash_rdata(&self) -> &FlashRdata {
        &self.flash_rdata
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn flash_erase(&self) -> &FlashErase {
        &self.flash_erase
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn flash_protect(&self) -> &FlashProtect {
        &self.flash_protect
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn flash_ready(&self) -> &FlashReady {
        &self.flash_ready
    }
}
#[doc = "FLASH_CFG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`flash_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_cfg`] module"]
#[doc(alias = "FLASH_CFG")]
pub type FlashCfg = crate::Reg<flash_cfg::FlashCfgSpec>;
#[doc = ""]
pub mod flash_cfg;
#[doc = "FLASH_ADDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`flash_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_addr`] module"]
#[doc(alias = "FLASH_ADDR")]
pub type FlashAddr = crate::Reg<flash_addr::FlashAddrSpec>;
#[doc = ""]
pub mod flash_addr;
#[doc = "FLASH_WDATA (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`flash_wdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_wdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_wdata`] module"]
#[doc(alias = "FLASH_WDATA")]
pub type FlashWdata = crate::Reg<flash_wdata::FlashWdataSpec>;
#[doc = ""]
pub mod flash_wdata;
#[doc = "FLASH_RDATA (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_rdata`] module"]
#[doc(alias = "FLASH_RDATA")]
pub type FlashRdata = crate::Reg<flash_rdata::FlashRdataSpec>;
#[doc = ""]
pub mod flash_rdata;
#[doc = "FLASH_ERASE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`flash_erase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_erase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_erase`] module"]
#[doc(alias = "FLASH_ERASE")]
pub type FlashErase = crate::Reg<flash_erase::FlashEraseSpec>;
#[doc = ""]
pub mod flash_erase;
#[doc = "FLASH_PROTECT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`flash_protect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_protect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_protect`] module"]
#[doc(alias = "FLASH_PROTECT")]
pub type FlashProtect = crate::Reg<flash_protect::FlashProtectSpec>;
#[doc = ""]
pub mod flash_protect;
#[doc = "FLASH_READY (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`flash_ready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_ready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_ready`] module"]
#[doc(alias = "FLASH_READY")]
pub type FlashReady = crate::Reg<flash_ready::FlashReadySpec>;
#[doc = ""]
pub mod flash_ready;
