#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    uart_ctrl: UartCtrl,
    uart_divh: UartDivh,
    uart_divl: UartDivl,
    uart_buff: UartBuff,
    uart_adr: UartAdr,
    uart_stt: UartStt,
    uart_re: UartRe,
    uart_ie: UartIe,
    uart_if: UartIf,
    uart_ioc: UartIoc,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn uart_ctrl(&self) -> &UartCtrl {
        &self.uart_ctrl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn uart_divh(&self) -> &UartDivh {
        &self.uart_divh
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn uart_divl(&self) -> &UartDivl {
        &self.uart_divl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn uart_buff(&self) -> &UartBuff {
        &self.uart_buff
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn uart_adr(&self) -> &UartAdr {
        &self.uart_adr
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn uart_stt(&self) -> &UartStt {
        &self.uart_stt
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn uart_re(&self) -> &UartRe {
        &self.uart_re
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn uart_ie(&self) -> &UartIe {
        &self.uart_ie
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn uart_if(&self) -> &UartIf {
        &self.uart_if
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn uart_ioc(&self) -> &UartIoc {
        &self.uart_ioc
    }
}
#[doc = "UART_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`uart_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_ctrl`] module"]
#[doc(alias = "UART_CTRL")]
pub type UartCtrl = crate::Reg<uart_ctrl::UartCtrlSpec>;
#[doc = ""]
pub mod uart_ctrl;
#[doc = "UART_DIVH (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`uart_divh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_divh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_divh`] module"]
#[doc(alias = "UART_DIVH")]
pub type UartDivh = crate::Reg<uart_divh::UartDivhSpec>;
#[doc = ""]
pub mod uart_divh;
#[doc = "UART_DIVL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`uart_divl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_divl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_divl`] module"]
#[doc(alias = "UART_DIVL")]
pub type UartDivl = crate::Reg<uart_divl::UartDivlSpec>;
#[doc = ""]
pub mod uart_divl;
#[doc = "UART_BUFF (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`uart_buff::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_buff::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_buff`] module"]
#[doc(alias = "UART_BUFF")]
pub type UartBuff = crate::Reg<uart_buff::UartBuffSpec>;
#[doc = ""]
pub mod uart_buff;
#[doc = "UART_ADR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`uart_adr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_adr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_adr`] module"]
#[doc(alias = "UART_ADR")]
pub type UartAdr = crate::Reg<uart_adr::UartAdrSpec>;
#[doc = ""]
pub mod uart_adr;
#[doc = "UART_STT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`uart_stt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_stt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_stt`] module"]
#[doc(alias = "UART_STT")]
pub type UartStt = crate::Reg<uart_stt::UartSttSpec>;
#[doc = ""]
pub mod uart_stt;
#[doc = "UART_RE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`uart_re::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_re::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_re`] module"]
#[doc(alias = "UART_RE")]
pub type UartRe = crate::Reg<uart_re::UartReSpec>;
#[doc = ""]
pub mod uart_re;
#[doc = "UART_IE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`uart_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_ie`] module"]
#[doc(alias = "UART_IE")]
pub type UartIe = crate::Reg<uart_ie::UartIeSpec>;
#[doc = ""]
pub mod uart_ie;
#[doc = "UART_IF (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`uart_if::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_if::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_if`] module"]
#[doc(alias = "UART_IF")]
pub type UartIf = crate::Reg<uart_if::UartIfSpec>;
#[doc = ""]
pub mod uart_if;
#[doc = "UART_IOC (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`uart_ioc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_ioc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_ioc`] module"]
#[doc(alias = "UART_IOC")]
pub type UartIoc = crate::Reg<uart_ioc::UartIocSpec>;
#[doc = ""]
pub mod uart_ioc;
