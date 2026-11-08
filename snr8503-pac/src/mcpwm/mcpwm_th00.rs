#[doc = "Register `MCPWM_TH00` reader"]
pub type R = crate::R<McpwmTh00Spec>;
#[doc = "Register `MCPWM_TH00` writer"]
pub type W = crate::W<McpwmTh00Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_th00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_th00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McpwmTh00Spec;
impl crate::RegisterSpec for McpwmTh00Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mcpwm_th00::R`](R) reader structure"]
impl crate::Readable for McpwmTh00Spec {}
#[doc = "`write(|w| ..)` method takes [`mcpwm_th00::W`](W) writer structure"]
impl crate::Writable for McpwmTh00Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCPWM_TH00 to value 0"]
impl crate::Resettable for McpwmTh00Spec {}
