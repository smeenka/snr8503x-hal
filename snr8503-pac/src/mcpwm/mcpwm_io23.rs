#[doc = "Register `MCPWM_IO23` reader"]
pub type R = crate::R<McpwmIo23Spec>;
#[doc = "Register `MCPWM_IO23` writer"]
pub type W = crate::W<McpwmIo23Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_io23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_io23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McpwmIo23Spec;
impl crate::RegisterSpec for McpwmIo23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcpwm_io23::R`](R) reader structure"]
impl crate::Readable for McpwmIo23Spec {}
#[doc = "`write(|w| ..)` method takes [`mcpwm_io23::W`](W) writer structure"]
impl crate::Writable for McpwmIo23Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCPWM_IO23 to value 0"]
impl crate::Resettable for McpwmIo23Spec {}
