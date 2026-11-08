#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    iwdg_psw: IwdgPsw,
    iwdg_cfg: IwdgCfg,
    iwdg_clr: IwdgClr,
    iwdg_wth: IwdgWth,
    iwdg_rth: IwdgRth,
    iwdg_cnt: IwdgCnt,
    _reserved6: [u8; 0x08],
    aon_pwr_cfg: AonPwrCfg,
    aon_evt_rcd: AonEvtRcd,
    aon_io_wake_pol: AonIoWakePol,
    aon_io_wake_en: AonIoWakeEn,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn iwdg_psw(&self) -> &IwdgPsw {
        &self.iwdg_psw
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn iwdg_cfg(&self) -> &IwdgCfg {
        &self.iwdg_cfg
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn iwdg_clr(&self) -> &IwdgClr {
        &self.iwdg_clr
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn iwdg_wth(&self) -> &IwdgWth {
        &self.iwdg_wth
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn iwdg_rth(&self) -> &IwdgRth {
        &self.iwdg_rth
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn iwdg_cnt(&self) -> &IwdgCnt {
        &self.iwdg_cnt
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn aon_pwr_cfg(&self) -> &AonPwrCfg {
        &self.aon_pwr_cfg
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn aon_evt_rcd(&self) -> &AonEvtRcd {
        &self.aon_evt_rcd
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn aon_io_wake_pol(&self) -> &AonIoWakePol {
        &self.aon_io_wake_pol
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn aon_io_wake_en(&self) -> &AonIoWakeEn {
        &self.aon_io_wake_en
    }
}
#[doc = "IWDG_PSW (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_psw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_psw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_psw`] module"]
#[doc(alias = "IWDG_PSW")]
pub type IwdgPsw = crate::Reg<iwdg_psw::IwdgPswSpec>;
#[doc = ""]
pub mod iwdg_psw;
#[doc = "IWDG_CFG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_cfg`] module"]
#[doc(alias = "IWDG_CFG")]
pub type IwdgCfg = crate::Reg<iwdg_cfg::IwdgCfgSpec>;
#[doc = ""]
pub mod iwdg_cfg;
#[doc = "IWDG_CLR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_clr`] module"]
#[doc(alias = "IWDG_CLR")]
pub type IwdgClr = crate::Reg<iwdg_clr::IwdgClrSpec>;
#[doc = ""]
pub mod iwdg_clr;
#[doc = "IWDG_WTH (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_wth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_wth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_wth`] module"]
#[doc(alias = "IWDG_WTH")]
pub type IwdgWth = crate::Reg<iwdg_wth::IwdgWthSpec>;
#[doc = ""]
pub mod iwdg_wth;
#[doc = "IWDG_RTH (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_rth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_rth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_rth`] module"]
#[doc(alias = "IWDG_RTH")]
pub type IwdgRth = crate::Reg<iwdg_rth::IwdgRthSpec>;
#[doc = ""]
pub mod iwdg_rth;
#[doc = "IWDG_CNT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_cnt`] module"]
#[doc(alias = "IWDG_CNT")]
pub type IwdgCnt = crate::Reg<iwdg_cnt::IwdgCntSpec>;
#[doc = ""]
pub mod iwdg_cnt;
#[doc = "AON_PWR_CFG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`aon_pwr_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon_pwr_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_pwr_cfg`] module"]
#[doc(alias = "AON_PWR_CFG")]
pub type AonPwrCfg = crate::Reg<aon_pwr_cfg::AonPwrCfgSpec>;
#[doc = ""]
pub mod aon_pwr_cfg;
#[doc = "AON_EVT_RCD (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`aon_evt_rcd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon_evt_rcd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_evt_rcd`] module"]
#[doc(alias = "AON_EVT_RCD")]
pub type AonEvtRcd = crate::Reg<aon_evt_rcd::AonEvtRcdSpec>;
#[doc = ""]
pub mod aon_evt_rcd;
#[doc = "AON_IO_WAKE_POL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`aon_io_wake_pol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon_io_wake_pol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_io_wake_pol`] module"]
#[doc(alias = "AON_IO_WAKE_POL")]
pub type AonIoWakePol = crate::Reg<aon_io_wake_pol::AonIoWakePolSpec>;
#[doc = ""]
pub mod aon_io_wake_pol;
#[doc = "AON_IO_WAKE_EN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`aon_io_wake_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon_io_wake_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_io_wake_en`] module"]
#[doc(alias = "AON_IO_WAKE_EN")]
pub type AonIoWakeEn = crate::Reg<aon_io_wake_en::AonIoWakeEnSpec>;
#[doc = ""]
pub mod aon_io_wake_en;
