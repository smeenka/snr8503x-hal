#[doc = "Register `ADC_IE` reader"]
pub type R = crate::R<AdcIeSpec>;
#[doc = "Register `ADC_IE` writer"]
pub type W = crate::W<AdcIeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcIeSpec;
impl crate::RegisterSpec for AdcIeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_ie::R`](R) reader structure"]
impl crate::Readable for AdcIeSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_ie::W`](W) writer structure"]
impl crate::Writable for AdcIeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC_IE to value 0"]
impl crate::Resettable for AdcIeSpec {}
