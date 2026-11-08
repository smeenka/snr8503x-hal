#[doc = "Register `SYS_DBG_CFG` reader"]
pub type R = crate::R<SysDbgCfgSpec>;
#[doc = "Register `SYS_DBG_CFG` writer"]
pub type W = crate::W<SysDbgCfgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_dbg_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_dbg_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysDbgCfgSpec;
impl crate::RegisterSpec for SysDbgCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_dbg_cfg::R`](R) reader structure"]
impl crate::Readable for SysDbgCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_dbg_cfg::W`](W) writer structure"]
impl crate::Writable for SysDbgCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_DBG_CFG to value 0"]
impl crate::Resettable for SysDbgCfgSpec {}
