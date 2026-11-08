#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hall_cfg: HallCfg,
    hall_info: HallInfo,
    hall_width: HallWidth,
    hall_th: HallTh,
    hall_cnt: HallCnt,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn hall_cfg(&self) -> &HallCfg {
        &self.hall_cfg
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn hall_info(&self) -> &HallInfo {
        &self.hall_info
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn hall_width(&self) -> &HallWidth {
        &self.hall_width
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn hall_th(&self) -> &HallTh {
        &self.hall_th
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn hall_cnt(&self) -> &HallCnt {
        &self.hall_cnt
    }
}
#[doc = "HALL_CFG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hall_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hall_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hall_cfg`] module"]
#[doc(alias = "HALL_CFG")]
pub type HallCfg = crate::Reg<hall_cfg::HallCfgSpec>;
#[doc = ""]
pub mod hall_cfg;
#[doc = "HALL_INFO (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hall_info::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hall_info::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hall_info`] module"]
#[doc(alias = "HALL_INFO")]
pub type HallInfo = crate::Reg<hall_info::HallInfoSpec>;
#[doc = ""]
pub mod hall_info;
#[doc = "HALL_WIDTH (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hall_width::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hall_width::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hall_width`] module"]
#[doc(alias = "HALL_WIDTH")]
pub type HallWidth = crate::Reg<hall_width::HallWidthSpec>;
#[doc = ""]
pub mod hall_width;
#[doc = "HALL_TH (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hall_th::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hall_th::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hall_th`] module"]
#[doc(alias = "HALL_TH")]
pub type HallTh = crate::Reg<hall_th::HallThSpec>;
#[doc = ""]
pub mod hall_th;
#[doc = "HALL_CNT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hall_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hall_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hall_cnt`] module"]
#[doc(alias = "HALL_CNT")]
pub type HallCnt = crate::Reg<hall_cnt::HallCntSpec>;
#[doc = ""]
pub mod hall_cnt;
