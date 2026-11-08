#[doc = "Register `HALL_TH` reader"]
pub type R = crate::R<HallThSpec>;
#[doc = "Register `HALL_TH` writer"]
pub type W = crate::W<HallThSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`hall_th::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hall_th::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HallThSpec;
impl crate::RegisterSpec for HallThSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hall_th::R`](R) reader structure"]
impl crate::Readable for HallThSpec {}
#[doc = "`write(|w| ..)` method takes [`hall_th::W`](W) writer structure"]
impl crate::Writable for HallThSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HALL_TH to value 0"]
impl crate::Resettable for HallThSpec {}
