#[doc = "Register `FLASH_PROTECT` reader"]
pub type R = crate::R<FlashProtectSpec>;
#[doc = "Register `FLASH_PROTECT` writer"]
pub type W = crate::W<FlashProtectSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_protect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_protect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashProtectSpec;
impl crate::RegisterSpec for FlashProtectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_protect::R`](R) reader structure"]
impl crate::Readable for FlashProtectSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_protect::W`](W) writer structure"]
impl crate::Writable for FlashProtectSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_PROTECT to value 0"]
impl crate::Resettable for FlashProtectSpec {}
