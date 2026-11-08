#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    exti_cr0: ExtiCr0,
    exti_cr1: ExtiCr1,
    exti_ie: ExtiIe,
    exti_if: ExtiIf,
    clko_sel: ClkoSel,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn exti_cr0(&self) -> &ExtiCr0 {
        &self.exti_cr0
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn exti_cr1(&self) -> &ExtiCr1 {
        &self.exti_cr1
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn exti_ie(&self) -> &ExtiIe {
        &self.exti_ie
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn exti_if(&self) -> &ExtiIf {
        &self.exti_if
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn clko_sel(&self) -> &ClkoSel {
        &self.clko_sel
    }
}
#[doc = "EXTI_CR0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`exti_cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_cr0`] module"]
#[doc(alias = "EXTI_CR0")]
pub type ExtiCr0 = crate::Reg<exti_cr0::ExtiCr0Spec>;
#[doc = ""]
pub mod exti_cr0;
#[doc = "EXTI_CR1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`exti_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_cr1`] module"]
#[doc(alias = "EXTI_CR1")]
pub type ExtiCr1 = crate::Reg<exti_cr1::ExtiCr1Spec>;
#[doc = ""]
pub mod exti_cr1;
#[doc = "EXTI_IE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`exti_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_ie`] module"]
#[doc(alias = "EXTI_IE")]
pub type ExtiIe = crate::Reg<exti_ie::ExtiIeSpec>;
#[doc = ""]
pub mod exti_ie;
#[doc = "EXTI_IF (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`exti_if::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_if::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_if`] module"]
#[doc(alias = "EXTI_IF")]
pub type ExtiIf = crate::Reg<exti_if::ExtiIfSpec>;
#[doc = ""]
pub mod exti_if;
#[doc = "CLKO_SEL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clko_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clko_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clko_sel`] module"]
#[doc(alias = "CLKO_SEL")]
pub type ClkoSel = crate::Reg<clko_sel::ClkoSelSpec>;
#[doc = ""]
pub mod clko_sel;
