#[doc = "Register `UTIMER_EVT` reader"]
pub type R = crate::R<UtimerEvtSpec>;
#[doc = "Register `UTIMER_EVT` writer"]
pub type W = crate::W<UtimerEvtSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`utimer_evt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utimer_evt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UtimerEvtSpec;
impl crate::RegisterSpec for UtimerEvtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`utimer_evt::R`](R) reader structure"]
impl crate::Readable for UtimerEvtSpec {}
#[doc = "`write(|w| ..)` method takes [`utimer_evt::W`](W) writer structure"]
impl crate::Writable for UtimerEvtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UTIMER_EVT to value 0"]
impl crate::Resettable for UtimerEvtSpec {}
