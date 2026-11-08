#[doc = "Register `AON_EVT_RCD` reader"]
pub type R = crate::R<AonEvtRcdSpec>;
#[doc = "Register `AON_EVT_RCD` writer"]
pub type W = crate::W<AonEvtRcdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`aon_evt_rcd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon_evt_rcd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonEvtRcdSpec;
impl crate::RegisterSpec for AonEvtRcdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_evt_rcd::R`](R) reader structure"]
impl crate::Readable for AonEvtRcdSpec {}
#[doc = "`write(|w| ..)` method takes [`aon_evt_rcd::W`](W) writer structure"]
impl crate::Writable for AonEvtRcdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AON_EVT_RCD to value 0"]
impl crate::Resettable for AonEvtRcdSpec {}
