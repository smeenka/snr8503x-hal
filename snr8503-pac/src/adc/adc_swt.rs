#[doc = "Register `ADC_SWT` reader"]
pub type R = crate::R<AdcSwtSpec>;
#[doc = "Register `ADC_SWT` writer"]
pub type W = crate::W<AdcSwtSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_swt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_swt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcSwtSpec;
impl crate::RegisterSpec for AdcSwtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_swt::R`](R) reader structure"]
impl crate::Readable for AdcSwtSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_swt::W`](W) writer structure"]
impl crate::Writable for AdcSwtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC_SWT to value 0"]
impl crate::Resettable for AdcSwtSpec {}
