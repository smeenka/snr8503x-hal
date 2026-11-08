#[doc = "Register `HALL_CNT` reader"]
pub type R = crate::R<HallCntSpec>;
#[doc = "Register `HALL_CNT` writer"]
pub type W = crate::W<HallCntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`hall_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hall_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HallCntSpec;
impl crate::RegisterSpec for HallCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hall_cnt::R`](R) reader structure"]
impl crate::Readable for HallCntSpec {}
#[doc = "`write(|w| ..)` method takes [`hall_cnt::W`](W) writer structure"]
impl crate::Writable for HallCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HALL_CNT to value 0"]
impl crate::Resettable for HallCntSpec {}
