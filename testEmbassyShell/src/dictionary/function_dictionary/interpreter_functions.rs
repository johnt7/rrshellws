use super::{FunctionEnum, ExecutableEnum};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum InterpreterFunctions {
    SiCcReturn,
    SiCcSpace,
    SiCcCr,
    SiCcLf,
    SiCcCrLf,
    SiCcReset,
    SiCcPrompt,
    SiCcClearPad,
    SiCcDot,
    SiCcDotByte,
    SiCcDotWord,
    SiCcDotX,
    SiCcDotByteX,
    SiCcDotWordX,
    SiCcDotStr,
    SiCcHex,
    SiCcDecimal,
    SiCcStringDef,
    SiCcIf,
    SiCcElse,
    SiCcThen,
    SiCcBegin,
    SiCcUntil,
    SiCcDup,
    SiCcSwap,
    SiCcNip,
    SiCcTuck,
    SiCcDrop,
    SiCcRot,
    SiCcToR,
    SiCcFromR,
    SiCcNegate,
    SiCcPlus,
    SiCcMinus,
    SiCcMultiply,
    SiCcDivide,
    SiCcMod,
    SiCcAnd,
    SiCcOr,
    SiCcXor,
    SiCcLessThan,
    SiCcLessEqual,
    SiCcEqual,
    SiCcGreaterThan,
    SiCcGreaterThanEqual,
    SiCcSetDebug,
    SiCcToC,
    SiCcFromC,
    SiCcParameterStackDepth,
    SiCcReturnStackDepth,
    SiCcCompileStackDepth,
    SiCcParameterStackAt,
    SiCcReturnStackAt,
    SiCcCompileStackAt,
    SiCcNotEqual,
    SiCcZeroEqual,
    SiCcZeroNotEqual,
    SiCcOver,
    SiCc2dup,
    SiCc2drop,
    SiCcStrlen,
    SiCcGetCurrentDictionary,
    SiCcGetCurrentDictionaryEnd,
    SiCcGetCurrentDictionaryLastWord,
    SiCcDelay,
        
    SiCcNextEntry,
    SiCcDotEntryName,
    SiCcEntryToken,
    SiCcKeyQ,
    SiCcAuxKeyQ,
    SiCcAuxIO,
    SiCcMainIO,
    SiCcEmit,
    SiCcAuxEmit,
    
    SiCcShellSize,
    SiCcPrintShellClass,
        
    SiCcdictionarySize,
    SiCcPadSize,
    SiCcNumRegisters,
    SiCcParameterStackSize,
    SiCcReturnStackSize,
    SiCcCompileStackSize,
    SiCcInqSize,
    SiCcAuxInqSize,
    SiCcOutqSize,
    SiCcAuxOutqSize,
    SiCcDictionaryClear,
    SiCcSetCommandEcho,
    SiCcSetExpandCR,
    SiCcSysticks,
    SiCcMicros,
    SiCcMillis,
    
    SiCcBang,
    SiCAt,
    SiCcClearStats,
    SiCcSliceStats,
    SiCcPrintSliceName,
        
    SiCcFind,
    SiCcFindEntry,
    SiCcFetchToken,
    
    SiCcLShift,
    SiCcIRShift,
    SiCcrShift,
    SiCcV_return,
    SiCcV_uint16,
    SiCcV_uint32,
    SiCcV_nint16,
    
    SiCcNoop,
    SiCcV_if,
    SiCcV_else,
    SiCcV_then,
    SiCcV_begin,
    SiCcV_until,
    SiCcV_string,
    SiCcIsFunction,
    SiCcV_dictionaryMask,
    SiCcV_dictionaryRelative,
    
    SiCcNot,
    SiCcDotRawStr,
    SiCcDotN,
    
    SiCcSetBaud,
    SiCcSysticksPerSecond,
    
    SiCcPrintMainFileName,
    
    SiCcSetPromptEnable,
    
    SiCcStrBang,
    SiCcCharBang,
    SiCcCharAt,
    SiCcTextIO,
    
    SiCcNextDelay,
        
    SiCcIsEntryMatch,
    
    //#[cfg(YRSHELL_INTERPRETER_FLOATING_POINT)]
    //{
        SiCcDotg,
        SiCcDotf,
        SiCcDote,
        SiCcFLessThan,
        SiCcFLessEqual,
        SiCcFEqual,
        SiCcFGreaterThan,
        SiCcFGreaterThanEqual,
            
        SiCcFPlus,
        SiCcFMinus,
        SiCcFMultiply,
        SiCcFDivide,
        SiCcFPI,
        SiCcFSin,
        SiCcFCos,
        SiCcFTan,
        SiCcFaSin,
        SiCcFaCos,
        SiCcFaTan,
        SiCcFExp,
        SiCcFLog,
        SiCcFPow,
        SiCcFLog10,
        SiCcFRound,
        SiCcFloatToInt,
        SiCcIntToFloat,
//    }
     SiCcEchoLoop,
    
    }

    impl FunctionEnum for InterpreterFunctions {}
    impl ExecutableEnum for InterpreterFunctions {}