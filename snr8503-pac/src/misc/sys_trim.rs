#[doc = "Register `SYS_TRIM` reader"]
pub type R = crate::R<SysTrimSpec>;
#[doc = "Register `SYS_TRIM` writer"]
pub type W = crate::W<SysTrimSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_trim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_trim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysTrimSpec;
impl crate::RegisterSpec for SysTrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_trim::R`](R) reader structure"]
impl crate::Readable for SysTrimSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_trim::W`](W) writer structure"]
impl crate::Writable for SysTrimSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_TRIM to value 0"]
impl crate::Resettable for SysTrimSpec {}
