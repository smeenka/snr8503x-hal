#[doc = "Register `SPI_SIZE` reader"]
pub type R = crate::R<SpiSizeSpec>;
#[doc = "Register `SPI_SIZE` writer"]
pub type W = crate::W<SpiSizeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiSizeSpec;
impl crate::RegisterSpec for SpiSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_size::R`](R) reader structure"]
impl crate::Readable for SpiSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_size::W`](W) writer structure"]
impl crate::Writable for SpiSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_SIZE to value 0"]
impl crate::Resettable for SpiSizeSpec {}
