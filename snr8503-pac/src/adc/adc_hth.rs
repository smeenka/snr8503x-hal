#[doc = "Register `ADC_HTH` reader"]
pub type R = crate::R<AdcHthSpec>;
#[doc = "Register `ADC_HTH` writer"]
pub type W = crate::W<AdcHthSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_hth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_hth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcHthSpec;
impl crate::RegisterSpec for AdcHthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_hth::R`](R) reader structure"]
impl crate::Readable for AdcHthSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_hth::W`](W) writer structure"]
impl crate::Writable for AdcHthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC_HTH to value 0"]
impl crate::Resettable for AdcHthSpec {}
