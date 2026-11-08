#[doc = "Register `UTIMER_IE` reader"]
pub type R = crate::R<UtimerIeSpec>;
#[doc = "Register `UTIMER_IE` writer"]
pub type W = crate::W<UtimerIeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`utimer_ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utimer_ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UtimerIeSpec;
impl crate::RegisterSpec for UtimerIeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`utimer_ie::R`](R) reader structure"]
impl crate::Readable for UtimerIeSpec {}
#[doc = "`write(|w| ..)` method takes [`utimer_ie::W`](W) writer structure"]
impl crate::Writable for UtimerIeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UTIMER_IE to value 0"]
impl crate::Resettable for UtimerIeSpec {}
