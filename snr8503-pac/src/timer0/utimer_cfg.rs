#[doc = "Register `UTIMER_CFG` reader"]
pub type R = crate::R<UtimerCfgSpec>;
#[doc = "Register `UTIMER_CFG` writer"]
pub type W = crate::W<UtimerCfgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`utimer_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utimer_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UtimerCfgSpec;
impl crate::RegisterSpec for UtimerCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`utimer_cfg::R`](R) reader structure"]
impl crate::Readable for UtimerCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`utimer_cfg::W`](W) writer structure"]
impl crate::Writable for UtimerCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UTIMER_CFG to value 0"]
impl crate::Resettable for UtimerCfgSpec {}
