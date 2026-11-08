#[doc = "Register `SYS_TEST` reader"]
pub type R = crate::R<SysTestSpec>;
#[doc = "Register `SYS_TEST` writer"]
pub type W = crate::W<SysTestSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysTestSpec;
impl crate::RegisterSpec for SysTestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_test::R`](R) reader structure"]
impl crate::Readable for SysTestSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_test::W`](W) writer structure"]
impl crate::Writable for SysTestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_TEST to value 0"]
impl crate::Resettable for SysTestSpec {}
