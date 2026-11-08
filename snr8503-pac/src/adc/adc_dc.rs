#[doc = "Register `ADC_DC` reader"]
pub type R = crate::R<AdcDcSpec>;
#[doc = "Register `ADC_DC` writer"]
pub type W = crate::W<AdcDcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcDcSpec;
impl crate::RegisterSpec for AdcDcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_dc::R`](R) reader structure"]
impl crate::Readable for AdcDcSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_dc::W`](W) writer structure"]
impl crate::Writable for AdcDcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC_DC to value 0"]
impl crate::Resettable for AdcDcSpec {}
