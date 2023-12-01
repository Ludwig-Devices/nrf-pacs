#[doc = "Register `PDMCLKCTRL` reader"]
pub struct R(crate::R<PDMCLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMCLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMCLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMCLKCTRL` writer"]
pub struct W(crate::W<PDMCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PDMCLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMCLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FREQ` reader - PDM_CLK frequency"]
pub type FREQ_R = crate::FieldReader<u32, FREQ_A>;
#[doc = "PDM_CLK frequency\n\nValue on reset: 0x08400000"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum FREQ_A {
    #[doc = "0x08000000: PDM_CLK = 1.000 MHz -> Sample rate = 15625.0 Hz"]
    _1000K = 0x08000000,
    #[doc = "0x08400000: PDM_CLK = 1.031 MHz -> Sample rate = 16113.3 Hz"]
    DEFAULT = 0x08400000,
    #[doc = "0x08800000: PDM_CLK = 1.063 MHz -> Sample rate = 16601.6 Hz"]
    _1067K = 0x08800000,
    #[doc = "0x0C000000: PDM_CLK = 1.500 MHz -> Sample rate = 23437.5 Hz"]
    _1500K = 0x0C000000,
    #[doc = "0x10000000: PDM_CLK = 2.000 MHz -> Sample rate = 31250.0 Hz"]
    _2000K = 0x10000000,
    #[doc = "0x20000000: PDM_CLK = 4.000 MHz -> Sample rate = 62500.0 Hz"]
    _4000K = 0x20000000,
}
impl From<FREQ_A> for u32 {
    #[inline(always)]
    fn from(variant: FREQ_A) -> Self {
        variant as _
    }
}
impl FREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FREQ_A> {
        match self.bits {
            0x08000000 => Some(FREQ_A::_1000K),
            0x08400000 => Some(FREQ_A::DEFAULT),
            0x08800000 => Some(FREQ_A::_1067K),
            0x0C000000 => Some(FREQ_A::_1500K),
            0x10000000 => Some(FREQ_A::_2000K),
            0x20000000 => Some(FREQ_A::_4000K),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1000K`"]
    #[inline(always)]
    pub fn is_1000k(&self) -> bool {
        *self == FREQ_A::_1000K
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == FREQ_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `_1067K`"]
    #[inline(always)]
    pub fn is_1067k(&self) -> bool {
        *self == FREQ_A::_1067K
    }
    #[doc = "Checks if the value of the field is `_1500K`"]
    #[inline(always)]
    pub fn is_1532k(&self) -> bool {
        *self == FREQ_A::_1500K
    }
    #[doc = "Checks if the value of the field is `_2000K`"]
    #[inline(always)]
    pub fn is_2000k(&self) -> bool {
        *self == FREQ_A::_2000K
    }
    #[doc = "Checks if the value of the field is `_4000K`"]
    #[inline(always)]
    pub fn is_4000k(&self) -> bool {
        *self == FREQ_A::_4000K
    }
}
#[doc = "Field `FREQ` writer - PDM_CLK frequency"]
pub type FREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDMCLKCTRL_SPEC, u32, FREQ_A, 32, O>;
impl<'a, const O: u8> FREQ_W<'a, O> {
    #[doc = "PDM_CLK = 1.000 MHz -> Sample rate = 15625.0 Hz"]
    #[inline(always)]
    pub fn _1000k(self) -> &'a mut W {
        self.variant(FREQ_A::_1000K)
    }
    #[doc = "PDM_CLK = 1.031 MHz -> Sample rate = 16113.3 Hz"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(FREQ_A::DEFAULT)
    }
    #[doc = "PDM_CLK = 1.063 MHz -> Sample rate = 16601.6 Hz"]
    #[inline(always)]
    pub fn _1067k(self) -> &'a mut W {
        self.variant(FREQ_A::_1067K)
    }
    #[doc = "PDM_CLK = 1.500 MHz -> Sample rate = 23437.5 Hz"]
    #[inline(always)]
    pub fn _1500k(self) -> &'a mut W {
        self.variant(FREQ_A::_1500K)
    }
    #[doc = "PDM_CLK = 2.000 MHz -> Sample rate = 31250.0 Hz"]
    #[inline(always)]
    pub fn _2000k(self) -> &'a mut W {
        self.variant(FREQ_A::_2000K)
    }
    #[doc = "PDM_CLK = 4.000 MHz -> Sample rate = 62500.0 Hz"]
    #[inline(always)]
    pub fn _4000k(self) -> &'a mut W {
        self.variant(FREQ_A::_4000K)
    }
}
impl R {
    #[doc = "Bits 0:31 - PDM_CLK frequency"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PDM_CLK frequency"]
    #[inline(always)]
    pub fn freq(&mut self) -> FREQ_W<0> {
        FREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDM clock generator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdmclkctrl](index.html) module"]
pub struct PDMCLKCTRL_SPEC;
impl crate::RegisterSpec for PDMCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdmclkctrl::R](R) reader structure"]
impl crate::Readable for PDMCLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdmclkctrl::W](W) writer structure"]
impl crate::Writable for PDMCLKCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMCLKCTRL to value 0x0840_0000"]
impl crate::Resettable for PDMCLKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0840_0000
    }
}
