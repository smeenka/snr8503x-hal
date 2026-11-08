#[doc = "Register `MCPWM_TMR[%s]` reader"]
pub type R = crate::R<McpwmTmrSpec>;
#[doc = "Register `MCPWM_TMR[%s]` writer"]
pub type W = crate::W<McpwmTmrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "MCPWM Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_tmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_tmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McpwmTmrSpec;
impl crate::RegisterSpec for McpwmTmrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mcpwm_tmr::R`](R) reader structure"]
impl crate::Readable for McpwmTmrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcpwm_tmr::W`](W) writer structure"]
impl crate::Writable for McpwmTmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCPWM_TMR[%s] to value 0"]
impl crate::Resettable for McpwmTmrSpec {}
