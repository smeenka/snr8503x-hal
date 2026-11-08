#[doc = "Register `EXTI_CR0` reader"]
pub type R = crate::R<ExtiCr0Spec>;
#[doc = "Register `EXTI_CR0` writer"]
pub type W = crate::W<ExtiCr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`exti_cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtiCr0Spec;
impl crate::RegisterSpec for ExtiCr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_cr0::R`](R) reader structure"]
impl crate::Readable for ExtiCr0Spec {}
#[doc = "`write(|w| ..)` method takes [`exti_cr0::W`](W) writer structure"]
impl crate::Writable for ExtiCr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTI_CR0 to value 0"]
impl crate::Resettable for ExtiCr0Spec {}
