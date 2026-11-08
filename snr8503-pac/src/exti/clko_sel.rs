#[doc = "Register `CLKO_SEL` reader"]
pub type R = crate::R<ClkoSelSpec>;
#[doc = "Register `CLKO_SEL` writer"]
pub type W = crate::W<ClkoSelSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clko_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clko_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkoSelSpec;
impl crate::RegisterSpec for ClkoSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clko_sel::R`](R) reader structure"]
impl crate::Readable for ClkoSelSpec {}
#[doc = "`write(|w| ..)` method takes [`clko_sel::W`](W) writer structure"]
impl crate::Writable for ClkoSelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKO_SEL to value 0"]
impl crate::Resettable for ClkoSelSpec {}
