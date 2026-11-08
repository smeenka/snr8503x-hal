#[doc = "Register `MCPWM_TH11` reader"]
pub type R = crate::R<McpwmTh11Spec>;
#[doc = "Register `MCPWM_TH11` writer"]
pub type W = crate::W<McpwmTh11Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_th11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_th11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McpwmTh11Spec;
impl crate::RegisterSpec for McpwmTh11Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mcpwm_th11::R`](R) reader structure"]
impl crate::Readable for McpwmTh11Spec {}
#[doc = "`write(|w| ..)` method takes [`mcpwm_th11::W`](W) writer structure"]
impl crate::Writable for McpwmTh11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCPWM_TH11 to value 0"]
impl crate::Resettable for McpwmTh11Spec {}
