#[doc = "Register `SPI_RX_DATA` reader"]
pub type R = crate::R<SpiRxDataSpec>;
#[doc = "Register `SPI_RX_DATA` writer"]
pub type W = crate::W<SpiRxDataSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_rx_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_rx_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiRxDataSpec;
impl crate::RegisterSpec for SpiRxDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_rx_data::R`](R) reader structure"]
impl crate::Readable for SpiRxDataSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_rx_data::W`](W) writer structure"]
impl crate::Writable for SpiRxDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_RX_DATA to value 0"]
impl crate::Resettable for SpiRxDataSpec {}
