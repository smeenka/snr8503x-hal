#[doc = "Register `SYS_WR_PROTECT` reader"]
pub type R = crate::R<SysWrProtectSpec>;
#[doc = "Register `SYS_WR_PROTECT` writer"]
pub type W = crate::W<SysWrProtectSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Write Protect (Aliases SYS_PROTECT, SYS_PROT)\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_wr_protect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_wr_protect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysWrProtectSpec;
impl crate::RegisterSpec for SysWrProtectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_wr_protect::R`](R) reader structure"]
impl crate::Readable for SysWrProtectSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_wr_protect::W`](W) writer structure"]
impl crate::Writable for SysWrProtectSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_WR_PROTECT to value 0"]
impl crate::Resettable for SysWrProtectSpec {}
