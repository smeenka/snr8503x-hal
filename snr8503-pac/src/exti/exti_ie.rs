#[doc = "Register `EXTI_IE` reader"]
pub type R = crate::R<ExtiIeSpec>;
#[doc = "Register `EXTI_IE` writer"]
pub type W = crate::W<ExtiIeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`exti_ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtiIeSpec;
impl crate::RegisterSpec for ExtiIeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_ie::R`](R) reader structure"]
impl crate::Readable for ExtiIeSpec {}
#[doc = "`write(|w| ..)` method takes [`exti_ie::W`](W) writer structure"]
impl crate::Writable for ExtiIeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTI_IE to value 0"]
impl crate::Resettable for ExtiIeSpec {}
