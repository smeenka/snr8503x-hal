#[doc = "Register `MCPWM_TCLK` reader"]
pub type R = crate::R<McpwmTclkSpec>;
#[doc = "Register `MCPWM_TCLK` writer"]
pub type W = crate::W<McpwmTclkSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_tclk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_tclk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McpwmTclkSpec;
impl crate::RegisterSpec for McpwmTclkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcpwm_tclk::R`](R) reader structure"]
impl crate::Readable for McpwmTclkSpec {}
#[doc = "`write(|w| ..)` method takes [`mcpwm_tclk::W`](W) writer structure"]
impl crate::Writable for McpwmTclkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCPWM_TCLK to value 0"]
impl crate::Resettable for McpwmTclkSpec {}
