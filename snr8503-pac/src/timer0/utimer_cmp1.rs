#[doc = "Register `UTIMER_CMP1` reader"]
pub type R = crate::R<UtimerCmp1Spec>;
#[doc = "Register `UTIMER_CMP1` writer"]
pub type W = crate::W<UtimerCmp1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`utimer_cmp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utimer_cmp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UtimerCmp1Spec;
impl crate::RegisterSpec for UtimerCmp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`utimer_cmp1::R`](R) reader structure"]
impl crate::Readable for UtimerCmp1Spec {}
#[doc = "`write(|w| ..)` method takes [`utimer_cmp1::W`](W) writer structure"]
impl crate::Writable for UtimerCmp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UTIMER_CMP1 to value 0"]
impl crate::Resettable for UtimerCmp1Spec {}
