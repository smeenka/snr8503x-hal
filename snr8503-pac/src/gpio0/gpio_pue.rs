#[doc = "Register `GPIO_PUE` reader"]
pub type R = crate::R<GpioPueSpec>;
#[doc = "Register `GPIO_PUE` writer"]
pub type W = crate::W<GpioPueSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_pue::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_pue::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioPueSpec;
impl crate::RegisterSpec for GpioPueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_pue::R`](R) reader structure"]
impl crate::Readable for GpioPueSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_pue::W`](W) writer structure"]
impl crate::Writable for GpioPueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_PUE to value 0"]
impl crate::Resettable for GpioPueSpec {}
