#[doc = "Register `FLASH_ADDR` reader"]
pub type R = crate::R<FlashAddrSpec>;
#[doc = "Register `FLASH_ADDR` writer"]
pub type W = crate::W<FlashAddrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashAddrSpec;
impl crate::RegisterSpec for FlashAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_addr::R`](R) reader structure"]
impl crate::Readable for FlashAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_addr::W`](W) writer structure"]
impl crate::Writable for FlashAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_ADDR to value 0"]
impl crate::Resettable for FlashAddrSpec {}
