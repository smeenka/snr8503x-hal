#[doc = "Register `DMA_SADR[%s]` reader"]
pub type R = crate::R<DmaSadrSpec>;
#[doc = "Register `DMA_SADR[%s]` writer"]
pub type W = crate::W<DmaSadrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_sadr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_sadr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaSadrSpec;
impl crate::RegisterSpec for DmaSadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_sadr::R`](R) reader structure"]
impl crate::Readable for DmaSadrSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_sadr::W`](W) writer structure"]
impl crate::Writable for DmaSadrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_SADR[%s] to value 0"]
impl crate::Resettable for DmaSadrSpec {}
