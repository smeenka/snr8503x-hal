#[doc = "Register `SYS_AFE_DBG` reader"]
pub type R = crate::R<SysAfeDbgSpec>;
#[doc = "Register `SYS_AFE_DBG` writer"]
pub type W = crate::W<SysAfeDbgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_afe_dbg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_afe_dbg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysAfeDbgSpec;
impl crate::RegisterSpec for SysAfeDbgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_afe_dbg::R`](R) reader structure"]
impl crate::Readable for SysAfeDbgSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_afe_dbg::W`](W) writer structure"]
impl crate::Writable for SysAfeDbgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_AFE_DBG to value 0"]
impl crate::Resettable for SysAfeDbgSpec {}
