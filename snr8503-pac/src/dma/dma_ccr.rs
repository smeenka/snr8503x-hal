#[doc = "Register `DMA_CCR[%s]` reader"]
pub type R = crate::R<DmaCcrSpec>;
#[doc = "Register `DMA_CCR[%s]` writer"]
pub type W = crate::W<DmaCcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCcrSpec;
impl crate::RegisterSpec for DmaCcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_ccr::R`](R) reader structure"]
impl crate::Readable for DmaCcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_ccr::W`](W) writer structure"]
impl crate::Writable for DmaCcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CCR[%s] to value 0"]
impl crate::Resettable for DmaCcrSpec {}
