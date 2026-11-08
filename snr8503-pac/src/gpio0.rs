#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpio_pie: GpioPie,
    gpio_poe: GpioPoe,
    gpio_pdi: GpioPdi,
    gpio_pdo: GpioPdo,
    gpio_pue: GpioPue,
    _reserved5: [u8; 0x04],
    gpio_pode: GpioPode,
    gpio_pflt: GpioPflt,
    gpio_f3210: GpioF3210,
    gpio_f7654: GpioF7654,
    gpio_fba98: GpioFba98,
    gpio_ffedc: GpioFfedc,
    gpio_bsrr: GpioBsrr,
    gpio_brr: GpioBrr,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn gpio_pie(&self) -> &GpioPie {
        &self.gpio_pie
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn gpio_poe(&self) -> &GpioPoe {
        &self.gpio_poe
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn gpio_pdi(&self) -> &GpioPdi {
        &self.gpio_pdi
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn gpio_pdo(&self) -> &GpioPdo {
        &self.gpio_pdo
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn gpio_pue(&self) -> &GpioPue {
        &self.gpio_pue
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn gpio_pode(&self) -> &GpioPode {
        &self.gpio_pode
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn gpio_pflt(&self) -> &GpioPflt {
        &self.gpio_pflt
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn gpio_f3210(&self) -> &GpioF3210 {
        &self.gpio_f3210
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn gpio_f7654(&self) -> &GpioF7654 {
        &self.gpio_f7654
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn gpio_fba98(&self) -> &GpioFba98 {
        &self.gpio_fba98
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn gpio_ffedc(&self) -> &GpioFfedc {
        &self.gpio_ffedc
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn gpio_bsrr(&self) -> &GpioBsrr {
        &self.gpio_bsrr
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn gpio_brr(&self) -> &GpioBrr {
        &self.gpio_brr
    }
}
#[doc = "GPIO_PIE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_pie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_pie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pie`] module"]
#[doc(alias = "GPIO_PIE")]
pub type GpioPie = crate::Reg<gpio_pie::GpioPieSpec>;
#[doc = ""]
pub mod gpio_pie;
#[doc = "GPIO_POE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_poe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_poe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_poe`] module"]
#[doc(alias = "GPIO_POE")]
pub type GpioPoe = crate::Reg<gpio_poe::GpioPoeSpec>;
#[doc = ""]
pub mod gpio_poe;
#[doc = "GPIO_PDI (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_pdi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_pdi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pdi`] module"]
#[doc(alias = "GPIO_PDI")]
pub type GpioPdi = crate::Reg<gpio_pdi::GpioPdiSpec>;
#[doc = ""]
pub mod gpio_pdi;
#[doc = "GPIO_PDO (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_pdo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_pdo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pdo`] module"]
#[doc(alias = "GPIO_PDO")]
pub type GpioPdo = crate::Reg<gpio_pdo::GpioPdoSpec>;
#[doc = ""]
pub mod gpio_pdo;
#[doc = "GPIO_PUE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_pue::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_pue::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pue`] module"]
#[doc(alias = "GPIO_PUE")]
pub type GpioPue = crate::Reg<gpio_pue::GpioPueSpec>;
#[doc = ""]
pub mod gpio_pue;
#[doc = "GPIO_PODE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_pode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_pode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pode`] module"]
#[doc(alias = "GPIO_PODE")]
pub type GpioPode = crate::Reg<gpio_pode::GpioPodeSpec>;
#[doc = ""]
pub mod gpio_pode;
#[doc = "GPIO_PFLT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_pflt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_pflt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pflt`] module"]
#[doc(alias = "GPIO_PFLT")]
pub type GpioPflt = crate::Reg<gpio_pflt::GpioPfltSpec>;
#[doc = ""]
pub mod gpio_pflt;
#[doc = "GPIO_F3210 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_f3210::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_f3210::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_f3210`] module"]
#[doc(alias = "GPIO_F3210")]
pub type GpioF3210 = crate::Reg<gpio_f3210::GpioF3210Spec>;
#[doc = ""]
pub mod gpio_f3210;
#[doc = "GPIO_F7654 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_f7654::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_f7654::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_f7654`] module"]
#[doc(alias = "GPIO_F7654")]
pub type GpioF7654 = crate::Reg<gpio_f7654::GpioF7654Spec>;
#[doc = ""]
pub mod gpio_f7654;
#[doc = "GPIO_FBA98 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_fba98::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_fba98::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_fba98`] module"]
#[doc(alias = "GPIO_FBA98")]
pub type GpioFba98 = crate::Reg<gpio_fba98::GpioFba98Spec>;
#[doc = ""]
pub mod gpio_fba98;
#[doc = "GPIO_FFEDC (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ffedc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ffedc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ffedc`] module"]
#[doc(alias = "GPIO_FFEDC")]
pub type GpioFfedc = crate::Reg<gpio_ffedc::GpioFfedcSpec>;
#[doc = ""]
pub mod gpio_ffedc;
#[doc = "GPIO_BSRR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_bsrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_bsrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_bsrr`] module"]
#[doc(alias = "GPIO_BSRR")]
pub type GpioBsrr = crate::Reg<gpio_bsrr::GpioBsrrSpec>;
#[doc = ""]
pub mod gpio_bsrr;
#[doc = "GPIO_BRR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_brr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_brr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_brr`] module"]
#[doc(alias = "GPIO_BRR")]
pub type GpioBrr = crate::Reg<gpio_brr::GpioBrrSpec>;
#[doc = ""]
pub mod gpio_brr;
