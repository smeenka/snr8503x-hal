#[doc = "Register `GPIO_BRR` reader"]
pub type R = crate::R<GpioBrrSpec>;
#[doc = "Register `GPIO_BRR` writer"]
pub type W = crate::W<GpioBrrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioBrrSpec;
impl crate::RegisterSpec for GpioBrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_brr::R`](R) reader structure"]
impl crate::Readable for GpioBrrSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_brr::W`](W) writer structure"]
impl crate::Writable for GpioBrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_BRR to value 0"]
impl crate::Resettable for GpioBrrSpec {}
