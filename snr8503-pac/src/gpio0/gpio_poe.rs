#[doc = "Register `GPIO_POE` reader"]
pub type R = crate::R<GpioPoeSpec>;
#[doc = "Register `GPIO_POE` writer"]
pub type W = crate::W<GpioPoeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_poe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_poe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioPoeSpec;
impl crate::RegisterSpec for GpioPoeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_poe::R`](R) reader structure"]
impl crate::Readable for GpioPoeSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_poe::W`](W) writer structure"]
impl crate::Writable for GpioPoeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_POE to value 0"]
impl crate::Resettable for GpioPoeSpec {}
