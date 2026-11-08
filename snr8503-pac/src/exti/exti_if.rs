#[doc = "Register `EXTI_IF` reader"]
pub type R = crate::R<ExtiIfSpec>;
#[doc = "Register `EXTI_IF` writer"]
pub type W = crate::W<ExtiIfSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`exti_if::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_if::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtiIfSpec;
impl crate::RegisterSpec for ExtiIfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_if::R`](R) reader structure"]
impl crate::Readable for ExtiIfSpec {}
#[doc = "`write(|w| ..)` method takes [`exti_if::W`](W) writer structure"]
impl crate::Writable for ExtiIfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTI_IF to value 0"]
impl crate::Resettable for ExtiIfSpec {}
