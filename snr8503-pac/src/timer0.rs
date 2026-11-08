#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    utimer_cfg: UtimerCfg,
    utimer_th: UtimerTh,
    utimer_cnt: UtimerCnt,
    utimer_cmp0: UtimerCmp0,
    utimer_cmp1: UtimerCmp1,
    utimer_evt: UtimerEvt,
    utimer_flt: UtimerFlt,
    utimer_ie: UtimerIe,
    utimer_if: UtimerIf,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn utimer_cfg(&self) -> &UtimerCfg {
        &self.utimer_cfg
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn utimer_th(&self) -> &UtimerTh {
        &self.utimer_th
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn utimer_cnt(&self) -> &UtimerCnt {
        &self.utimer_cnt
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn utimer_cmp0(&self) -> &UtimerCmp0 {
        &self.utimer_cmp0
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn utimer_cmp1(&self) -> &UtimerCmp1 {
        &self.utimer_cmp1
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn utimer_evt(&self) -> &UtimerEvt {
        &self.utimer_evt
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn utimer_flt(&self) -> &UtimerFlt {
        &self.utimer_flt
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn utimer_ie(&self) -> &UtimerIe {
        &self.utimer_ie
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn utimer_if(&self) -> &UtimerIf {
        &self.utimer_if
    }
}
#[doc = "UTIMER_CFG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`utimer_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utimer_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@utimer_cfg`] module"]
#[doc(alias = "UTIMER_CFG")]
pub type UtimerCfg = crate::Reg<utimer_cfg::UtimerCfgSpec>;
#[doc = ""]
pub mod utimer_cfg;
#[doc = "UTIMER_TH (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`utimer_th::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utimer_th::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@utimer_th`] module"]
#[doc(alias = "UTIMER_TH")]
pub type UtimerTh = crate::Reg<utimer_th::UtimerThSpec>;
#[doc = ""]
pub mod utimer_th;
#[doc = "UTIMER_CNT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`utimer_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utimer_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@utimer_cnt`] module"]
#[doc(alias = "UTIMER_CNT")]
pub type UtimerCnt = crate::Reg<utimer_cnt::UtimerCntSpec>;
#[doc = ""]
pub mod utimer_cnt;
#[doc = "UTIMER_CMP0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`utimer_cmp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utimer_cmp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@utimer_cmp0`] module"]
#[doc(alias = "UTIMER_CMP0")]
pub type UtimerCmp0 = crate::Reg<utimer_cmp0::UtimerCmp0Spec>;
#[doc = ""]
pub mod utimer_cmp0;
#[doc = "UTIMER_CMP1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`utimer_cmp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utimer_cmp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@utimer_cmp1`] module"]
#[doc(alias = "UTIMER_CMP1")]
pub type UtimerCmp1 = crate::Reg<utimer_cmp1::UtimerCmp1Spec>;
#[doc = ""]
pub mod utimer_cmp1;
#[doc = "UTIMER_EVT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`utimer_evt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utimer_evt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@utimer_evt`] module"]
#[doc(alias = "UTIMER_EVT")]
pub type UtimerEvt = crate::Reg<utimer_evt::UtimerEvtSpec>;
#[doc = ""]
pub mod utimer_evt;
#[doc = "UTIMER_FLT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`utimer_flt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utimer_flt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@utimer_flt`] module"]
#[doc(alias = "UTIMER_FLT")]
pub type UtimerFlt = crate::Reg<utimer_flt::UtimerFltSpec>;
#[doc = ""]
pub mod utimer_flt;
#[doc = "UTIMER_IE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`utimer_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utimer_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@utimer_ie`] module"]
#[doc(alias = "UTIMER_IE")]
pub type UtimerIe = crate::Reg<utimer_ie::UtimerIeSpec>;
#[doc = ""]
pub mod utimer_ie;
#[doc = "UTIMER_IF (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`utimer_if::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utimer_if::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@utimer_if`] module"]
#[doc(alias = "UTIMER_IF")]
pub type UtimerIf = crate::Reg<utimer_if::UtimerIfSpec>;
#[doc = ""]
pub mod utimer_if;
