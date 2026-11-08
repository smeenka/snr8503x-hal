#[doc = "Register `FLASH_RDATA` reader"]
pub type R = crate::R<FlashRdataSpec>;
#[doc = "Register `FLASH_RDATA` writer"]
pub type W = crate::W<FlashRdataSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashRdataSpec;
impl crate::RegisterSpec for FlashRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_rdata::R`](R) reader structure"]
impl crate::Readable for FlashRdataSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_rdata::W`](W) writer structure"]
impl crate::Writable for FlashRdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_RDATA to value 0"]
impl crate::Resettable for FlashRdataSpec {}
