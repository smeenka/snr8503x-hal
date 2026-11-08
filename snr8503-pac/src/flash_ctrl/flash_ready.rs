#[doc = "Register `FLASH_READY` reader"]
pub type R = crate::R<FlashReadySpec>;
#[doc = "Register `FLASH_READY` writer"]
pub type W = crate::W<FlashReadySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_ready::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_ready::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashReadySpec;
impl crate::RegisterSpec for FlashReadySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_ready::R`](R) reader structure"]
impl crate::Readable for FlashReadySpec {}
#[doc = "`write(|w| ..)` method takes [`flash_ready::W`](W) writer structure"]
impl crate::Writable for FlashReadySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_READY to value 0"]
impl crate::Resettable for FlashReadySpec {}
