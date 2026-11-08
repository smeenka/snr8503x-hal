#[doc = "Register `UTIMER_FLT` reader"]
pub type R = crate::R<UtimerFltSpec>;
#[doc = "Register `UTIMER_FLT` writer"]
pub type W = crate::W<UtimerFltSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`utimer_flt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utimer_flt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UtimerFltSpec;
impl crate::RegisterSpec for UtimerFltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`utimer_flt::R`](R) reader structure"]
impl crate::Readable for UtimerFltSpec {}
#[doc = "`write(|w| ..)` method takes [`utimer_flt::W`](W) writer structure"]
impl crate::Writable for UtimerFltSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UTIMER_FLT to value 0"]
impl crate::Resettable for UtimerFltSpec {}
