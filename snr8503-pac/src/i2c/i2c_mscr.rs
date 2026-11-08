#[doc = "Register `I2C_MSCR` reader"]
pub type R = crate::R<I2cMscrSpec>;
#[doc = "Register `I2C_MSCR` writer"]
pub type W = crate::W<I2cMscrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_mscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_mscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cMscrSpec;
impl crate::RegisterSpec for I2cMscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_mscr::R`](R) reader structure"]
impl crate::Readable for I2cMscrSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_mscr::W`](W) writer structure"]
impl crate::Writable for I2cMscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_MSCR to value 0"]
impl crate::Resettable for I2cMscrSpec {}
