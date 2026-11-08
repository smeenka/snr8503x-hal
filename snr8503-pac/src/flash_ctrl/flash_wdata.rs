#[doc = "Register `FLASH_WDATA` reader"]
pub type R = crate::R<FlashWdataSpec>;
#[doc = "Register `FLASH_WDATA` writer"]
pub type W = crate::W<FlashWdataSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_wdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_wdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashWdataSpec;
impl crate::RegisterSpec for FlashWdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_wdata::R`](R) reader structure"]
impl crate::Readable for FlashWdataSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_wdata::W`](W) writer structure"]
impl crate::Writable for FlashWdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_WDATA to value 0"]
impl crate::Resettable for FlashWdataSpec {}
