#[doc = "Register `SYS_FLSP` reader"]
pub type R = crate::R<SysFlspSpec>;
#[doc = "Register `SYS_FLSP` writer"]
pub type W = crate::W<SysFlspSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_flsp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_flsp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysFlspSpec;
impl crate::RegisterSpec for SysFlspSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_flsp::R`](R) reader structure"]
impl crate::Readable for SysFlspSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_flsp::W`](W) writer structure"]
impl crate::Writable for SysFlspSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_FLSP to value 0"]
impl crate::Resettable for SysFlspSpec {}
