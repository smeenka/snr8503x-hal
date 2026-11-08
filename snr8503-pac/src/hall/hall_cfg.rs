#[doc = "Register `HALL_CFG` reader"]
pub type R = crate::R<HallCfgSpec>;
#[doc = "Register `HALL_CFG` writer"]
pub type W = crate::W<HallCfgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`hall_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hall_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HallCfgSpec;
impl crate::RegisterSpec for HallCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hall_cfg::R`](R) reader structure"]
impl crate::Readable for HallCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`hall_cfg::W`](W) writer structure"]
impl crate::Writable for HallCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HALL_CFG to value 0"]
impl crate::Resettable for HallCfgSpec {}
