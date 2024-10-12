use super::*;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShellFunctions {
    SE_CC_first = DictionaryMask::YRSHELL_DICTIONARY_EXTENSION_FUNCTION as isize,
	SE_CC_getUniqueDeviceId,

//#ifdef	ALLOW_DEV_SHELL_ACCESS_TO_EVERYTHING
	SE_CC_reboot,
	SE_CC_hardFault,
	SE_CC_hardReset,
	
	SE_CC_testStack,
	SE_CC_testAssert,
	SE_CC_testInterruptMask,

	SE_CC_getadc,
	SE_CC_setPwm,
	SE_CC_setDac,

	SE_CC_lcdInit,
	SE_CC_lcdChar,
	SE_CC_lcdColor,
	SE_CC_lcdRect,
	SE_CC_lcdLine,
	SE_CC_lcdPixel,
	SE_CC_lcdStr,

	SE_CC_hxClk,
	SE_CC_hxIn,

	SE_CC_setLed,
	SE_CC_setAllLeds,
	SE_CC_updateString,
	SE_CC_getLoadCell,

	SE_CC_set24,
	
	SE_CC_setFCDoneFrequency,
//	#endif

   SE_CC_last
}
impl FunctionEnum for ShellFunctions {}
impl ExecutableEnum for ShellFunctions {}
