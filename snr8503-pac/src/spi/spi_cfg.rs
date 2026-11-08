#[doc = "Register `SPI_CFG` reader"]
pub type R = crate::R<SpiCfgSpec>;
#[doc = "Register `SPI_CFG` writer"]
pub type W = crate::W<SpiCfgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiCfgSpec;
impl crate::RegisterSpec for SpiCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_cfg::R`](R) reader structure"]
impl crate::Readable for SpiCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_cfg::W`](W) writer structure"]
impl crate::Writable for SpiCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_CFG to value 0"]
impl crate::Resettable for SpiCfgSpec {}
