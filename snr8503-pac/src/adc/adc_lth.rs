#[doc = "Register `ADC_LTH` reader"]
pub type R = crate::R<AdcLthSpec>;
#[doc = "Register `ADC_LTH` writer"]
pub type W = crate::W<AdcLthSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_lth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_lth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcLthSpec;
impl crate::RegisterSpec for AdcLthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_lth::R`](R) reader structure"]
impl crate::Readable for AdcLthSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_lth::W`](W) writer structure"]
impl crate::Writable for AdcLthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC_LTH to value 0"]
impl crate::Resettable for AdcLthSpec {}
