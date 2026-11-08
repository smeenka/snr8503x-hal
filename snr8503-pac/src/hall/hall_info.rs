#[doc = "Register `HALL_INFO` reader"]
pub type R = crate::R<HallInfoSpec>;
#[doc = "Register `HALL_INFO` writer"]
pub type W = crate::W<HallInfoSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`hall_info::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hall_info::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HallInfoSpec;
impl crate::RegisterSpec for HallInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hall_info::R`](R) reader structure"]
impl crate::Readable for HallInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`hall_info::W`](W) writer structure"]
impl crate::Writable for HallInfoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HALL_INFO to value 0"]
impl crate::Resettable for HallInfoSpec {}
