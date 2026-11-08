#[doc = "Register `SYS_FLST` reader"]
pub type R = crate::R<SysFlstSpec>;
#[doc = "Register `SYS_FLST` writer"]
pub type W = crate::W<SysFlstSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_flst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_flst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysFlstSpec;
impl crate::RegisterSpec for SysFlstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_flst::R`](R) reader structure"]
impl crate::Readable for SysFlstSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_flst::W`](W) writer structure"]
impl crate::Writable for SysFlstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_FLST to value 0"]
impl crate::Resettable for SysFlstSpec {}
