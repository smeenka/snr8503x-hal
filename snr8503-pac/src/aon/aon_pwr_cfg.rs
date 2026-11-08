#[doc = "Register `AON_PWR_CFG` reader"]
pub type R = crate::R<AonPwrCfgSpec>;
#[doc = "Register `AON_PWR_CFG` writer"]
pub type W = crate::W<AonPwrCfgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`aon_pwr_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon_pwr_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonPwrCfgSpec;
impl crate::RegisterSpec for AonPwrCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_pwr_cfg::R`](R) reader structure"]
impl crate::Readable for AonPwrCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`aon_pwr_cfg::W`](W) writer structure"]
impl crate::Writable for AonPwrCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AON_PWR_CFG to value 0"]
impl crate::Resettable for AonPwrCfgSpec {}
