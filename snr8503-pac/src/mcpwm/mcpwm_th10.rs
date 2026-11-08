#[doc = "Register `MCPWM_TH10` reader"]
pub type R = crate::R<McpwmTh10Spec>;
#[doc = "Register `MCPWM_TH10` writer"]
pub type W = crate::W<McpwmTh10Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_th10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_th10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McpwmTh10Spec;
impl crate::RegisterSpec for McpwmTh10Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mcpwm_th10::R`](R) reader structure"]
impl crate::Readable for McpwmTh10Spec {}
#[doc = "`write(|w| ..)` method takes [`mcpwm_th10::W`](W) writer structure"]
impl crate::Writable for McpwmTh10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCPWM_TH10 to value 0"]
impl crate::Resettable for McpwmTh10Spec {}
