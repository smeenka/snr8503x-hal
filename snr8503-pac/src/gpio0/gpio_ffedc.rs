#[doc = "Register `GPIO_FFEDC` reader"]
pub type R = crate::R<GpioFfedcSpec>;
#[doc = "Register `GPIO_FFEDC` writer"]
pub type W = crate::W<GpioFfedcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ffedc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ffedc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioFfedcSpec;
impl crate::RegisterSpec for GpioFfedcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_ffedc::R`](R) reader structure"]
impl crate::Readable for GpioFfedcSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_ffedc::W`](W) writer structure"]
impl crate::Writable for GpioFfedcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_FFEDC to value 0"]
impl crate::Resettable for GpioFfedcSpec {}
