#[doc = "Register `GPIO_FBA98` reader"]
pub type R = crate::R<GpioFba98Spec>;
#[doc = "Register `GPIO_FBA98` writer"]
pub type W = crate::W<GpioFba98Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_fba98::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_fba98::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioFba98Spec;
impl crate::RegisterSpec for GpioFba98Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_fba98::R`](R) reader structure"]
impl crate::Readable for GpioFba98Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_fba98::W`](W) writer structure"]
impl crate::Writable for GpioFba98Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_FBA98 to value 0"]
impl crate::Resettable for GpioFba98Spec {}
