#[doc = "Register `IWDG_CFG` reader"]
pub type R = crate::R<IwdgCfgSpec>;
#[doc = "Register `IWDG_CFG` writer"]
pub type W = crate::W<IwdgCfgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwdgCfgSpec;
impl crate::RegisterSpec for IwdgCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_cfg::R`](R) reader structure"]
impl crate::Readable for IwdgCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`iwdg_cfg::W`](W) writer structure"]
impl crate::Writable for IwdgCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IWDG_CFG to value 0"]
impl crate::Resettable for IwdgCfgSpec {}
