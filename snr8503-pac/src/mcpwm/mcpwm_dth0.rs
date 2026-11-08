#[doc = "Register `MCPWM_DTH0` reader"]
pub type R = crate::R<McpwmDth0Spec>;
#[doc = "Register `MCPWM_DTH0` writer"]
pub type W = crate::W<McpwmDth0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_dth0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_dth0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McpwmDth0Spec;
impl crate::RegisterSpec for McpwmDth0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcpwm_dth0::R`](R) reader structure"]
impl crate::Readable for McpwmDth0Spec {}
#[doc = "`write(|w| ..)` method takes [`mcpwm_dth0::W`](W) writer structure"]
impl crate::Writable for McpwmDth0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCPWM_DTH0 to value 0"]
impl crate::Resettable for McpwmDth0Spec {}
