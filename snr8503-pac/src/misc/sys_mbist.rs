#[doc = "Register `SYS_MBIST` reader"]
pub type R = crate::R<SysMbistSpec>;
#[doc = "Register `SYS_MBIST` writer"]
pub type W = crate::W<SysMbistSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_mbist::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_mbist::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysMbistSpec;
impl crate::RegisterSpec for SysMbistSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_mbist::R`](R) reader structure"]
impl crate::Readable for SysMbistSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_mbist::W`](W) writer structure"]
impl crate::Writable for SysMbistSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_MBIST to value 0"]
impl crate::Resettable for SysMbistSpec {}
