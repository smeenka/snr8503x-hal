#[doc = "Register `SPI_TX_DATA` reader"]
pub type R = crate::R<SpiTxDataSpec>;
#[doc = "Register `SPI_TX_DATA` writer"]
pub type W = crate::W<SpiTxDataSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_tx_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_tx_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiTxDataSpec;
impl crate::RegisterSpec for SpiTxDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_tx_data::R`](R) reader structure"]
impl crate::Readable for SpiTxDataSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_tx_data::W`](W) writer structure"]
impl crate::Writable for SpiTxDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_TX_DATA to value 0"]
impl crate::Resettable for SpiTxDataSpec {}
