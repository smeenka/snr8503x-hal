#[doc = "Register `GPIO_F7654` reader"]
pub type R = crate::R<GpioF7654Spec>;
#[doc = "Register `GPIO_F7654` writer"]
pub type W = crate::W<GpioF7654Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_f7654::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_f7654::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioF7654Spec;
impl crate::RegisterSpec for GpioF7654Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_f7654::R`](R) reader structure"]
impl crate::Readable for GpioF7654Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_f7654::W`](W) writer structure"]
impl crate::Writable for GpioF7654Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_F7654 to value 0"]
impl crate::Resettable for GpioF7654Spec {}
