#[doc = "Register `GPIO_F3210` reader"]
pub type R = crate::R<GpioF3210Spec>;
#[doc = "Register `GPIO_F3210` writer"]
pub type W = crate::W<GpioF3210Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_f3210::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_f3210::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioF3210Spec;
impl crate::RegisterSpec for GpioF3210Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_f3210::R`](R) reader structure"]
impl crate::Readable for GpioF3210Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_f3210::W`](W) writer structure"]
impl crate::Writable for GpioF3210Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_F3210 to value 0"]
impl crate::Resettable for GpioF3210Spec {}
