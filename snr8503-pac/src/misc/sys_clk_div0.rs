#[doc = "Register `SYS_CLK_DIV0` reader"]
pub type R = crate::R<SysClkDiv0Spec>;
#[doc = "Register `SYS_CLK_DIV0` writer"]
pub type W = crate::W<SysClkDiv0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "SPI clock div\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_clk_div0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_clk_div0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysClkDiv0Spec;
impl crate::RegisterSpec for SysClkDiv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_clk_div0::R`](R) reader structure"]
impl crate::Readable for SysClkDiv0Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_clk_div0::W`](W) writer structure"]
impl crate::Writable for SysClkDiv0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_CLK_DIV0 to value 0"]
impl crate::Resettable for SysClkDiv0Spec {}
