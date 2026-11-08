#[doc = "Register `CMP_IE` reader"]
pub type R = crate::R<CmpIeSpec>;
#[doc = "Register `CMP_IE` writer"]
pub type W = crate::W<CmpIeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp_ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp_ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpIeSpec;
impl crate::RegisterSpec for CmpIeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp_ie::R`](R) reader structure"]
impl crate::Readable for CmpIeSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp_ie::W`](W) writer structure"]
impl crate::Writable for CmpIeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMP_IE to value 0"]
impl crate::Resettable for CmpIeSpec {}
