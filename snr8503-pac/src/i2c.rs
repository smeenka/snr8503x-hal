#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    i2c_addr: I2cAddr,
    i2c_cfg: I2cCfg,
    i2c_scr: I2cScr,
    i2c_data: I2cData,
    i2c_mscr: I2cMscr,
    i2c_bcr: I2cBcr,
    i2c_bsize: I2cBsize,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn i2c_addr(&self) -> &I2cAddr {
        &self.i2c_addr
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn i2c_cfg(&self) -> &I2cCfg {
        &self.i2c_cfg
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn i2c_scr(&self) -> &I2cScr {
        &self.i2c_scr
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn i2c_data(&self) -> &I2cData {
        &self.i2c_data
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn i2c_mscr(&self) -> &I2cMscr {
        &self.i2c_mscr
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn i2c_bcr(&self) -> &I2cBcr {
        &self.i2c_bcr
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn i2c_bsize(&self) -> &I2cBsize {
        &self.i2c_bsize
    }
}
#[doc = "I2C_ADDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_addr`] module"]
#[doc(alias = "I2C_ADDR")]
pub type I2cAddr = crate::Reg<i2c_addr::I2cAddrSpec>;
#[doc = ""]
pub mod i2c_addr;
#[doc = "I2C_CFG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_cfg`] module"]
#[doc(alias = "I2C_CFG")]
pub type I2cCfg = crate::Reg<i2c_cfg::I2cCfgSpec>;
#[doc = ""]
pub mod i2c_cfg;
#[doc = "I2C_SCR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_scr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_scr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_scr`] module"]
#[doc(alias = "I2C_SCR")]
pub type I2cScr = crate::Reg<i2c_scr::I2cScrSpec>;
#[doc = ""]
pub mod i2c_scr;
#[doc = "I2C_DATA (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_data`] module"]
#[doc(alias = "I2C_DATA")]
pub type I2cData = crate::Reg<i2c_data::I2cDataSpec>;
#[doc = ""]
pub mod i2c_data;
#[doc = "I2C_MSCR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_mscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_mscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_mscr`] module"]
#[doc(alias = "I2C_MSCR")]
pub type I2cMscr = crate::Reg<i2c_mscr::I2cMscrSpec>;
#[doc = ""]
pub mod i2c_mscr;
#[doc = "I2C_BCR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_bcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_bcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_bcr`] module"]
#[doc(alias = "I2C_BCR")]
pub type I2cBcr = crate::Reg<i2c_bcr::I2cBcrSpec>;
#[doc = ""]
pub mod i2c_bcr;
#[doc = "I2C_BSIZE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_bsize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_bsize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_bsize`] module"]
#[doc(alias = "I2C_BSIZE")]
pub type I2cBsize = crate::Reg<i2c_bsize::I2cBsizeSpec>;
#[doc = ""]
pub mod i2c_bsize;
