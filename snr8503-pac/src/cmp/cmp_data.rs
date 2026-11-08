#[doc = "Register `CMP_DATA` reader"]
pub type R = crate::R<CmpDataSpec>;
#[doc = "Register `CMP_DATA` writer"]
pub type W = crate::W<CmpDataSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpDataSpec;
impl crate::RegisterSpec for CmpDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp_data::R`](R) reader structure"]
impl crate::Readable for CmpDataSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp_data::W`](W) writer structure"]
impl crate::Writable for CmpDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMP_DATA to value 0"]
impl crate::Resettable for CmpDataSpec {}
