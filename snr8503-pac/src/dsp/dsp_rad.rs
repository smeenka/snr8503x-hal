#[doc = "Register `DSP_RAD` reader"]
pub type R = crate::R<DspRadSpec>;
#[doc = "Register `DSP_RAD` writer"]
pub type W = crate::W<DspRadSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp_rad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp_rad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DspRadSpec;
impl crate::RegisterSpec for DspRadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp_rad::R`](R) reader structure"]
impl crate::Readable for DspRadSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp_rad::W`](W) writer structure"]
impl crate::Writable for DspRadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSP_RAD to value 0"]
impl crate::Resettable for DspRadSpec {}
