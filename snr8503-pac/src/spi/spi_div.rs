#[doc = "Register `SPI_DIV` reader"]
pub type R = crate::R<SpiDivSpec>;
#[doc = "Register `SPI_DIV` writer"]
pub type W = crate::W<SpiDivSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiDivSpec;
impl crate::RegisterSpec for SpiDivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_div::R`](R) reader structure"]
impl crate::Readable for SpiDivSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_div::W`](W) writer structure"]
impl crate::Writable for SpiDivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_DIV to value 0"]
impl crate::Resettable for SpiDivSpec {}
