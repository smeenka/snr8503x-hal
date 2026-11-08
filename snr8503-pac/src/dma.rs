#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dma_ccr: (),
    _reserved1: [u8; 0x04],
    dma_ren: (),
    _reserved2: [u8; 0x04],
    dma_ctms: (),
    _reserved3: [u8; 0x04],
    dma_sadr: (),
    _reserved4: [u8; 0x04],
    dma_dadr: (),
    _reserved5: [u8; 0x70],
    dma_ctrl: DmaCtrl,
    dma_ie: DmaIe,
    dma_if: DmaIf,
}
impl RegisterBlock {
    #[doc = "0x00..0x10 - "]
    #[inline(always)]
    pub const fn dma_ccr(&self, n: usize) -> &DmaCcr {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - "]
    #[inline(always)]
    pub fn dma_ccr_iter(&self) -> impl Iterator<Item = &DmaCcr> {
        (0..4).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32 * n).cast() })
    }
    #[doc = "0x04..0x14 - "]
    #[inline(always)]
    pub const fn dma_ren(&self, n: usize) -> &DmaRen {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x14 - "]
    #[inline(always)]
    pub fn dma_ren_iter(&self) -> impl Iterator<Item = &DmaRen> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x08..0x18 - "]
    #[inline(always)]
    pub const fn dma_ctms(&self, n: usize) -> &DmaCtms {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x18 - "]
    #[inline(always)]
    pub fn dma_ctms_iter(&self) -> impl Iterator<Item = &DmaCtms> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x0c..0x1c - "]
    #[inline(always)]
    pub const fn dma_sadr(&self, n: usize) -> &DmaSadr {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(12)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x1c - "]
    #[inline(always)]
    pub fn dma_sadr_iter(&self) -> impl Iterator<Item = &DmaSadr> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(12)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x10..0x20 - "]
    #[inline(always)]
    pub const fn dma_dadr(&self, n: usize) -> &DmaDadr {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(16)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x20 - "]
    #[inline(always)]
    pub fn dma_dadr_iter(&self) -> impl Iterator<Item = &DmaDadr> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(16)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn dma_ctrl(&self) -> &DmaCtrl {
        &self.dma_ctrl
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn dma_ie(&self) -> &DmaIe {
        &self.dma_ie
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn dma_if(&self) -> &DmaIf {
        &self.dma_if
    }
}
#[doc = "DMA_CCR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma_ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ccr`] module"]
#[doc(alias = "DMA_CCR")]
pub type DmaCcr = crate::Reg<dma_ccr::DmaCcrSpec>;
#[doc = ""]
pub mod dma_ccr;
#[doc = "DMA_REN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma_ren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ren`] module"]
#[doc(alias = "DMA_REN")]
pub type DmaRen = crate::Reg<dma_ren::DmaRenSpec>;
#[doc = ""]
pub mod dma_ren;
#[doc = "DMA_CTMS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma_ctms::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ctms::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ctms`] module"]
#[doc(alias = "DMA_CTMS")]
pub type DmaCtms = crate::Reg<dma_ctms::DmaCtmsSpec>;
#[doc = ""]
pub mod dma_ctms;
#[doc = "DMA_SADR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma_sadr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_sadr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_sadr`] module"]
#[doc(alias = "DMA_SADR")]
pub type DmaSadr = crate::Reg<dma_sadr::DmaSadrSpec>;
#[doc = ""]
pub mod dma_sadr;
#[doc = "DMA_DADR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma_dadr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_dadr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_dadr`] module"]
#[doc(alias = "DMA_DADR")]
pub type DmaDadr = crate::Reg<dma_dadr::DmaDadrSpec>;
#[doc = ""]
pub mod dma_dadr;
#[doc = "DMA_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ctrl`] module"]
#[doc(alias = "DMA_CTRL")]
pub type DmaCtrl = crate::Reg<dma_ctrl::DmaCtrlSpec>;
#[doc = ""]
pub mod dma_ctrl;
#[doc = "DMA_IE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ie`] module"]
#[doc(alias = "DMA_IE")]
pub type DmaIe = crate::Reg<dma_ie::DmaIeSpec>;
#[doc = ""]
pub mod dma_ie;
#[doc = "DMA_IF (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma_if::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_if::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_if`] module"]
#[doc(alias = "DMA_IF")]
pub type DmaIf = crate::Reg<dma_if::DmaIfSpec>;
#[doc = ""]
pub mod dma_if;
