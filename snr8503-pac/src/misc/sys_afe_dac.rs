#[doc = "Register `SYS_AFE_DAC` reader"]
pub type R = crate::R<SysAfeDacSpec>;
#[doc = "Register `SYS_AFE_DAC` writer"]
pub type W = crate::W<SysAfeDacSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_afe_dac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_afe_dac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysAfeDacSpec;
impl crate::RegisterSpec for SysAfeDacSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_afe_dac::R`](R) reader structure"]
impl crate::Readable for SysAfeDacSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_afe_dac::W`](W) writer structure"]
impl crate::Writable for SysAfeDacSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_AFE_DAC to value 0"]
impl crate::Resettable for SysAfeDacSpec {}
