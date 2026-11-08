#[doc = "Register `SYS_FLSE` reader"]
pub type R = crate::R<SysFlseSpec>;
#[doc = "Register `SYS_FLSE` writer"]
pub type W = crate::W<SysFlseSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_flse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_flse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysFlseSpec;
impl crate::RegisterSpec for SysFlseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_flse::R`](R) reader structure"]
impl crate::Readable for SysFlseSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_flse::W`](W) writer structure"]
impl crate::Writable for SysFlseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_FLSE to value 0"]
impl crate::Resettable for SysFlseSpec {}
