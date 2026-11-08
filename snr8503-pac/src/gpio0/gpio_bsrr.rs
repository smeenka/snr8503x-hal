#[doc = "Register `GPIO_BSRR` reader"]
pub type R = crate::R<GpioBsrrSpec>;
#[doc = "Register `GPIO_BSRR` writer"]
pub type W = crate::W<GpioBsrrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_bsrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_bsrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioBsrrSpec;
impl crate::RegisterSpec for GpioBsrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_bsrr::R`](R) reader structure"]
impl crate::Readable for GpioBsrrSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_bsrr::W`](W) writer structure"]
impl crate::Writable for GpioBsrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_BSRR to value 0"]
impl crate::Resettable for GpioBsrrSpec {}
