#[doc = "Register `UTIMER_TH` reader"]
pub type R = crate::R<UtimerThSpec>;
#[doc = "Register `UTIMER_TH` writer"]
pub type W = crate::W<UtimerThSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`utimer_th::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utimer_th::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UtimerThSpec;
impl crate::RegisterSpec for UtimerThSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`utimer_th::R`](R) reader structure"]
impl crate::Readable for UtimerThSpec {}
#[doc = "`write(|w| ..)` method takes [`utimer_th::W`](W) writer structure"]
impl crate::Writable for UtimerThSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UTIMER_TH to value 0"]
impl crate::Resettable for UtimerThSpec {}
