#[doc = "Register `MCPWM_PRT` reader"]
pub type R = crate::R<McpwmPrtSpec>;
#[doc = "Register `MCPWM_PRT` writer"]
pub type W = crate::W<McpwmPrtSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_prt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_prt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McpwmPrtSpec;
impl crate::RegisterSpec for McpwmPrtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcpwm_prt::R`](R) reader structure"]
impl crate::Readable for McpwmPrtSpec {}
#[doc = "`write(|w| ..)` method takes [`mcpwm_prt::W`](W) writer structure"]
impl crate::Writable for McpwmPrtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCPWM_PRT to value 0"]
impl crate::Resettable for McpwmPrtSpec {}
