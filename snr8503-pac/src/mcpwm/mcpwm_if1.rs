#[doc = "Register `MCPWM_IF1` reader"]
pub type R = crate::R<McpwmIf1Spec>;
#[doc = "Register `MCPWM_IF1` writer"]
pub type W = crate::W<McpwmIf1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_if1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_if1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McpwmIf1Spec;
impl crate::RegisterSpec for McpwmIf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcpwm_if1::R`](R) reader structure"]
impl crate::Readable for McpwmIf1Spec {}
#[doc = "`write(|w| ..)` method takes [`mcpwm_if1::W`](W) writer structure"]
impl crate::Writable for McpwmIf1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCPWM_IF1 to value 0"]
impl crate::Resettable for McpwmIf1Spec {}
