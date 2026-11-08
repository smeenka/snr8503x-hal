#[doc = "Register `HALL_WIDTH` reader"]
pub type R = crate::R<HallWidthSpec>;
#[doc = "Register `HALL_WIDTH` writer"]
pub type W = crate::W<HallWidthSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`hall_width::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hall_width::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HallWidthSpec;
impl crate::RegisterSpec for HallWidthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hall_width::R`](R) reader structure"]
impl crate::Readable for HallWidthSpec {}
#[doc = "`write(|w| ..)` method takes [`hall_width::W`](W) writer structure"]
impl crate::Writable for HallWidthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HALL_WIDTH to value 0"]
impl crate::Resettable for HallWidthSpec {}
