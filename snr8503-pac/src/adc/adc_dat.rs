#[doc = "Register `ADC_DAT[%s]` reader"]
pub type R = crate::R<AdcDatSpec>;
#[doc = "Register `ADC_DAT[%s]` writer"]
pub type W = crate::W<AdcDatSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Data Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_dat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_dat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcDatSpec;
impl crate::RegisterSpec for AdcDatSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc_dat::R`](R) reader structure"]
impl crate::Readable for AdcDatSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_dat::W`](W) writer structure"]
impl crate::Writable for AdcDatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC_DAT[%s] to value 0"]
impl crate::Resettable for AdcDatSpec {}
