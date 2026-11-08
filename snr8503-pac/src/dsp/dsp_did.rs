#[doc = "Register `DSP_DID` reader"]
pub type R = crate::R<DspDidSpec>;
#[doc = "Register `DSP_DID` writer"]
pub type W = crate::W<DspDidSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp_did::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp_did::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DspDidSpec;
impl crate::RegisterSpec for DspDidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp_did::R`](R) reader structure"]
impl crate::Readable for DspDidSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp_did::W`](W) writer structure"]
impl crate::Writable for DspDidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSP_DID to value 0"]
impl crate::Resettable for DspDidSpec {}
