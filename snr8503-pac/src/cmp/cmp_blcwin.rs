#[doc = "Register `CMP_BLCWIN` reader"]
pub type R = crate::R<CmpBlcwinSpec>;
#[doc = "Register `CMP_BLCWIN` writer"]
pub type W = crate::W<CmpBlcwinSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp_blcwin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp_blcwin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpBlcwinSpec;
impl crate::RegisterSpec for CmpBlcwinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp_blcwin::R`](R) reader structure"]
impl crate::Readable for CmpBlcwinSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp_blcwin::W`](W) writer structure"]
impl crate::Writable for CmpBlcwinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMP_BLCWIN to value 0"]
impl crate::Resettable for CmpBlcwinSpec {}
