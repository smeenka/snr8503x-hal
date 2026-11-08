#[doc = "Register `DMA_CTRL` reader"]
pub type R = crate::R<DmaCtrlSpec>;
#[doc = "Register `DMA_CTRL` writer"]
pub type W = crate::W<DmaCtrlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCtrlSpec;
impl crate::RegisterSpec for DmaCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_ctrl::R`](R) reader structure"]
impl crate::Readable for DmaCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_ctrl::W`](W) writer structure"]
impl crate::Writable for DmaCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CTRL to value 0"]
impl crate::Resettable for DmaCtrlSpec {}
