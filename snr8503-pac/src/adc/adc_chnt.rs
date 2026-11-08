#[doc = "Register `ADC_CHNT` reader"]
pub type R = crate::R<AdcChntSpec>;
#[doc = "Register `ADC_CHNT` writer"]
pub type W = crate::W<AdcChntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_chnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_chnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcChntSpec;
impl crate::RegisterSpec for AdcChntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_chnt::R`](R) reader structure"]
impl crate::Readable for AdcChntSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_chnt::W`](W) writer structure"]
impl crate::Writable for AdcChntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC_CHNT to value 0"]
impl crate::Resettable for AdcChntSpec {}
