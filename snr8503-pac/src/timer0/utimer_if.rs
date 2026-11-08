#[doc = "Register `UTIMER_IF` reader"]
pub type R = crate::R<UtimerIfSpec>;
#[doc = "Register `UTIMER_IF` writer"]
pub type W = crate::W<UtimerIfSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`utimer_if::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utimer_if::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UtimerIfSpec;
impl crate::RegisterSpec for UtimerIfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`utimer_if::R`](R) reader structure"]
impl crate::Readable for UtimerIfSpec {}
#[doc = "`write(|w| ..)` method takes [`utimer_if::W`](W) writer structure"]
impl crate::Writable for UtimerIfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UTIMER_IF to value 0"]
impl crate::Resettable for UtimerIfSpec {}
