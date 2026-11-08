#[doc = "Register `ADC_IF` reader"]
pub type R = crate::R<AdcIfSpec>;
#[doc = "Register `ADC_IF` writer"]
pub type W = crate::W<AdcIfSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_if::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_if::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcIfSpec;
impl crate::RegisterSpec for AdcIfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_if::R`](R) reader structure"]
impl crate::Readable for AdcIfSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_if::W`](W) writer structure"]
impl crate::Writable for AdcIfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC_IF to value 0"]
impl crate::Resettable for AdcIfSpec {}
