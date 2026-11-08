#[doc = "Register `GPIO_PODE` reader"]
pub type R = crate::R<GpioPodeSpec>;
#[doc = "Register `GPIO_PODE` writer"]
pub type W = crate::W<GpioPodeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_pode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_pode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioPodeSpec;
impl crate::RegisterSpec for GpioPodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_pode::R`](R) reader structure"]
impl crate::Readable for GpioPodeSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_pode::W`](W) writer structure"]
impl crate::Writable for GpioPodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_PODE to value 0"]
impl crate::Resettable for GpioPodeSpec {}
