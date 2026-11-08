#[doc = "Register `DMA_CTMS[%s]` reader"]
pub type R = crate::R<DmaCtmsSpec>;
#[doc = "Register `DMA_CTMS[%s]` writer"]
pub type W = crate::W<DmaCtmsSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_ctms::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ctms::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCtmsSpec;
impl crate::RegisterSpec for DmaCtmsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_ctms::R`](R) reader structure"]
impl crate::Readable for DmaCtmsSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_ctms::W`](W) writer structure"]
impl crate::Writable for DmaCtmsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CTMS[%s] to value 0"]
impl crate::Resettable for DmaCtmsSpec {}
