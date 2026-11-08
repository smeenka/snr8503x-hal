#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    adc_dat: (),
    _reserved1: [u8; 0x30],
    adc_lth: AdcLth,
    adc_hth: AdcHth,
    adc_gen: AdcGen,
    _reserved4: [u8; 0x04],
    adc_chn0: AdcChn0,
    adc_chn1: AdcChn1,
    adc_chn2: AdcChn2,
    _reserved7: [u8; 0x04],
    adc_chnt: AdcChnt,
    adc_cfg: AdcCfg,
    adc_swt: AdcSwt,
    _reserved10: [u8; 0x04],
    adc_dc: AdcDc,
    adc_amc: AdcAmc,
    adc_ie: AdcIe,
    adc_if: AdcIf,
}
impl RegisterBlock {
    #[doc = "0x00..0x14 - ADC Data Registers"]
    #[inline(always)]
    pub const fn adc_dat(&self, n: usize) -> &AdcDat {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x14 - ADC Data Registers"]
    #[inline(always)]
    pub fn adc_dat_iter(&self) -> impl Iterator<Item = &AdcDat> {
        (0..10).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4 * n).cast() })
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn adc_lth(&self) -> &AdcLth {
        &self.adc_lth
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn adc_hth(&self) -> &AdcHth {
        &self.adc_hth
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn adc_gen(&self) -> &AdcGen {
        &self.adc_gen
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn adc_chn0(&self) -> &AdcChn0 {
        &self.adc_chn0
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn adc_chn1(&self) -> &AdcChn1 {
        &self.adc_chn1
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn adc_chn2(&self) -> &AdcChn2 {
        &self.adc_chn2
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn adc_chnt(&self) -> &AdcChnt {
        &self.adc_chnt
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn adc_cfg(&self) -> &AdcCfg {
        &self.adc_cfg
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn adc_swt(&self) -> &AdcSwt {
        &self.adc_swt
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn adc_dc(&self) -> &AdcDc {
        &self.adc_dc
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn adc_amc(&self) -> &AdcAmc {
        &self.adc_amc
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn adc_ie(&self) -> &AdcIe {
        &self.adc_ie
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn adc_if(&self) -> &AdcIf {
        &self.adc_if
    }
}
#[doc = "ADC_DAT (rw) register accessor: ADC Data Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_dat`] module"]
#[doc(alias = "ADC_DAT")]
pub type AdcDat = crate::Reg<adc_dat::AdcDatSpec>;
#[doc = "ADC Data Registers"]
pub mod adc_dat;
#[doc = "ADC_LTH (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`adc_lth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_lth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_lth`] module"]
#[doc(alias = "ADC_LTH")]
pub type AdcLth = crate::Reg<adc_lth::AdcLthSpec>;
#[doc = ""]
pub mod adc_lth;
#[doc = "ADC_HTH (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`adc_hth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_hth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_hth`] module"]
#[doc(alias = "ADC_HTH")]
pub type AdcHth = crate::Reg<adc_hth::AdcHthSpec>;
#[doc = ""]
pub mod adc_hth;
#[doc = "ADC_GEN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`adc_gen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_gen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_gen`] module"]
#[doc(alias = "ADC_GEN")]
pub type AdcGen = crate::Reg<adc_gen::AdcGenSpec>;
#[doc = ""]
pub mod adc_gen;
#[doc = "ADC_CHN0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`adc_chn0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_chn0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_chn0`] module"]
#[doc(alias = "ADC_CHN0")]
pub type AdcChn0 = crate::Reg<adc_chn0::AdcChn0Spec>;
#[doc = ""]
pub mod adc_chn0;
#[doc = "ADC_CHN1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`adc_chn1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_chn1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_chn1`] module"]
#[doc(alias = "ADC_CHN1")]
pub type AdcChn1 = crate::Reg<adc_chn1::AdcChn1Spec>;
#[doc = ""]
pub mod adc_chn1;
#[doc = "ADC_CHN2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`adc_chn2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_chn2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_chn2`] module"]
#[doc(alias = "ADC_CHN2")]
pub type AdcChn2 = crate::Reg<adc_chn2::AdcChn2Spec>;
#[doc = ""]
pub mod adc_chn2;
#[doc = "ADC_CHNT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`adc_chnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_chnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_chnt`] module"]
#[doc(alias = "ADC_CHNT")]
pub type AdcChnt = crate::Reg<adc_chnt::AdcChntSpec>;
#[doc = ""]
pub mod adc_chnt;
#[doc = "ADC_CFG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`adc_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_cfg`] module"]
#[doc(alias = "ADC_CFG")]
pub type AdcCfg = crate::Reg<adc_cfg::AdcCfgSpec>;
#[doc = ""]
pub mod adc_cfg;
#[doc = "ADC_SWT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`adc_swt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_swt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_swt`] module"]
#[doc(alias = "ADC_SWT")]
pub type AdcSwt = crate::Reg<adc_swt::AdcSwtSpec>;
#[doc = ""]
pub mod adc_swt;
#[doc = "ADC_DC (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`adc_dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_dc`] module"]
#[doc(alias = "ADC_DC")]
pub type AdcDc = crate::Reg<adc_dc::AdcDcSpec>;
#[doc = ""]
pub mod adc_dc;
#[doc = "ADC_AMC (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`adc_amc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_amc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_amc`] module"]
#[doc(alias = "ADC_AMC")]
pub type AdcAmc = crate::Reg<adc_amc::AdcAmcSpec>;
#[doc = ""]
pub mod adc_amc;
#[doc = "ADC_IE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`adc_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_ie`] module"]
#[doc(alias = "ADC_IE")]
pub type AdcIe = crate::Reg<adc_ie::AdcIeSpec>;
#[doc = ""]
pub mod adc_ie;
#[doc = "ADC_IF (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`adc_if::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_if::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_if`] module"]
#[doc(alias = "ADC_IF")]
pub type AdcIf = crate::Reg<adc_if::AdcIfSpec>;
#[doc = ""]
pub mod adc_if;
