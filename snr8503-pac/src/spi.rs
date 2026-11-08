#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    spi_cfg: SpiCfg,
    spi_ie: SpiIe,
    spi_div: SpiDiv,
    spi_tx_data: SpiTxData,
    spi_rx_data: SpiRxData,
    spi_size: SpiSize,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn spi_cfg(&self) -> &SpiCfg {
        &self.spi_cfg
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn spi_ie(&self) -> &SpiIe {
        &self.spi_ie
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn spi_div(&self) -> &SpiDiv {
        &self.spi_div
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn spi_tx_data(&self) -> &SpiTxData {
        &self.spi_tx_data
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn spi_rx_data(&self) -> &SpiRxData {
        &self.spi_rx_data
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn spi_size(&self) -> &SpiSize {
        &self.spi_size
    }
}
#[doc = "SPI_CFG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_cfg`] module"]
#[doc(alias = "SPI_CFG")]
pub type SpiCfg = crate::Reg<spi_cfg::SpiCfgSpec>;
#[doc = ""]
pub mod spi_cfg;
#[doc = "SPI_IE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ie`] module"]
#[doc(alias = "SPI_IE")]
pub type SpiIe = crate::Reg<spi_ie::SpiIeSpec>;
#[doc = ""]
pub mod spi_ie;
#[doc = "SPI_DIV (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_div`] module"]
#[doc(alias = "SPI_DIV")]
pub type SpiDiv = crate::Reg<spi_div::SpiDivSpec>;
#[doc = ""]
pub mod spi_div;
#[doc = "SPI_TX_DATA (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_tx_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_tx_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_tx_data`] module"]
#[doc(alias = "SPI_TX_DATA")]
pub type SpiTxData = crate::Reg<spi_tx_data::SpiTxDataSpec>;
#[doc = ""]
pub mod spi_tx_data;
#[doc = "SPI_RX_DATA (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_rx_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_rx_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_rx_data`] module"]
#[doc(alias = "SPI_RX_DATA")]
pub type SpiRxData = crate::Reg<spi_rx_data::SpiRxDataSpec>;
#[doc = ""]
pub mod spi_rx_data;
#[doc = "SPI_SIZE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_size`] module"]
#[doc(alias = "SPI_SIZE")]
pub type SpiSize = crate::Reg<spi_size::SpiSizeSpec>;
#[doc = ""]
pub mod spi_size;
