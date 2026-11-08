#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x20],
    dsp_did: DspDid,
    dsp_dis: DspDis,
    dsp_quo: DspQuo,
    dsp_rem: DspRem,
    dsp_rad: DspRad,
    dsp_sqrt: DspSqrt,
}
impl RegisterBlock {
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn dsp_did(&self) -> &DspDid {
        &self.dsp_did
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn dsp_dis(&self) -> &DspDis {
        &self.dsp_dis
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn dsp_quo(&self) -> &DspQuo {
        &self.dsp_quo
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn dsp_rem(&self) -> &DspRem {
        &self.dsp_rem
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn dsp_rad(&self) -> &DspRad {
        &self.dsp_rad
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn dsp_sqrt(&self) -> &DspSqrt {
        &self.dsp_sqrt
    }
}
#[doc = "DSP_DID (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dsp_did::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp_did::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_did`] module"]
#[doc(alias = "DSP_DID")]
pub type DspDid = crate::Reg<dsp_did::DspDidSpec>;
#[doc = ""]
pub mod dsp_did;
#[doc = "DSP_DIS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dsp_dis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp_dis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_dis`] module"]
#[doc(alias = "DSP_DIS")]
pub type DspDis = crate::Reg<dsp_dis::DspDisSpec>;
#[doc = ""]
pub mod dsp_dis;
#[doc = "DSP_QUO (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dsp_quo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp_quo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_quo`] module"]
#[doc(alias = "DSP_QUO")]
pub type DspQuo = crate::Reg<dsp_quo::DspQuoSpec>;
#[doc = ""]
pub mod dsp_quo;
#[doc = "DSP_REM (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dsp_rem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp_rem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_rem`] module"]
#[doc(alias = "DSP_REM")]
pub type DspRem = crate::Reg<dsp_rem::DspRemSpec>;
#[doc = ""]
pub mod dsp_rem;
#[doc = "DSP_RAD (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dsp_rad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp_rad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_rad`] module"]
#[doc(alias = "DSP_RAD")]
pub type DspRad = crate::Reg<dsp_rad::DspRadSpec>;
#[doc = ""]
pub mod dsp_rad;
#[doc = "DSP_SQRT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dsp_sqrt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp_sqrt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_sqrt`] module"]
#[doc(alias = "DSP_SQRT")]
pub type DspSqrt = crate::Reg<dsp_sqrt::DspSqrtSpec>;
#[doc = ""]
pub mod dsp_sqrt;
