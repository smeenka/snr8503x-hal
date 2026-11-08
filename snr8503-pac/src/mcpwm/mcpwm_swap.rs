#[doc = "Register `MCPWM_SWAP` reader"]
pub type R = crate::R<McpwmSwapSpec>;
#[doc = "Register `MCPWM_SWAP` writer"]
pub type W = crate::W<McpwmSwapSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_swap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_swap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McpwmSwapSpec;
impl crate::RegisterSpec for McpwmSwapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcpwm_swap::R`](R) reader structure"]
impl crate::Readable for McpwmSwapSpec {}
#[doc = "`write(|w| ..)` method takes [`mcpwm_swap::W`](W) writer structure"]
impl crate::Writable for McpwmSwapSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCPWM_SWAP to value 0"]
impl crate::Resettable for McpwmSwapSpec {}
