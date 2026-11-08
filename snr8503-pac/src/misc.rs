#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sys_afe_adc: SysAfeAdc,
    sys_afe_info: SysAfeInfo,
    sys_afe_dbg: SysAfeDbg,
    sys_opa_sel: SysOpaSel,
    sys_afe_reg0: SysAfeReg0,
    sys_afe_reg1: SysAfeReg1,
    sys_afe_reg2: SysAfeReg2,
    sys_afe_reg3: SysAfeReg3,
    sys_afe_reg4: SysAfeReg4,
    sys_afe_reg5: SysAfeReg5,
    sys_afe_reg6: SysAfeReg6,
    sys_afe_dac: SysAfeDac,
    _reserved12: [u8; 0x50],
    sys_clk_cfg: SysClkCfg,
    sys_io_cfg: SysIoCfg,
    sys_dbg_cfg: SysDbgCfg,
    _reserved15: [u8; 0x04],
    sys_clk_div0: SysClkDiv0,
    sys_clk_div1: SysClkDiv1,
    sys_clk_div2: SysClkDiv2,
    sys_clk_fen: SysClkFen,
    sys_trim: SysTrim,
    sys_sft_rst: SysSftRst,
    sys_wr_protect: SysWrProtect,
    _reserved22: [u8; 0x14],
    sys_mbist: SysMbist,
    _reserved23: [u8; 0x0c],
    sys_flse: SysFlse,
    sys_flsp: SysFlsp,
    sys_flst: SysFlst,
    _reserved26: [u8; 0x20],
    sys_test: SysTest,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn sys_afe_adc(&self) -> &SysAfeAdc {
        &self.sys_afe_adc
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn sys_afe_info(&self) -> &SysAfeInfo {
        &self.sys_afe_info
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn sys_afe_dbg(&self) -> &SysAfeDbg {
        &self.sys_afe_dbg
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn sys_opa_sel(&self) -> &SysOpaSel {
        &self.sys_opa_sel
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn sys_afe_reg0(&self) -> &SysAfeReg0 {
        &self.sys_afe_reg0
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn sys_afe_reg1(&self) -> &SysAfeReg1 {
        &self.sys_afe_reg1
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn sys_afe_reg2(&self) -> &SysAfeReg2 {
        &self.sys_afe_reg2
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn sys_afe_reg3(&self) -> &SysAfeReg3 {
        &self.sys_afe_reg3
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn sys_afe_reg4(&self) -> &SysAfeReg4 {
        &self.sys_afe_reg4
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn sys_afe_reg5(&self) -> &SysAfeReg5 {
        &self.sys_afe_reg5
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn sys_afe_reg6(&self) -> &SysAfeReg6 {
        &self.sys_afe_reg6
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn sys_afe_dac(&self) -> &SysAfeDac {
        &self.sys_afe_dac
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn sys_clk_cfg(&self) -> &SysClkCfg {
        &self.sys_clk_cfg
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn sys_io_cfg(&self) -> &SysIoCfg {
        &self.sys_io_cfg
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn sys_dbg_cfg(&self) -> &SysDbgCfg {
        &self.sys_dbg_cfg
    }
    #[doc = "0x90 - SPI clock div"]
    #[inline(always)]
    pub const fn sys_clk_div0(&self) -> &SysClkDiv0 {
        &self.sys_clk_div0
    }
    #[doc = "0x94 - I2C clock div"]
    #[inline(always)]
    pub const fn sys_clk_div1(&self) -> &SysClkDiv1 {
        &self.sys_clk_div1
    }
    #[doc = "0x98 - UART0/1 clock div"]
    #[inline(always)]
    pub const fn sys_clk_div2(&self) -> &SysClkDiv2 {
        &self.sys_clk_div2
    }
    #[doc = "0x9c - peripheral clock enable"]
    #[inline(always)]
    pub const fn sys_clk_fen(&self) -> &SysClkFen {
        &self.sys_clk_fen
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn sys_trim(&self) -> &SysTrim {
        &self.sys_trim
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn sys_sft_rst(&self) -> &SysSftRst {
        &self.sys_sft_rst
    }
    #[doc = "0xa8 - Write Protect (Aliases SYS_PROTECT, SYS_PROT)"]
    #[inline(always)]
    pub const fn sys_wr_protect(&self) -> &SysWrProtect {
        &self.sys_wr_protect
    }
    #[doc = "0xc0 - "]
    #[inline(always)]
    pub const fn sys_mbist(&self) -> &SysMbist {
        &self.sys_mbist
    }
    #[doc = "0xd0 - "]
    #[inline(always)]
    pub const fn sys_flse(&self) -> &SysFlse {
        &self.sys_flse
    }
    #[doc = "0xd4 - "]
    #[inline(always)]
    pub const fn sys_flsp(&self) -> &SysFlsp {
        &self.sys_flsp
    }
    #[doc = "0xd8 - "]
    #[inline(always)]
    pub const fn sys_flst(&self) -> &SysFlst {
        &self.sys_flst
    }
    #[doc = "0xfc - "]
    #[inline(always)]
    pub const fn sys_test(&self) -> &SysTest {
        &self.sys_test
    }
}
#[doc = "SYS_AFE_ADC (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_afe_adc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_afe_adc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_afe_adc`] module"]
#[doc(alias = "SYS_AFE_ADC")]
pub type SysAfeAdc = crate::Reg<sys_afe_adc::SysAfeAdcSpec>;
#[doc = ""]
pub mod sys_afe_adc;
#[doc = "SYS_AFE_INFO (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_afe_info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_afe_info`] module"]
#[doc(alias = "SYS_AFE_INFO")]
pub type SysAfeInfo = crate::Reg<sys_afe_info::SysAfeInfoSpec>;
#[doc = ""]
pub mod sys_afe_info;
#[doc = "SYS_AFE_DBG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_afe_dbg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_afe_dbg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_afe_dbg`] module"]
#[doc(alias = "SYS_AFE_DBG")]
pub type SysAfeDbg = crate::Reg<sys_afe_dbg::SysAfeDbgSpec>;
#[doc = ""]
pub mod sys_afe_dbg;
#[doc = "SYS_OPA_SEL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_opa_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_opa_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_opa_sel`] module"]
#[doc(alias = "SYS_OPA_SEL")]
pub type SysOpaSel = crate::Reg<sys_opa_sel::SysOpaSelSpec>;
#[doc = ""]
pub mod sys_opa_sel;
#[doc = "SYS_AFE_REG0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_afe_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_afe_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_afe_reg0`] module"]
#[doc(alias = "SYS_AFE_REG0")]
pub type SysAfeReg0 = crate::Reg<sys_afe_reg0::SysAfeReg0Spec>;
#[doc = ""]
pub mod sys_afe_reg0;
#[doc = "SYS_AFE_REG1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_afe_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_afe_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_afe_reg1`] module"]
#[doc(alias = "SYS_AFE_REG1")]
pub type SysAfeReg1 = crate::Reg<sys_afe_reg1::SysAfeReg1Spec>;
#[doc = ""]
pub mod sys_afe_reg1;
#[doc = "SYS_AFE_REG2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_afe_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_afe_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_afe_reg2`] module"]
#[doc(alias = "SYS_AFE_REG2")]
pub type SysAfeReg2 = crate::Reg<sys_afe_reg2::SysAfeReg2Spec>;
#[doc = ""]
pub mod sys_afe_reg2;
#[doc = "SYS_AFE_REG3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_afe_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_afe_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_afe_reg3`] module"]
#[doc(alias = "SYS_AFE_REG3")]
pub type SysAfeReg3 = crate::Reg<sys_afe_reg3::SysAfeReg3Spec>;
#[doc = ""]
pub mod sys_afe_reg3;
#[doc = "SYS_AFE_REG4 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_afe_reg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_afe_reg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_afe_reg4`] module"]
#[doc(alias = "SYS_AFE_REG4")]
pub type SysAfeReg4 = crate::Reg<sys_afe_reg4::SysAfeReg4Spec>;
#[doc = ""]
pub mod sys_afe_reg4;
#[doc = "SYS_AFE_REG5 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_afe_reg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_afe_reg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_afe_reg5`] module"]
#[doc(alias = "SYS_AFE_REG5")]
pub type SysAfeReg5 = crate::Reg<sys_afe_reg5::SysAfeReg5Spec>;
#[doc = ""]
pub mod sys_afe_reg5;
#[doc = "SYS_AFE_REG6 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_afe_reg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_afe_reg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_afe_reg6`] module"]
#[doc(alias = "SYS_AFE_REG6")]
pub type SysAfeReg6 = crate::Reg<sys_afe_reg6::SysAfeReg6Spec>;
#[doc = ""]
pub mod sys_afe_reg6;
#[doc = "SYS_AFE_DAC (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_afe_dac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_afe_dac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_afe_dac`] module"]
#[doc(alias = "SYS_AFE_DAC")]
pub type SysAfeDac = crate::Reg<sys_afe_dac::SysAfeDacSpec>;
#[doc = ""]
pub mod sys_afe_dac;
#[doc = "SYS_CLK_CFG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_clk_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_clk_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_clk_cfg`] module"]
#[doc(alias = "SYS_CLK_CFG")]
pub type SysClkCfg = crate::Reg<sys_clk_cfg::SysClkCfgSpec>;
#[doc = ""]
pub mod sys_clk_cfg;
#[doc = "SYS_IO_CFG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_io_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_io_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_io_cfg`] module"]
#[doc(alias = "SYS_IO_CFG")]
pub type SysIoCfg = crate::Reg<sys_io_cfg::SysIoCfgSpec>;
#[doc = ""]
pub mod sys_io_cfg;
#[doc = "SYS_DBG_CFG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_dbg_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_dbg_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_dbg_cfg`] module"]
#[doc(alias = "SYS_DBG_CFG")]
pub type SysDbgCfg = crate::Reg<sys_dbg_cfg::SysDbgCfgSpec>;
#[doc = ""]
pub mod sys_dbg_cfg;
#[doc = "SYS_CLK_DIV0 (rw) register accessor: SPI clock div\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_clk_div0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_clk_div0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_clk_div0`] module"]
#[doc(alias = "SYS_CLK_DIV0")]
pub type SysClkDiv0 = crate::Reg<sys_clk_div0::SysClkDiv0Spec>;
#[doc = "SPI clock div"]
pub mod sys_clk_div0;
#[doc = "SYS_CLK_DIV1 (rw) register accessor: I2C clock div\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_clk_div1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_clk_div1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_clk_div1`] module"]
#[doc(alias = "SYS_CLK_DIV1")]
pub type SysClkDiv1 = crate::Reg<sys_clk_div1::SysClkDiv1Spec>;
#[doc = "I2C clock div"]
pub mod sys_clk_div1;
#[doc = "SYS_CLK_DIV2 (rw) register accessor: UART0/1 clock div\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_clk_div2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_clk_div2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_clk_div2`] module"]
#[doc(alias = "SYS_CLK_DIV2")]
pub type SysClkDiv2 = crate::Reg<sys_clk_div2::SysClkDiv2Spec>;
#[doc = "UART0/1 clock div"]
pub mod sys_clk_div2;
#[doc = "SYS_CLK_FEN (rw) register accessor: peripheral clock enable\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_clk_fen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_clk_fen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_clk_fen`] module"]
#[doc(alias = "SYS_CLK_FEN")]
pub type SysClkFen = crate::Reg<sys_clk_fen::SysClkFenSpec>;
#[doc = "peripheral clock enable"]
pub mod sys_clk_fen;
#[doc = "SYS_TRIM (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_trim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_trim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_trim`] module"]
#[doc(alias = "SYS_TRIM")]
pub type SysTrim = crate::Reg<sys_trim::SysTrimSpec>;
#[doc = ""]
pub mod sys_trim;
#[doc = "SYS_SFT_RST (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_sft_rst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_sft_rst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sft_rst`] module"]
#[doc(alias = "SYS_SFT_RST")]
pub type SysSftRst = crate::Reg<sys_sft_rst::SysSftRstSpec>;
#[doc = ""]
pub mod sys_sft_rst;
#[doc = "SYS_WR_PROTECT (rw) register accessor: Write Protect (Aliases SYS_PROTECT, SYS_PROT)\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_wr_protect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_wr_protect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_wr_protect`] module"]
#[doc(alias = "SYS_WR_PROTECT")]
pub type SysWrProtect = crate::Reg<sys_wr_protect::SysWrProtectSpec>;
#[doc = "Write Protect (Aliases SYS_PROTECT, SYS_PROT)"]
pub mod sys_wr_protect;
#[doc = "SYS_MBIST (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_mbist::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_mbist::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_mbist`] module"]
#[doc(alias = "SYS_MBIST")]
pub type SysMbist = crate::Reg<sys_mbist::SysMbistSpec>;
#[doc = ""]
pub mod sys_mbist;
#[doc = "SYS_FLSE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_flse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_flse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_flse`] module"]
#[doc(alias = "SYS_FLSE")]
pub type SysFlse = crate::Reg<sys_flse::SysFlseSpec>;
#[doc = ""]
pub mod sys_flse;
#[doc = "SYS_FLSP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_flsp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_flsp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_flsp`] module"]
#[doc(alias = "SYS_FLSP")]
pub type SysFlsp = crate::Reg<sys_flsp::SysFlspSpec>;
#[doc = ""]
pub mod sys_flsp;
#[doc = "SYS_FLST (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_flst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_flst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_flst`] module"]
#[doc(alias = "SYS_FLST")]
pub type SysFlst = crate::Reg<sys_flst::SysFlstSpec>;
#[doc = ""]
pub mod sys_flst;
#[doc = "SYS_TEST (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_test`] module"]
#[doc(alias = "SYS_TEST")]
pub type SysTest = crate::Reg<sys_test::SysTestSpec>;
#[doc = ""]
pub mod sys_test;
