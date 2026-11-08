#[doc = "Register `CMP_TCLK` reader"]
pub type R = crate::R<CmpTclkSpec>;
#[doc = "Register `CMP_TCLK` writer"]
pub type W = crate::W<CmpTclkSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp_tclk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp_tclk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpTclkSpec;
impl crate::RegisterSpec for CmpTclkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp_tclk::R`](R) reader structure"]
impl crate::Readable for CmpTclkSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp_tclk::W`](W) writer structure"]
impl crate::Writable for CmpTclkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMP_TCLK to value 0"]
impl crate::Resettable for CmpTclkSpec {}
