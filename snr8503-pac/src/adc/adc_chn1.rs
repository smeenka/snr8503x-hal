#[doc = "Register `ADC_CHN1` reader"]
pub type R = crate::R<AdcChn1Spec>;
#[doc = "Register `ADC_CHN1` writer"]
pub type W = crate::W<AdcChn1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_chn1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_chn1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcChn1Spec;
impl crate::RegisterSpec for AdcChn1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_chn1::R`](R) reader structure"]
impl crate::Readable for AdcChn1Spec {}
#[doc = "`write(|w| ..)` method takes [`adc_chn1::W`](W) writer structure"]
impl crate::Writable for AdcChn1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC_CHN1 to value 0"]
impl crate::Resettable for AdcChn1Spec {}
