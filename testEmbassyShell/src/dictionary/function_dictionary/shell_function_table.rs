//use super::*;
use super::{FuncDictionary, DictionaryMask, FunctionEntry, ShellFunctions, Dictionary, DictionaryToken};

pub static SHELL_FUNCTIONS_TABLE: [FunctionEntry<ShellFunctions>; 27] = [
    FunctionEntry {function_number: ShellFunctions::SE_CC_getUniqueDeviceId, function_name : "uid" },

    //#[cfg(ALLOW_DEV_SHELL_ACCESS_TO_EVERYTHING)]
    //{ TODO

    FunctionEntry {function_number: ShellFunctions::SE_CC_reboot, function_name : "reboot" },
    FunctionEntry {function_number: ShellFunctions::SE_CC_hardFault, function_name : "hardFault"},
    FunctionEntry {function_number: ShellFunctions::SE_CC_hardReset, function_name : "hardReset"},

    FunctionEntry {function_number: ShellFunctions::SE_CC_testStack, function_name : "testStack" },
    FunctionEntry {function_number: ShellFunctions::SE_CC_testAssert, function_name : "testAssert" },
    FunctionEntry {function_number: ShellFunctions::SE_CC_testInterruptMask, function_name : "testInterruptMask"},

    FunctionEntry {function_number: ShellFunctions::SE_CC_getadc, function_name : "getAdc"},
    FunctionEntry {function_number: ShellFunctions::SE_CC_setPwm, function_name : "setPwm"},
    FunctionEntry {function_number: ShellFunctions::SE_CC_setDac, function_name : "setDac"},

    FunctionEntry {function_number: ShellFunctions::SE_CC_lcdInit, function_name : "lcdInit"},
    FunctionEntry {function_number: ShellFunctions::SE_CC_lcdChar, function_name : "lcdChar"},
    FunctionEntry {function_number: ShellFunctions::SE_CC_lcdColor, function_name : "lcdColor"},
    FunctionEntry {function_number: ShellFunctions::SE_CC_lcdRect, function_name : "lcdRect"},
    FunctionEntry {function_number: ShellFunctions::SE_CC_lcdPixel, function_name : "lcdPixel"},
    FunctionEntry {function_number: ShellFunctions::SE_CC_lcdLine, function_name : "lcdLine"},
    FunctionEntry {function_number: ShellFunctions::SE_CC_lcdStr, function_name : "lcdStr" },
    FunctionEntry {function_number: ShellFunctions::SE_CC_hxClk, function_name : "hxClk"},
    FunctionEntry {function_number: ShellFunctions::SE_CC_hxIn, function_name : "hxIn"},

    FunctionEntry {function_number: ShellFunctions::SE_CC_setLed, function_name : "setLed" },
    FunctionEntry {function_number: ShellFunctions::SE_CC_setAllLeds, function_name : "setAllLeds" },
    FunctionEntry {function_number: ShellFunctions::SE_CC_updateString, function_name : "updateString"},
    FunctionEntry {function_number: ShellFunctions::SE_CC_getLoadCell, function_name : "getLoadCell"},

    FunctionEntry {function_number: ShellFunctions::SE_CC_set24, function_name : "set24"},
    FunctionEntry {function_number: ShellFunctions::SE_CC_setFCDoneFrequency, function_name : "setFCDoneFrequency"},
    FunctionEntry {function_number: ShellFunctions::SE_CC_setFCDoneFrequency, function_name : "setf1"},

    //}
    // TODO - do I need this? it has a NULL in original
    FunctionEntry {function_number: ShellFunctions::SE_CC_last, function_name : ""}
];

pub static EXTENSION_FUNCTION_DICTIONARY : FuncDictionary<ShellFunctions> = FuncDictionary {
    table: &SHELL_FUNCTIONS_TABLE,
    mask: DictionaryMask::YRSHELL_DICTIONARY_EXTENSION_FUNCTION,
};

