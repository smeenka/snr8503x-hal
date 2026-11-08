#[doc = "Register `DSP_QUO` reader"]
pub type R = crate::R<DspQuoSpec>;
#[doc = "Register `DSP_QUO` writer"]
pub type W = crate::W<DspQuoSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp_quo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp_quo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DspQuoSpec;
impl crate::RegisterSpec for DspQuoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp_quo::R`](R) reader structure"]
impl crate::Readable for DspQuoSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp_quo::W`](W) writer structure"]
impl crate::Writable for DspQuoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSP_QUO to value 0"]
impl crate::Resettable for DspQuoSpec {}
