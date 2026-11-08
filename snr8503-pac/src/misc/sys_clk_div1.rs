#[doc = "Register `SYS_CLK_DIV1` reader"]
pub type R = crate::R<SysClkDiv1Spec>;
#[doc = "Register `SYS_CLK_DIV1` writer"]
pub type W = crate::W<SysClkDiv1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "I2C clock div\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_clk_div1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_clk_div1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysClkDiv1Spec;
impl crate::RegisterSpec for SysClkDiv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_clk_div1::R`](R) reader structure"]
impl crate::Readable for SysClkDiv1Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_clk_div1::W`](W) writer structure"]
impl crate::Writable for SysClkDiv1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_CLK_DIV1 to value 0"]
impl crate::Resettable for SysClkDiv1Spec {}
