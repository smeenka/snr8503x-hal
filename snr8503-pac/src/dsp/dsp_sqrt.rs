#[doc = "Register `DSP_SQRT` reader"]
pub type R = crate::R<DspSqrtSpec>;
#[doc = "Register `DSP_SQRT` writer"]
pub type W = crate::W<DspSqrtSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp_sqrt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp_sqrt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DspSqrtSpec;
impl crate::RegisterSpec for DspSqrtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp_sqrt::R`](R) reader structure"]
impl crate::Readable for DspSqrtSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp_sqrt::W`](W) writer structure"]
impl crate::Writable for DspSqrtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSP_SQRT to value 0"]
impl crate::Resettable for DspSqrtSpec {}
