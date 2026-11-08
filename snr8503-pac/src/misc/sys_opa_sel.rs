#[doc = "Register `SYS_OPA_SEL` reader"]
pub type R = crate::R<SysOpaSelSpec>;
#[doc = "Register `SYS_OPA_SEL` writer"]
pub type W = crate::W<SysOpaSelSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_opa_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_opa_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysOpaSelSpec;
impl crate::RegisterSpec for SysOpaSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_opa_sel::R`](R) reader structure"]
impl crate::Readable for SysOpaSelSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_opa_sel::W`](W) writer structure"]
impl crate::Writable for SysOpaSelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_OPA_SEL to value 0"]
impl crate::Resettable for SysOpaSelSpec {}
