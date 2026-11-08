#[doc = "Register `ADC_GEN` reader"]
pub type R = crate::R<AdcGenSpec>;
#[doc = "Register `ADC_GEN` writer"]
pub type W = crate::W<AdcGenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_gen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_gen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcGenSpec;
impl crate::RegisterSpec for AdcGenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_gen::R`](R) reader structure"]
impl crate::Readable for AdcGenSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_gen::W`](W) writer structure"]
impl crate::Writable for AdcGenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC_GEN to value 0"]
impl crate::Resettable for AdcGenSpec {}
