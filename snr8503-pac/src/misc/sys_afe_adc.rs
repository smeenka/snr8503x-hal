#[doc = "Register `SYS_AFE_ADC` reader"]
pub type R = crate::R<SysAfeAdcSpec>;
#[doc = "Register `SYS_AFE_ADC` writer"]
pub type W = crate::W<SysAfeAdcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_afe_adc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_afe_adc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysAfeAdcSpec;
impl crate::RegisterSpec for SysAfeAdcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_afe_adc::R`](R) reader structure"]
impl crate::Readable for SysAfeAdcSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_afe_adc::W`](W) writer structure"]
impl crate::Writable for SysAfeAdcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_AFE_ADC to value 0"]
impl crate::Resettable for SysAfeAdcSpec {}
