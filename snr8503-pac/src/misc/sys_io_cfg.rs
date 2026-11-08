#[doc = "Register `SYS_IO_CFG` reader"]
pub type R = crate::R<SysIoCfgSpec>;
#[doc = "Register `SYS_IO_CFG` writer"]
pub type W = crate::W<SysIoCfgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_io_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_io_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysIoCfgSpec;
impl crate::RegisterSpec for SysIoCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_io_cfg::R`](R) reader structure"]
impl crate::Readable for SysIoCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_io_cfg::W`](W) writer structure"]
impl crate::Writable for SysIoCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_IO_CFG to value 0"]
impl crate::Resettable for SysIoCfgSpec {}
