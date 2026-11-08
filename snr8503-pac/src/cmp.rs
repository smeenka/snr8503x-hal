#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cmp_ie: CmpIe,
    cmp_if: CmpIf,
    cmp_tclk: CmpTclk,
    cmp_cfg: CmpCfg,
    cmp_blcwin: CmpBlcwin,
    cmp_data: CmpData,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn cmp_ie(&self) -> &CmpIe {
        &self.cmp_ie
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn cmp_if(&self) -> &CmpIf {
        &self.cmp_if
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn cmp_tclk(&self) -> &CmpTclk {
        &self.cmp_tclk
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn cmp_cfg(&self) -> &CmpCfg {
        &self.cmp_cfg
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn cmp_blcwin(&self) -> &CmpBlcwin {
        &self.cmp_blcwin
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn cmp_data(&self) -> &CmpData {
        &self.cmp_data
    }
}
#[doc = "CMP_IE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cmp_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp_ie`] module"]
#[doc(alias = "CMP_IE")]
pub type CmpIe = crate::Reg<cmp_ie::CmpIeSpec>;
#[doc = ""]
pub mod cmp_ie;
#[doc = "CMP_IF (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cmp_if::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp_if::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp_if`] module"]
#[doc(alias = "CMP_IF")]
pub type CmpIf = crate::Reg<cmp_if::CmpIfSpec>;
#[doc = ""]
pub mod cmp_if;
#[doc = "CMP_TCLK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cmp_tclk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp_tclk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp_tclk`] module"]
#[doc(alias = "CMP_TCLK")]
pub type CmpTclk = crate::Reg<cmp_tclk::CmpTclkSpec>;
#[doc = ""]
pub mod cmp_tclk;
#[doc = "CMP_CFG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cmp_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp_cfg`] module"]
#[doc(alias = "CMP_CFG")]
pub type CmpCfg = crate::Reg<cmp_cfg::CmpCfgSpec>;
#[doc = ""]
pub mod cmp_cfg;
#[doc = "CMP_BLCWIN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cmp_blcwin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp_blcwin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp_blcwin`] module"]
#[doc(alias = "CMP_BLCWIN")]
pub type CmpBlcwin = crate::Reg<cmp_blcwin::CmpBlcwinSpec>;
#[doc = ""]
pub mod cmp_blcwin;
#[doc = "CMP_DATA (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cmp_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp_data`] module"]
#[doc(alias = "CMP_DATA")]
pub type CmpData = crate::Reg<cmp_data::CmpDataSpec>;
#[doc = ""]
pub mod cmp_data;
