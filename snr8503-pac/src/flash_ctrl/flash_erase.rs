#[doc = "Register `FLASH_ERASE` reader"]
pub type R = crate::R<FlashEraseSpec>;
#[doc = "Register `FLASH_ERASE` writer"]
pub type W = crate::W<FlashEraseSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_erase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_erase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashEraseSpec;
impl crate::RegisterSpec for FlashEraseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_erase::R`](R) reader structure"]
impl crate::Readable for FlashEraseSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_erase::W`](W) writer structure"]
impl crate::Writable for FlashEraseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_ERASE to value 0"]
impl crate::Resettable for FlashEraseSpec {}
