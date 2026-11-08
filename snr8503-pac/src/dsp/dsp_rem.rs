#[doc = "Register `DSP_REM` reader"]
pub type R = crate::R<DspRemSpec>;
#[doc = "Register `DSP_REM` writer"]
pub type W = crate::W<DspRemSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp_rem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp_rem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DspRemSpec;
impl crate::RegisterSpec for DspRemSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp_rem::R`](R) reader structure"]
impl crate::Readable for DspRemSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp_rem::W`](W) writer structure"]
impl crate::Writable for DspRemSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSP_REM to value 0"]
impl crate::Resettable for DspRemSpec {}
