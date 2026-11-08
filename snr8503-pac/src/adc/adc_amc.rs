#[doc = "Register `ADC_AMC` reader"]
pub type R = crate::R<AdcAmcSpec>;
#[doc = "Register `ADC_AMC` writer"]
pub type W = crate::W<AdcAmcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_amc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_amc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcAmcSpec;
impl crate::RegisterSpec for AdcAmcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_amc::R`](R) reader structure"]
impl crate::Readable for AdcAmcSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_amc::W`](W) writer structure"]
impl crate::Writable for AdcAmcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC_AMC to value 0"]
impl crate::Resettable for AdcAmcSpec {}
