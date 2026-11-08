#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mcpwm_th00: McpwmTh00,
    _reserved1: [u8; 0x02],
    mcpwm_th01: McpwmTh01,
    _reserved2: [u8; 0x02],
    mcpwm_th10: McpwmTh10,
    _reserved3: [u8; 0x02],
    mcpwm_th11: McpwmTh11,
    _reserved4: [u8; 0x02],
    mcpwm_th20: McpwmTh20,
    _reserved5: [u8; 0x02],
    mcpwm_th21: McpwmTh21,
    _reserved6: [u8; 0x02],
    mcpwm_th30: McpwmTh30,
    _reserved7: [u8; 0x02],
    mcpwm_th31: McpwmTh31,
    _reserved8: [u8; 0x02],
    mcpwm_tmr: (),
    _reserved9: [u8; 0x10],
    mcpwm_th0: McpwmTh0,
    mcpwm_th1: McpwmTh1,
    mcpwm_cnt0: McpwmCnt0,
    mcpwm_cnt1: McpwmCnt1,
    mcpwm_update: McpwmUpdate,
    mcpwm_fcnt: McpwmFcnt,
    mcpwm_evt0: McpwmEvt0,
    mcpwm_evt1: McpwmEvt1,
    mcpwm_dth0: McpwmDth0,
    mcpwm_dth1: McpwmDth1,
    _reserved19: [u8; 0x18],
    mcpwm_flt: McpwmFlt,
    mcpwm_sdcfg: McpwmSdcfg,
    mcpwm_auen: McpwmAuen,
    mcpwm_tclk: McpwmTclk,
    mcpwm_ie0: McpwmIe0,
    mcpwm_if0: McpwmIf0,
    mcpwm_ie1: McpwmIe1,
    mcpwm_if1: McpwmIf1,
    mcpwm_eie: McpwmEie,
    mcpwm_eif: McpwmEif,
    mcpwm_re: McpwmRe,
    mcpwm_pp: McpwmPp,
    mcpwm_io01: McpwmIo01,
    mcpwm_io23: McpwmIo23,
    mcpwm_fail012: McpwmFail012,
    mcpwm_fail3: McpwmFail3,
    mcpwm_prt: McpwmPrt,
    mcpwm_swap: McpwmSwap,
    mcpwm_ch_mask: McpwmChMask,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn mcpwm_th00(&self) -> &McpwmTh00 {
        &self.mcpwm_th00
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn mcpwm_th01(&self) -> &McpwmTh01 {
        &self.mcpwm_th01
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn mcpwm_th10(&self) -> &McpwmTh10 {
        &self.mcpwm_th10
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn mcpwm_th11(&self) -> &McpwmTh11 {
        &self.mcpwm_th11
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn mcpwm_th20(&self) -> &McpwmTh20 {
        &self.mcpwm_th20
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn mcpwm_th21(&self) -> &McpwmTh21 {
        &self.mcpwm_th21
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn mcpwm_th30(&self) -> &McpwmTh30 {
        &self.mcpwm_th30
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn mcpwm_th31(&self) -> &McpwmTh31 {
        &self.mcpwm_th31
    }
    #[doc = "0x20..0x28 - MCPWM Timer"]
    #[inline(always)]
    pub const fn mcpwm_tmr(&self, n: usize) -> &McpwmTmr {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(32)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x28 - MCPWM Timer"]
    #[inline(always)]
    pub fn mcpwm_tmr_iter(&self) -> impl Iterator<Item = &McpwmTmr> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(32)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn mcpwm_th0(&self) -> &McpwmTh0 {
        &self.mcpwm_th0
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn mcpwm_th1(&self) -> &McpwmTh1 {
        &self.mcpwm_th1
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn mcpwm_cnt0(&self) -> &McpwmCnt0 {
        &self.mcpwm_cnt0
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn mcpwm_cnt1(&self) -> &McpwmCnt1 {
        &self.mcpwm_cnt1
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn mcpwm_update(&self) -> &McpwmUpdate {
        &self.mcpwm_update
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn mcpwm_fcnt(&self) -> &McpwmFcnt {
        &self.mcpwm_fcnt
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn mcpwm_evt0(&self) -> &McpwmEvt0 {
        &self.mcpwm_evt0
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn mcpwm_evt1(&self) -> &McpwmEvt1 {
        &self.mcpwm_evt1
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn mcpwm_dth0(&self) -> &McpwmDth0 {
        &self.mcpwm_dth0
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn mcpwm_dth1(&self) -> &McpwmDth1 {
        &self.mcpwm_dth1
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn mcpwm_flt(&self) -> &McpwmFlt {
        &self.mcpwm_flt
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn mcpwm_sdcfg(&self) -> &McpwmSdcfg {
        &self.mcpwm_sdcfg
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn mcpwm_auen(&self) -> &McpwmAuen {
        &self.mcpwm_auen
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn mcpwm_tclk(&self) -> &McpwmTclk {
        &self.mcpwm_tclk
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn mcpwm_ie0(&self) -> &McpwmIe0 {
        &self.mcpwm_ie0
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn mcpwm_if0(&self) -> &McpwmIf0 {
        &self.mcpwm_if0
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn mcpwm_ie1(&self) -> &McpwmIe1 {
        &self.mcpwm_ie1
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn mcpwm_if1(&self) -> &McpwmIf1 {
        &self.mcpwm_if1
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn mcpwm_eie(&self) -> &McpwmEie {
        &self.mcpwm_eie
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn mcpwm_eif(&self) -> &McpwmEif {
        &self.mcpwm_eif
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn mcpwm_re(&self) -> &McpwmRe {
        &self.mcpwm_re
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn mcpwm_pp(&self) -> &McpwmPp {
        &self.mcpwm_pp
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn mcpwm_io01(&self) -> &McpwmIo01 {
        &self.mcpwm_io01
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn mcpwm_io23(&self) -> &McpwmIo23 {
        &self.mcpwm_io23
    }
    #[doc = "0xa8 - "]
    #[inline(always)]
    pub const fn mcpwm_fail012(&self) -> &McpwmFail012 {
        &self.mcpwm_fail012
    }
    #[doc = "0xac - "]
    #[inline(always)]
    pub const fn mcpwm_fail3(&self) -> &McpwmFail3 {
        &self.mcpwm_fail3
    }
    #[doc = "0xb0 - "]
    #[inline(always)]
    pub const fn mcpwm_prt(&self) -> &McpwmPrt {
        &self.mcpwm_prt
    }
    #[doc = "0xb4 - "]
    #[inline(always)]
    pub const fn mcpwm_swap(&self) -> &McpwmSwap {
        &self.mcpwm_swap
    }
    #[doc = "0xb8 - "]
    #[inline(always)]
    pub const fn mcpwm_ch_mask(&self) -> &McpwmChMask {
        &self.mcpwm_ch_mask
    }
}
#[doc = "MCPWM_TH00 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_th00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_th00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_th00`] module"]
#[doc(alias = "MCPWM_TH00")]
pub type McpwmTh00 = crate::Reg<mcpwm_th00::McpwmTh00Spec>;
#[doc = ""]
pub mod mcpwm_th00;
#[doc = "MCPWM_TH01 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_th01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_th01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_th01`] module"]
#[doc(alias = "MCPWM_TH01")]
pub type McpwmTh01 = crate::Reg<mcpwm_th01::McpwmTh01Spec>;
#[doc = ""]
pub mod mcpwm_th01;
#[doc = "MCPWM_TH10 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_th10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_th10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_th10`] module"]
#[doc(alias = "MCPWM_TH10")]
pub type McpwmTh10 = crate::Reg<mcpwm_th10::McpwmTh10Spec>;
#[doc = ""]
pub mod mcpwm_th10;
#[doc = "MCPWM_TH11 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_th11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_th11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_th11`] module"]
#[doc(alias = "MCPWM_TH11")]
pub type McpwmTh11 = crate::Reg<mcpwm_th11::McpwmTh11Spec>;
#[doc = ""]
pub mod mcpwm_th11;
#[doc = "MCPWM_TH20 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_th20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_th20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_th20`] module"]
#[doc(alias = "MCPWM_TH20")]
pub type McpwmTh20 = crate::Reg<mcpwm_th20::McpwmTh20Spec>;
#[doc = ""]
pub mod mcpwm_th20;
#[doc = "MCPWM_TH21 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_th21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_th21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_th21`] module"]
#[doc(alias = "MCPWM_TH21")]
pub type McpwmTh21 = crate::Reg<mcpwm_th21::McpwmTh21Spec>;
#[doc = ""]
pub mod mcpwm_th21;
#[doc = "MCPWM_TH30 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_th30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_th30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_th30`] module"]
#[doc(alias = "MCPWM_TH30")]
pub type McpwmTh30 = crate::Reg<mcpwm_th30::McpwmTh30Spec>;
#[doc = ""]
pub mod mcpwm_th30;
#[doc = "MCPWM_TH31 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_th31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_th31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_th31`] module"]
#[doc(alias = "MCPWM_TH31")]
pub type McpwmTh31 = crate::Reg<mcpwm_th31::McpwmTh31Spec>;
#[doc = ""]
pub mod mcpwm_th31;
#[doc = "MCPWM_TMR (rw) register accessor: MCPWM Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_tmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_tmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_tmr`] module"]
#[doc(alias = "MCPWM_TMR")]
pub type McpwmTmr = crate::Reg<mcpwm_tmr::McpwmTmrSpec>;
#[doc = "MCPWM Timer"]
pub mod mcpwm_tmr;
#[doc = "MCPWM_TH0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_th0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_th0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_th0`] module"]
#[doc(alias = "MCPWM_TH0")]
pub type McpwmTh0 = crate::Reg<mcpwm_th0::McpwmTh0Spec>;
#[doc = ""]
pub mod mcpwm_th0;
#[doc = "MCPWM_TH1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_th1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_th1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_th1`] module"]
#[doc(alias = "MCPWM_TH1")]
pub type McpwmTh1 = crate::Reg<mcpwm_th1::McpwmTh1Spec>;
#[doc = ""]
pub mod mcpwm_th1;
#[doc = "MCPWM_CNT0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_cnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_cnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_cnt0`] module"]
#[doc(alias = "MCPWM_CNT0")]
pub type McpwmCnt0 = crate::Reg<mcpwm_cnt0::McpwmCnt0Spec>;
#[doc = ""]
pub mod mcpwm_cnt0;
#[doc = "MCPWM_CNT1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_cnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_cnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_cnt1`] module"]
#[doc(alias = "MCPWM_CNT1")]
pub type McpwmCnt1 = crate::Reg<mcpwm_cnt1::McpwmCnt1Spec>;
#[doc = ""]
pub mod mcpwm_cnt1;
#[doc = "MCPWM_UPDATE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_update::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_update::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_update`] module"]
#[doc(alias = "MCPWM_UPDATE")]
pub type McpwmUpdate = crate::Reg<mcpwm_update::McpwmUpdateSpec>;
#[doc = ""]
pub mod mcpwm_update;
#[doc = "MCPWM_FCNT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_fcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_fcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_fcnt`] module"]
#[doc(alias = "MCPWM_FCNT")]
pub type McpwmFcnt = crate::Reg<mcpwm_fcnt::McpwmFcntSpec>;
#[doc = ""]
pub mod mcpwm_fcnt;
#[doc = "MCPWM_EVT0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_evt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_evt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_evt0`] module"]
#[doc(alias = "MCPWM_EVT0")]
pub type McpwmEvt0 = crate::Reg<mcpwm_evt0::McpwmEvt0Spec>;
#[doc = ""]
pub mod mcpwm_evt0;
#[doc = "MCPWM_EVT1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_evt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_evt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_evt1`] module"]
#[doc(alias = "MCPWM_EVT1")]
pub type McpwmEvt1 = crate::Reg<mcpwm_evt1::McpwmEvt1Spec>;
#[doc = ""]
pub mod mcpwm_evt1;
#[doc = "MCPWM_DTH0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_dth0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_dth0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_dth0`] module"]
#[doc(alias = "MCPWM_DTH0")]
pub type McpwmDth0 = crate::Reg<mcpwm_dth0::McpwmDth0Spec>;
#[doc = ""]
pub mod mcpwm_dth0;
#[doc = "MCPWM_DTH1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_dth1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_dth1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_dth1`] module"]
#[doc(alias = "MCPWM_DTH1")]
pub type McpwmDth1 = crate::Reg<mcpwm_dth1::McpwmDth1Spec>;
#[doc = ""]
pub mod mcpwm_dth1;
#[doc = "MCPWM_FLT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_flt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_flt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_flt`] module"]
#[doc(alias = "MCPWM_FLT")]
pub type McpwmFlt = crate::Reg<mcpwm_flt::McpwmFltSpec>;
#[doc = ""]
pub mod mcpwm_flt;
#[doc = "MCPWM_SDCFG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_sdcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_sdcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_sdcfg`] module"]
#[doc(alias = "MCPWM_SDCFG")]
pub type McpwmSdcfg = crate::Reg<mcpwm_sdcfg::McpwmSdcfgSpec>;
#[doc = ""]
pub mod mcpwm_sdcfg;
#[doc = "MCPWM_AUEN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_auen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_auen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_auen`] module"]
#[doc(alias = "MCPWM_AUEN")]
pub type McpwmAuen = crate::Reg<mcpwm_auen::McpwmAuenSpec>;
#[doc = ""]
pub mod mcpwm_auen;
#[doc = "MCPWM_TCLK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_tclk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_tclk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_tclk`] module"]
#[doc(alias = "MCPWM_TCLK")]
pub type McpwmTclk = crate::Reg<mcpwm_tclk::McpwmTclkSpec>;
#[doc = ""]
pub mod mcpwm_tclk;
#[doc = "MCPWM_IE0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_ie0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_ie0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_ie0`] module"]
#[doc(alias = "MCPWM_IE0")]
pub type McpwmIe0 = crate::Reg<mcpwm_ie0::McpwmIe0Spec>;
#[doc = ""]
pub mod mcpwm_ie0;
#[doc = "MCPWM_IF0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_if0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_if0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_if0`] module"]
#[doc(alias = "MCPWM_IF0")]
pub type McpwmIf0 = crate::Reg<mcpwm_if0::McpwmIf0Spec>;
#[doc = ""]
pub mod mcpwm_if0;
#[doc = "MCPWM_IE1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_ie1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_ie1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_ie1`] module"]
#[doc(alias = "MCPWM_IE1")]
pub type McpwmIe1 = crate::Reg<mcpwm_ie1::McpwmIe1Spec>;
#[doc = ""]
pub mod mcpwm_ie1;
#[doc = "MCPWM_IF1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_if1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_if1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_if1`] module"]
#[doc(alias = "MCPWM_IF1")]
pub type McpwmIf1 = crate::Reg<mcpwm_if1::McpwmIf1Spec>;
#[doc = ""]
pub mod mcpwm_if1;
#[doc = "MCPWM_EIE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_eie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_eie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_eie`] module"]
#[doc(alias = "MCPWM_EIE")]
pub type McpwmEie = crate::Reg<mcpwm_eie::McpwmEieSpec>;
#[doc = ""]
pub mod mcpwm_eie;
#[doc = "MCPWM_EIF (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_eif::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_eif::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_eif`] module"]
#[doc(alias = "MCPWM_EIF")]
pub type McpwmEif = crate::Reg<mcpwm_eif::McpwmEifSpec>;
#[doc = ""]
pub mod mcpwm_eif;
#[doc = "MCPWM_RE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_re::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_re::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_re`] module"]
#[doc(alias = "MCPWM_RE")]
pub type McpwmRe = crate::Reg<mcpwm_re::McpwmReSpec>;
#[doc = ""]
pub mod mcpwm_re;
#[doc = "MCPWM_PP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_pp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_pp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_pp`] module"]
#[doc(alias = "MCPWM_PP")]
pub type McpwmPp = crate::Reg<mcpwm_pp::McpwmPpSpec>;
#[doc = ""]
pub mod mcpwm_pp;
#[doc = "MCPWM_IO01 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_io01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_io01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_io01`] module"]
#[doc(alias = "MCPWM_IO01")]
pub type McpwmIo01 = crate::Reg<mcpwm_io01::McpwmIo01Spec>;
#[doc = ""]
pub mod mcpwm_io01;
#[doc = "MCPWM_IO23 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_io23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_io23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_io23`] module"]
#[doc(alias = "MCPWM_IO23")]
pub type McpwmIo23 = crate::Reg<mcpwm_io23::McpwmIo23Spec>;
#[doc = ""]
pub mod mcpwm_io23;
#[doc = "MCPWM_FAIL012 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_fail012::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_fail012::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_fail012`] module"]
#[doc(alias = "MCPWM_FAIL012")]
pub type McpwmFail012 = crate::Reg<mcpwm_fail012::McpwmFail012Spec>;
#[doc = ""]
pub mod mcpwm_fail012;
#[doc = "MCPWM_FAIL3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_fail3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_fail3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_fail3`] module"]
#[doc(alias = "MCPWM_FAIL3")]
pub type McpwmFail3 = crate::Reg<mcpwm_fail3::McpwmFail3Spec>;
#[doc = ""]
pub mod mcpwm_fail3;
#[doc = "MCPWM_PRT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_prt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_prt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_prt`] module"]
#[doc(alias = "MCPWM_PRT")]
pub type McpwmPrt = crate::Reg<mcpwm_prt::McpwmPrtSpec>;
#[doc = ""]
pub mod mcpwm_prt;
#[doc = "MCPWM_SWAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_swap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_swap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_swap`] module"]
#[doc(alias = "MCPWM_SWAP")]
pub type McpwmSwap = crate::Reg<mcpwm_swap::McpwmSwapSpec>;
#[doc = ""]
pub mod mcpwm_swap;
#[doc = "MCPWM_CH_MASK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_ch_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_ch_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm_ch_mask`] module"]
#[doc(alias = "MCPWM_CH_MASK")]
pub type McpwmChMask = crate::Reg<mcpwm_ch_mask::McpwmChMaskSpec>;
#[doc = ""]
pub mod mcpwm_ch_mask;
