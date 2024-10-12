/*


dictClear

: adc1 dup getAdc . 1 + 
: adc2 0 adc1 adc1 adc1 adc1 adc1 adc1 adc1 adc1 drop cr
// dump adcs
: adc { adc2 1000 delay esc? }

// generate a sawtooth wave
: st 0 0 { dup 0 setDac over 1 setDac 100 + dup 4000 > [ drop 0 ] swap 200 + dup 4000 > [ drop 0 ] swap esc?  }

: 3dup >c >c >c 0 cs@ 1 cs@ 2 cs@ c> c> c>

: lc1 3 >> rot 0xF8 & 8 << rot 0xFC & 3 <<  | | 
: rgb lc1 lcdColor

: tc1 3dup lcdChar 1 + dup 0x7E >= [ drop 0x21 ] swap 1 + swap
: tc5 tc1 tc1 tc1 tc1 tc1
: tc20 tc5 tc5 tc5 tc5

: tlcdLine 0 rot tc20 nip nip
: lcd0 255 255 255 rgb 0 tlcdLine
: lcd1 255 0   0   rgb 1 tlcdLine
: lcd2 0   255 0   rgb 2 tlcdLine
: lcd3 0   0   255 rgb 3 tlcdLine
: lcd4 255 255 0   rgb 4 tlcdLine
: lcd5 0   255 255 rgb 5 tlcdLine

: lcdClear 0 lcdColor 0 0 160 80 lcdRect

: lcdt lcdClear 0x21 lcd0 lcd1 lcd2 lcd3 lcd4 lcd5 drop

: lcdHelo lcdClear 255 255 255 rgb 2 8 s' helo' lcdStr

: lcdGreet lcdClear 0   255 0   rgb 2 1 s' Good Morning Dave>' lcdStr

: x1! 0 !
: x2! 1 !
: y1! 2 !
: y2! 3 !
: xd! 4 !
: yd! 5 !
: m! 6 !
: mx1! 7 !
: mx2! 8 !
: my1! 9 !
: my2! 10 !
: cx1! 11 !
: cx2! 12 !
: cy1! 13 !
: cy2! 14 !
: ms! 15 !
: mc! 16 !
: mcc! 17 !
: mstop! 18 !

: x1@ 0 @
: x2@ 1 @
: y1@ 2 @
: y2@ 3 @
: xd@ 4 @
: yd@ 5 @
: m@ 6 @
: mx1@ 7 @
: mx2@ 8 @
: my1@ 9 @
: my2@ 10 @
: cx1@ 11 @
: cx2@ 12 @
: cy1@ 13 @
: cy2@ 14 @
: ms@ 15 @
: mc@ 16 @
: mcc@ 17 @
: mstop@ 18 @

: mo1_1 60  40  0   0    my2! mx2! my1! mx1! 100 40  0   0   cy2! cx2! cy1! cx1! 
: mo2_1 60  40  159 0    my2! mx2! my1! mx1! 100 40  159 0   cy2! cx2! cy1! cx1! 
: mo3_1 60  40  159 79   my2! mx2! my1! mx1! 100 40  159 79  cy2! cx2! cy1! cx1! 
: mo4_1 60  40  0   79   my2! mx2! my1! mx1! 100 40  0   79  cy2! cx2! cy1! cx1! 

: mo1 mo1_1 { mx1@ my1@ mx2@ my2@ mc@ lcdColor lcdLine cx1@ cy1@ cx2@ cy2@ mcc@ lcdColor lcdLine mx2@ ms@ + dup mx2! dup cx2! 159 > mstop@ | esc? dup [ -1 mstop! ] | }
: mo2 mo2_1 { mx1@ my1@ mx2@ my2@ mc@ lcdColor lcdLine cx1@ cy1@ cx2@ cy2@ mcc@ lcdColor lcdLine my2@ ms@ + dup my2! dup cy2! 79 >  mstop@ | esc? dup [ -1 mstop! ] | }
: mo3 mo3_1 { mx1@ my1@ mx2@ my2@ mc@ lcdColor lcdLine cx1@ cy1@ cx2@ cy2@ mcc@ lcdColor lcdLine mx2@ ms@ - dup mx2! dup cx2! 0 <   mstop@ | esc? dup [ -1 mstop! ] | }
: mo4 mo4_1 { mx1@ my1@ mx2@ my2@ mc@ lcdColor lcdLine cx1@ cy1@ cx2@ cy2@ mcc@ lcdColor lcdLine my2@ ms@ - dup my2! dup cy2! 0 <   mstop@ | esc? dup [ -1 mstop! ] | }

: mor 255 0 0 lc1 mc!
: mog 0 255 0 lc1 mc!
: mob 0 0 255 lc1 mc!
: mow 255 255 255 lc1 mc!

: mo_1 1 ms! 0 0 0 lc1 mcc! 0 mstop! 
: mo_2 mo1 mo2 mo3 mo4 ms@ 1 + dup 10 > [ drop 1 ] ms!
: mo mo_1 { mow mo_2 mor mo_2 mog mo_2 mob mo_2 mstop@ } 

: to { lcdt esc? }

: lcdStart lcdInit 3000 7 setPwm lcdClear lcdt 1000 delay lcdClear lcdHelo 3000 delay lcdClear lcdGreet


compileExtensionDictionary





: tps dup 10000 > [ drop 10000 ] dup 0 == [ drop 1 ] dup 275000000 i>f swap i>f f/ f>i 1 - dup set24 . 30 * . cr 



 { 0 0 0x41 lcdChar esc? }
 { lcdClear esc? }



 0 0 s' helo' lcdStr


255 0   0   rgb 0  0  0x41 lcdChar   1  10 0x41 lcdChar
0   255 0   rgb 0  1  0x42 lcdChar   1  11 0x42 lcdChar
0   0   255 rgb 0  2  0x43 lcdChar   1  12 0x43 lcdChar




255 255 0 rgb 0 { dup dup lcdPixel 1 + dup 40 == } drop 
x1 y1 x2 y2




lcdClear 60 40 0 0 my2! mx2! my1! mx1! 100 40 0 0 cy2! cx2! cy1! cx1! 

lcdClear 255 0 255 rgb
10 0 10 70 lcdLine
0 70 100 70 lcdLine

80 40 160 0 lcdLine



















dictClear

: adc1 dup getAdc . 1 +
: adc2 0 adc1 adc1 adc1 adc1 adc1 adc1 adc1 adc1 drop cr
// dump adcs
: adc { adc2 1000 delay esc? }


CT_0>dictCl
// generate a sawtooth wave
ear

CT_0>

CT_0>: st 0 0 { dup 0 setDac over 1 setDac 100 + dup 4000 > [ drop 0 ] swap 200 + dup 4000 > [ drop 0 ] swap esc?  }

: adc1 dup getA: 3dup >c >c >c 0 cs@ 1 cs@ 2 cs@ c> c> c>

dc . 1 +
: lc1 3 >> rot 0xF8 & 8 << rot 0xFC & 3 <<  | |
: rgb lc1 lcdColor

CT_0>: a
dc2 : tc1 3dup lcdChar 1 + dup 0x7E >= [ drop 0x21 ] swap 1 + swap
: tc5 tc1 tc1 tc1 tc1 tc1
0 adc1 adc1 adc1: tc20 tc5 tc5 tc5 tc5

 adc1 adc1 a: tlcdLine 0 rot tc20 nip nip
: lcd0 255 255 255 rgb 0 tlcdLine
: lcd1 255 0   0   rgb 1 tlcdLine
: lcd2 0   255 0   rgb 2 tlcdLine
dc1 adc1 adc1 dr: lcd3 0   0   255 rgb 3 tlcdLine
: lcd4 255 255 0   rgb 4 tlcdLine
: lcd5 0   255 255 rgb 5 tlcdLine

op cr
: lcdClear 0 lcdColor 0 0 160 80 lcdRect


CT_0>// dump: lcdt lcdClear 0x21 lcd0 lcd1 lcd2 lcd3 lcd4 lcd5 drop

: lcdHelo lcdClear 255 255 255 rgb 2 8 s' helo' lcdStr

 adcs

CT_0>: lcdGreet lcdClear 0   255 0   rgb 2 1 s' Good Morning Dave>' lcdStr
: adc {
: x1! 0 !
: x2! 1 !
 adc2 1000 delay esc? }
: y1! 2 !
: y2! 3 !
: xd! 4 !

CT_0>

CT_0>: yd! 5 !
: m! 6 !
: mx1! 7 !
// generate a sawtooth w: mx2! 8 !
: my1! 9 !
ave

CT_0>: st 0 0 { dup 0 setDac over 1 setDac 100 + dup 4000 > [ drop 0 ] swap 200 + dup 4000 > [ drop 0 ] swap esc? : my2! 10 !
 }
: cx1! 11 !
: cx2! 12 !
: cy1! 13 !
: cy2! 14 !
: ms! 15 !
: mc! 16 !
: mcc! 17 !
: mstop! 18 !

: x1@ 0 @
: x2@ 1 @

CT_0>: y1@ 2 @
: y2@ 3 @
: xd@ 4 @
: yd@ 5 @


CT_0>: 3dup >c >c: m@ 6 @
: mx1@ 7 @
: mx2@ 8 @
: my1@ 9 @
 >c 0 cs@ 1 cs@ 2 cs: my2@ 10 @
: cx1@ 11 @
: cx2@ 12 @
@ c> c> c>
: cy1@ 13 @

CT_0>

CT_0>: lc1 3 >> rot 0xF8 & 8 << rot 0xFC & 3 <<  | |
: cy2@ 14 @
: ms@ 15 @

CT_0>: rg: mc@ 16 @
: mcc@ 17 @
: mstop@ 18 @
b lc1 lcdCol
: mo1_1 60  40  0   0    my2! mx2! my1! mx1! 100 40  0   0   cy2! cx2! cy1! cx1!
: mo2_1 60  40  159 0    my2! mx2! my1! mx1! 100 40  159 0   cy2! cx2! cy1! cx1!
or

CT_0>

CT_0>: t: mo3_1 60  40  159 79   my2! mx2! my1! mx1! 100 40  159 79  cy2! cx2! cy1! cx1!
: mo4_1 60  40  0   79   my2! mx2! my1! mx1! 100 40  0   79  cy2! cx2! cy1! cx1!

: mo1 mo1_1 { mx1@ my1@ mx2@ my2@ mc@ lcdColor lcdLine cx1@ cy1@ cx2@ cy2@ mcc@ lcdColor lcdLine mx2@ ms@ + dup mx2! dup cx2! 159 > mstop@ | esc? dup [ -1 mstop! ] | }
c1 3dup lcdChar 1 + : mo2 mo2_1 { mx1@ my1@ mx2@ my2@ mc@ lcdColor lcdLine cx1@ cy1@ cx2@ cy2@ mcc@ lcdColor lcdLine my2@ ms@ + dup my2! dup cy2! 79 >  mstop@ | esc? dup [ -1 mstop! ] | }
: mo3 mo3_1 { mx1@ my1@ mx2@ my2@ mc@ lcdColor lcdLine cx1@ cy1@ cx2@ cy2@ mcc@ lcdColor lcdLine mx2@ ms@ - dup mx2! dup cx2! 0 <   mstop@ | esc? dup [ -1 mstop! ] | }
dup 0x7E >= [ dr: mo4 mo4_1 { mx1@ my1@ mx2@ my2@ mc@ lcdColor lcdLine cx1@ cy1@ cx2@ cy2@ mcc@ lcdColor lcdLine my2@ ms@ - dup my2! dup cy2! 0 <   mstop@ | esc? dup [ -1 mstop! ] | }

: mor 255 0 0 lc1 mc!
op 0: mog 0 255 0 lc1 mc!
x21 ] swap 1: mob 0 0 255 lc1 mc!
: mow 255 255 255 lc1 mc!

 + swap
: mo_1 1 ms! 0 0 0 lc1 mcc! 0 mstop!
: mo_2 mo1 mo2 mo3 mo4 ms@ 1 + dup 10 > [ drop 1 ] ms!
: mo mo_1 { mow mo_2 mor mo_2 mog mo_2 mob mo_2 mstop@ }

: to { lcdt esc? }

: lcdStart lcdInit 3000 7 setPwm lcdClear lcdt 1000 delay lcdClear lcdHelo 3000 delay lcdClear lcdGreet


CT_0>: tc5 tc1 t
compileExtensionDictionary

c1 tc1 tc1 tc1

CT_0>: tc20 tc5 tc5 tc5 tc5

CT_0>

CT_0>: tlcdLine 0 rot tc20 nip nip

CT_0>: lcd0 255 255 255 rgb 0 tlcdLine

CT_0>: lcd1 255 0   0   rgb 1 tlcdLine

CT_0>: lcd2 0   255 0   rgb 2 tlcdLine

CT_0>: lcd3 0   0   255 rgb 3 tlcdLine

CT_0>: lcd4 255 255 0   rgb 4 tlcdLine

CT_0>: lcd5 0   255 255 rgb 5 tlcdLine

CT_0>

CT_0>: lcdClear 0 lcdColor 0 0 160 80 lcdRect

CT_0>

CT_0>: lcdt lcdClear 0x21 lcd0 lcd1 lcd2 lcd3 lcd4 lcd5 drop

CT_0>

CT_0>: lcdHelo lcdClear 255 255 255 rgb 2 8 s' helo' lcdStr

CT_0>

CT_0>: lcdGreet lcdClear 0   255 0   rgb 2 1 s' Good Morning Dave>' lcdStr

CT_0>

CT_0>: x1! 0 !

CT_0>: x2! 1 !

CT_0>: y1! 2 !

CT_0>: y2! 3 !

CT_0>: xd! 4 !

CT_0>: yd! 5 !

CT_0>: m! 6 !

CT_0>: mx1! 7 !

CT_0>: mx2! 8 !

CT_0>: my1! 9 !

CT_0>: my2! 10 !

CT_0>: cx1! 11 !

CT_0>: cx2! 12 !

CT_0>: cy1! 13 !

CT_0>: cy2! 14 !

CT_0>: ms! 15 !

CT_0>: mc! 16 !

CT_0>: mcc! 17 !

CT_0>: mstop! 18 !

CT_0>

CT_0>: x1@ 0 @

CT_0>: x2@ 1 @

CT_0>: y1@ 2 @

CT_0>: y2@ 3 @

CT_0>: xd@ 4 @

CT_0>: yd@ 5 @

CT_0>: m@ 6 @

CT_0>: mx1@ 7 @

CT_0>: mx2@ 8 @

CT_0>: my1@ 9 @

CT_0>: my2@ 10 @

CT_0>: cx1@ 11 @

CT_0>: cx2@ 12 @

CT_0>: cy1@ 13 @

CT_0>: cy2@ 14 @

CT_0>: ms@ 15 @

CT_0>: mc@ 16 @

CT_0>: mcc@ 17 @

CT_0>: mstop@ 18 @

CT_0>

CT_0>: mo1_1 60  40  0   0    my2! mx2! my1! mx1! 100 40  0   0   cy2! cx2! cy1! cx1!

CT_0>: mo2_1 60  40  159 0    my2! mx2! my1! mx1! 100 40  159 0   cy2! cx2! cy1! cx1!

CT_0>: mo3_1 60  40  159 79   my2! mx2! my1! mx1! 100 40  159 79  cy2! cx2! cy1! cx1!

CT_0>: mo4_1 60  40  0   79   my2! mx2! my1! mx1! 100 40  0   79  cy2! cx2! cy1! cx1!

CT_0>

CT_0>: mo1 mo1_1 { mx1@ my1@ mx2@ my2@ mc@ lcdColor lcdLine cx1@ cy1@ cx2@ cy2@ mcc@ lcdColor lcdLine mx2@ ms@ + dup mx2! dup cx2! 159 > mstop@ | esc? dup [ -1 mstop! ] | }

CT_0>: mo2 mo2_1 { mx1@ my1@ mx2@ my2@ mc@ lcdColor lcdLine cx1@ cy1@ cx2@ cy2@ mcc@ lcdColor lcdLine my2@ ms@ + dup my2! dup cy2! 79 >  mstop@ | esc? dup [ -1 mstop! ] | }

CT_0>: mo3 mo3_1 { mx1@ my1@ mx2@ my2@ mc@ lcdColor lcdLine cx1@ cy1@ cx2@ cy2@ mcc@ lcdColor lcdLine mx2@ ms@ - dup mx2! dup cx2! 0 <   mstop@ | esc? dup [ -1 mstop! ] | }

CT_0>: mo4 mo4_1 { mx1@ my1@ mx2@ my2@ mc@ lcdColor lcdLine cx1@ cy1@ cx2@ cy2@ mcc@ lcdColor lcdLine my2@ ms@ - dup my2! dup cy2! 0 <   mstop@ | esc? dup [ -1 mstop! ] | }

CT_0>

CT_0>: mor 255 0 0 lc1 mc!

CT_0>: mog 0 255 0 lc1 mc!

CT_0>: mob 0 0 255 lc1 mc!

CT_0>: mow 255 255 255 lc1 mc!

CT_0>

CT_0>: mo_1 1 ms! 0 0 0 lc1 mcc! 0 mstop!

CT_0>: mo_2 mo1 mo2 mo3 mo4 ms@ 1 + dup 10 > [ drop 1 ] ms!

CT_0>: mo mo_1 { mow mo_2 mor mo_2 mog mo_2 mob mo_2 mstop@ }

CT_0>

CT_0>: to { lcdt esc? }

CT_0>

CT_0>: lcdStart lcdInit 3000 7 setPwm lcdClear lcdt 1000 delay lcdClear lcdHelo 3000 delay lcdClear lcdGreet

CT_0>

CT_0>

CT_0>compileExtensionDictionary

static const uint16_t compiledExtensionDictionaryData[] = {
0xFFFF , 0x6461 , 0x3163 , 0x0000 , 0xC01E , 0x6008 , 0xC00C , 0xC009 , 0x0001 , 0xC027 , 0xC001 , 0x0000 , 0x6461 , 0x3263 , 0x0000 , 0xC009 , // 0x0000  ..adc1.....`......'.....adc2....
0x0000 , 0xE004 , 0xE004 , 0xE004 , 0xE004 , 0xE004 , 0xE004 , 0xE004 , 0xE004 , 0xC022 , 0xC003 , 0xC001 , 0x000B , 0x6461 , 0x0063 , 0xC076 , // 0x0010  ..................".......adc.v.
0xE00F , 0xC009 , 0x03E8 , 0xC047 , 0x8752 , 0xC077 , 0xE020 , 0xC001 , 0x001C , 0x7473 , 0x0000 , 0xC009 , 0x0000 , 0xC009 , 0x0000 , 0xC076 , // 0x0020  ......G.R.w. .....st..........v.
0xC01E , 0xC009 , 0x0000 , 0x600A , 0xC040 , 0xC009 , 0x0001 , 0x600A , 0xC009 , 0x0064 , 0xC027 , 0xC01E , 0xC009 , 0x0FA0 , 0xC032 , 0xC073 , // 0x0030  .......`@......`..d.'.......2.s.
0xE045 , 0xC022 , 0xC009 , 0x0000 , 0xC075 , 0xC01F , 0xC009 , 0x00C8 , 0xC027 , 0xC01E , 0xC009 , 0x0FA0 , 0xC032 , 0xC073 , 0xE053 , 0xC022 , // 0x0040  E.".....u.......'.......2.s.S.".
0xC009 , 0x0000 , 0xC075 , 0xC01F , 0x8752 , 0xC077 , 0xE030 , 0xC001 , 0x0028 , 0x6433 , 0x7075 , 0x0000 , 0xC035 , 0xC035 , 0xC035 , 0xC009 , // 0x0050  ....u...R.w.0...(.3dup..5.5.5...
0x0000 , 0xC03C , 0xC009 , 0x0001 , 0xC03C , 0xC009 , 0x0002 , 0xC03C , 0xC036 , 0xC036 , 0xC036 , 0xC001 , 0x0058 , 0x636C , 0x0031 , 0xC009 , // 0x0060  ..<.....<.....<.6.6.6...X.lc1...
0x0003 , 0xC06D , 0xC023 , 0xC009 , 0x00F8 , 0xC02C , 0xC009 , 0x0008 , 0xC06B , 0xC023 , 0xC009 , 0x00FC , 0xC02C , 0xC009 , 0x0003 , 0xC06B , // 0x0070  ..m.#.....,.....k.#.....,.....k.
0xC02D , 0xC02D , 0xC001 , 0x006C , 0x6772 , 0x0062 , 0xE06F , 0x600D , 0xC001 , 0x0083 , 0x6374 , 0x0031 , 0xE05C , 0x600C , 0xC009 , 0x0001 , // 0x0080  -.-...l.rgb.o..`....tc1.\..`....
0xC027 , 0xC01E , 0xC009 , 0x007E , 0xC033 , 0xC073 , 0xE09B , 0xC022 , 0xC009 , 0x0021 , 0xC075 , 0xC01F , 0xC009 , 0x0001 , 0xC027 , 0xC01F , // 0x0090  '.....~.3.s..."...!.u.......'...
0xC001 , 0x0089 , 0x6374 , 0x0035 , 0xE08C , 0xE08C , 0xE08C , 0xE08C , 0xE08C , 0xC001 , 0x00A1 , 0x6374 , 0x3032 , 0x0000 , 0xE0A4 , 0xE0A4 , // 0x00A0  ....tc5...............tc20......
0xE0A4 , 0xE0A4 , 0xC001 , 0x00AA , 0x6C74 , 0x6463 , 0x694C , 0x656E , 0x0000 , 0xC009 , 0x0000 , 0xC023 , 0xE0AE , 0xC020 , 0xC020 , 0xC001 , // 0x00B0  ........tlcdLine......#... . ...
0x00B3 , 0x636C , 0x3064 , 0x0000 , 0xC009 , 0x00FF , 0xC009 , 0x00FF , 0xC009 , 0x00FF , 0xE086 , 0xC009 , 0x0000 , 0xE0B9 , 0xC001 , 0x00C0 , // 0x00C0  ..lcd0..........................
0x636C , 0x3164 , 0x0000 , 0xC009 , 0x00FF , 0xC009 , 0x0000 , 0xC009 , 0x0000 , 0xE086 , 0xC009 , 0x0001 , 0xE0B9 , 0xC001 , 0x00CF , 0x636C , // 0x00D0  lcd1..........................lc
0x3264 , 0x0000 , 0xC009 , 0x0000 , 0xC009 , 0x00FF , 0xC009 , 0x0000 , 0xE086 , 0xC009 , 0x0002 , 0xE0B9 , 0xC001 , 0x00DE , 0x636C , 0x3364 , // 0x00E0  d2..........................lcd3
0x0000 , 0xC009 , 0x0000 , 0xC009 , 0x0000 , 0xC009 , 0x00FF , 0xE086 , 0xC009 , 0x0003 , 0xE0B9 , 0xC001 , 0x00ED , 0x636C , 0x3464 , 0x0000 , // 0x00F0  ..........................lcd4..
0xC009 , 0x00FF , 0xC009 , 0x00FF , 0xC009 , 0x0000 , 0xE086 , 0xC009 , 0x0004 , 0xE0B9 , 0xC001 , 0x00FC , 0x636C , 0x3564 , 0x0000 , 0xC009 , // 0x0100  ........................lcd5....
0x0000 , 0xC009 , 0x00FF , 0xC009 , 0x00FF , 0xE086 , 0xC009 , 0x0005 , 0xE0B9 , 0xC001 , 0x010B , 0x636C , 0x4364 , 0x656C , 0x7261 , 0x0000 , // 0x0110  ......................lcdClear..
0xC009 , 0x0000 , 0x600D , 0xC009 , 0x0000 , 0xC009 , 0x0000 , 0xC009 , 0x00A0 , 0xC009 , 0x0050 , 0x600E , 0xC001 , 0x011A , 0x636C , 0x7464 , // 0x0120  .....`..............P..`....lcdt
0x0000 , 0xE120 , 0xC009 , 0x0021 , 0xE0C4 , 0xE0D3 , 0xE0E2 , 0xE0F1 , 0xE100 , 0xE10F , 0xC022 , 0xC001 , 0x012D , 0x636C , 0x4864 , 0x6C65 , // 0x0130  .. ...!............."...-.lcdHel
0x006F , 0xE120 , 0xC009 , 0x00FF , 0xC009 , 0x00FF , 0xC009 , 0x00FF , 0xE086 , 0xC009 , 0x0002 , 0xC009 , 0x0008 , 0xC016 , 0x6568 , 0x6F6C , // 0x0140  o. .........................helo
0x0000 , 0x6011 , 0xC001 , 0x013C , 0x636C , 0x4764 , 0x6572 , 0x7465 , 0x0000 , 0xE120 , 0xC009 , 0x0000 , 0xC009 , 0x00FF , 0xC009 , 0x0000 , // 0x0150  ...`..<.lcdGreet.. .............
0xE086 , 0xC009 , 0x0002 , 0xC009 , 0x0001 , 0xC016 , 0x6F47 , 0x646F , 0x4D20 , 0x726F , 0x696E , 0x676E , 0x4420 , 0x7661 , 0x3E65 , 0x0000 , // 0x0160  ............Good Morning Dave>..
0x6011 , 0xC001 , 0x0153 , 0x3178 , 0x0021 , 0xC009 , 0x0000 , 0xC063 , 0xC001 , 0x0172 , 0x3278 , 0x0021 , 0xC009 , 0x0001 , 0xC063 , 0xC001 , // 0x0170  .`..S.x1!.....c...r.x2!.....c...
0x0179 , 0x3179 , 0x0021 , 0xC009 , 0x0002 , 0xC063 , 0xC001 , 0x0180 , 0x3279 , 0x0021 , 0xC009 , 0x0003 , 0xC063 , 0xC001 , 0x0187 , 0x6478 , // 0x0180  y.y1!.....c.....y2!.....c.....xd
0x0021 , 0xC009 , 0x0004 , 0xC063 , 0xC001 , 0x018E , 0x6479 , 0x0021 , 0xC009 , 0x0005 , 0xC063 , 0xC001 , 0x0195 , 0x216D , 0x0000 , 0xC009 , // 0x0190  !.....c.....yd!.....c.....m!....
0x0006 , 0xC063 , 0xC001 , 0x019C , 0x786D , 0x2131 , 0x0000 , 0xC009 , 0x0007 , 0xC063 , 0xC001 , 0x01A3 , 0x786D , 0x2132 , 0x0000 , 0xC009 , // 0x01A0  ..c.....mx1!......c.....mx2!....
0x0008 , 0xC063 , 0xC001 , 0x01AB , 0x796D , 0x2131 , 0x0000 , 0xC009 , 0x0009 , 0xC063 , 0xC001 , 0x01B3 , 0x796D , 0x2132 , 0x0000 , 0xC009 , // 0x01B0  ..c.....my1!......c.....my2!....
0x000A , 0xC063 , 0xC001 , 0x01BB , 0x7863 , 0x2131 , 0x0000 , 0xC009 , 0x000B , 0xC063 , 0xC001 , 0x01C3 , 0x7863 , 0x2132 , 0x0000 , 0xC009 , // 0x01C0  ..c.....cx1!......c.....cx2!....
0x000C , 0xC063 , 0xC001 , 0x01CB , 0x7963 , 0x2131 , 0x0000 , 0xC009 , 0x000D , 0xC063 , 0xC001 , 0x01D3 , 0x7963 , 0x2132 , 0x0000 , 0xC009 , // 0x01D0  ..c.....cy1!......c.....cy2!....
0x000E , 0xC063 , 0xC001 , 0x01DB , 0x736D , 0x0021 , 0xC009 , 0x000F , 0xC063 , 0xC001 , 0x01E3 , 0x636D , 0x0021 , 0xC009 , 0x0010 , 0xC063 , // 0x01E0  ..c.....ms!.....c.....mc!.....c.
0xC001 , 0x01EA , 0x636D , 0x2163 , 0x0000 , 0xC009 , 0x0011 , 0xC063 , 0xC001 , 0x01F1 , 0x736D , 0x6F74 , 0x2170 , 0x0000 , 0xC009 , 0x0012 , // 0x01F0  ....mcc!......c.....mstop!......
0xC063 , 0xC001 , 0x01F9 , 0x3178 , 0x0040 , 0xC009 , 0x0000 , 0xC064 , 0xC001 , 0x0202 , 0x3278 , 0x0040 , 0xC009 , 0x0001 , 0xC064 , 0xC001 , // 0x0200  c.....x1@.....d.....x2@.....d...
0x0209 , 0x3179 , 0x0040 , 0xC009 , 0x0002 , 0xC064 , 0xC001 , 0x0210 , 0x3279 , 0x0040 , 0xC009 , 0x0003 , 0xC064 , 0xC001 , 0x0217 , 0x6478 , // 0x0210  ..y1@.....d.....y2@.....d.....xd
0x0040 , 0xC009 , 0x0004 , 0xC064 , 0xC001 , 0x021E , 0x6479 , 0x0040 , 0xC009 , 0x0005 , 0xC064 , 0xC001 , 0x0225 , 0x406D , 0x0000 , 0xC009 , // 0x0220  @.....d.....yd@.....d...%.m@....
0x0006 , 0xC064 , 0xC001 , 0x022C , 0x786D , 0x4031 , 0x0000 , 0xC009 , 0x0007 , 0xC064 , 0xC001 , 0x0233 , 0x786D , 0x4032 , 0x0000 , 0xC009 , // 0x0230  ..d...,.mx1@......d...3.mx2@....
0x0008 , 0xC064 , 0xC001 , 0x023B , 0x796D , 0x4031 , 0x0000 , 0xC009 , 0x0009 , 0xC064 , 0xC001 , 0x0243 , 0x796D , 0x4032 , 0x0000 , 0xC009 , // 0x0240  ..d...;.my1@......d...C.my2@....
0x000A , 0xC064 , 0xC001 , 0x024B , 0x7863 , 0x4031 , 0x0000 , 0xC009 , 0x000B , 0xC064 , 0xC001 , 0x0253 , 0x7863 , 0x4032 , 0x0000 , 0xC009 , // 0x0250  ..d...K.cx1@......d...S.cx2@....
0x000C , 0xC064 , 0xC001 , 0x025B , 0x7963 , 0x4031 , 0x0000 , 0xC009 , 0x000D , 0xC064 , 0xC001 , 0x0263 , 0x7963 , 0x4032 , 0x0000 , 0xC009 , // 0x0260  ..d...[.cy1@......d...c.cy2@....
0x000E , 0xC064 , 0xC001 , 0x026B , 0x736D , 0x0040 , 0xC009 , 0x000F , 0xC064 , 0xC001 , 0x0273 , 0x636D , 0x0040 , 0xC009 , 0x0010 , 0xC064 , // 0x0270  ..d...k.ms@.....d...s.mc@.....d.
0xC001 , 0x027A , 0x636D , 0x4063 , 0x0000 , 0xC009 , 0x0011 , 0xC064 , 0xC001 , 0x0281 , 0x736D , 0x6F74 , 0x4070 , 0x0000 , 0xC009 , 0x0012 , // 0x0280  ..z.mcc@......d.....mstop@......
0xC064 , 0xC001 , 0x0289 , 0x6F6D , 0x5F31 , 0x0031 , 0xC009 , 0x003C , 0xC009 , 0x0028 , 0xC009 , 0x0000 , 0xC009 , 0x0000 , 0xE1BF , 0xE1AF , // 0x0290  d.....mo1_1...<...(.............
0xE1B7 , 0xE1A7 , 0xC009 , 0x0064 , 0xC009 , 0x0028 , 0xC009 , 0x0000 , 0xC009 , 0x0000 , 0xE1DF , 0xE1CF , 0xE1D7 , 0xE1C7 , 0xC001 , 0x0292 , // 0x02A0  ......d...(.....................
0x6F6D , 0x5F32 , 0x0031 , 0xC009 , 0x003C , 0xC009 , 0x0028 , 0xC009 , 0x009F , 0xC009 , 0x0000 , 0xE1BF , 0xE1AF , 0xE1B7 , 0xE1A7 , 0xC009 , // 0x02B0  mo2_1...<...(...................
0x0064 , 0xC009 , 0x0028 , 0xC009 , 0x009F , 0xC009 , 0x0000 , 0xE1DF , 0xE1CF , 0xE1D7 , 0xE1C7 , 0xC001 , 0x02AF , 0x6F6D , 0x5F33 , 0x0031 , // 0x02C0  d...(.....................mo3_1.
0xC009 , 0x003C , 0xC009 , 0x0028 , 0xC009 , 0x009F , 0xC009 , 0x004F , 0xE1BF , 0xE1AF , 0xE1B7 , 0xE1A7 , 0xC009 , 0x0064 , 0xC009 , 0x0028 , // 0x02D0  ..<...(.......O...........d...(.
0xC009 , 0x009F , 0xC009 , 0x004F , 0xE1DF , 0xE1CF , 0xE1D7 , 0xE1C7 , 0xC001 , 0x02CC , 0x6F6D , 0x5F34 , 0x0031 , 0xC009 , 0x003C , 0xC009 , // 0x02E0  ......O.............mo4_1...<...
0x0028 , 0xC009 , 0x0000 , 0xC009 , 0x004F , 0xE1BF , 0xE1AF , 0xE1B7 , 0xE1A7 , 0xC009 , 0x0064 , 0xC009 , 0x0028 , 0xC009 , 0x0000 , 0xC009 , // 0x02F0  (.......O...........d...(.......
0x004F , 0xE1DF , 0xE1CF , 0xE1D7 , 0xE1C7 , 0xC001 , 0x02E9 , 0x6F6D , 0x0031 , 0xE296 , 0xC076 , 0xE237 , 0xE247 , 0xE23F , 0xE24F , 0xE27D , // 0x0300  O.............mo1...v.7.G.?.O.}.
0x600D , 0x600F , 0xE257 , 0xE267 , 0xE25F , 0xE26F , 0xE285 , 0x600D , 0x600F , 0xE23F , 0xE276 , 0xC027 , 0xC01E , 0xE1AF , 0xC01E , 0xE1CF , // 0x0310  .`.`W.g._.o....`.`?.v.'.........
0xC009 , 0x009F , 0xC032 , 0xE28E , 0xC02D , 0x8752 , 0xC01E , 0xC073 , 0xE32D , 0xC00B , 0xFFFF , 0xE1FE , 0xC075 , 0xC02D , 0xC077 , 0xE30B , // 0x0320  ....2...-.R...s.-.......u.-.w...
0xC001 , 0x0306 , 0x6F6D , 0x0032 , 0xE2B3 , 0xC076 , 0xE237 , 0xE247 , 0xE23F , 0xE24F , 0xE27D , 0x600D , 0x600F , 0xE257 , 0xE267 , 0xE25F , // 0x0330  ....mo2...v.7.G.?.O.}..`.`W.g._.
0xE26F , 0xE285 , 0x600D , 0x600F , 0xE24F , 0xE276 , 0xC027 , 0xC01E , 0xE1BF , 0xC01E , 0xE1DF , 0xC009 , 0x004F , 0xC032 , 0xE28E , 0xC02D , // 0x0340  o....`.`O.v.'...........O.2...-.
0x8752 , 0xC01E , 0xC073 , 0xE358 , 0xC00B , 0xFFFF , 0xE1FE , 0xC075 , 0xC02D , 0xC077 , 0xE336 , 0xC001 , 0x0331 , 0x6F6D , 0x0033 , 0xE2D0 , // 0x0350  R...s.X.......u.-.w.6...1.mo3...
0xC076 , 0xE237 , 0xE247 , 0xE23F , 0xE24F , 0xE27D , 0x600D , 0x600F , 0xE257 , 0xE267 , 0xE25F , 0xE26F , 0xE285 , 0x600D , 0x600F , 0xE23F , // 0x0360  v.7.G.?.O.}..`.`W.g._.o....`.`?.
0xE276 , 0xC028 , 0xC01E , 0xE1AF , 0xC01E , 0xE1CF , 0xC009 , 0x0000 , 0xC02F , 0xE28E , 0xC02D , 0x8752 , 0xC01E , 0xC073 , 0xE383 , 0xC00B , // 0x0370  v.(............./...-.R...s.....
0xFFFF , 0xE1FE , 0xC075 , 0xC02D , 0xC077 , 0xE361 , 0xC001 , 0x035C , 0x6F6D , 0x0034 , 0xE2ED , 0xC076 , 0xE237 , 0xE247 , 0xE23F , 0xE24F , // 0x0380  ....u.-.w.a...\.mo4...v.7.G.?.O.
0xE27D , 0x600D , 0x600F , 0xE257 , 0xE267 , 0xE25F , 0xE26F , 0xE285 , 0x600D , 0x600F , 0xE24F , 0xE276 , 0xC028 , 0xC01E , 0xE1BF , 0xC01E , // 0x0390  }..`.`W.g._.o....`.`O.v.(.......
0xE1DF , 0xC009 , 0x0000 , 0xC02F , 0xE28E , 0xC02D , 0x8752 , 0xC01E , 0xC073 , 0xE3AE , 0xC00B , 0xFFFF , 0xE1FE , 0xC075 , 0xC02D , 0xC077 , // 0x03A0  ....../...-.R...s.........u.-.w.
0xE38C , 0xC001 , 0x0387 , 0x6F6D , 0x0072 , 0xC009 , 0x00FF , 0xC009 , 0x0000 , 0xC009 , 0x0000 , 0xE06F , 0xE1ED , 0xC001 , 0x03B2 , 0x6F6D , // 0x03B0  ......mor.............o.......mo
0x0067 , 0xC009 , 0x0000 , 0xC009 , 0x00FF , 0xC009 , 0x0000 , 0xE06F , 0xE1ED , 0xC001 , 0x03BE , 0x6F6D , 0x0062 , 0xC009 , 0x0000 , 0xC009 , // 0x03C0  g.............o.......mob.......
0x0000 , 0xC009 , 0x00FF , 0xE06F , 0xE1ED , 0xC001 , 0x03CA , 0x6F6D , 0x0077 , 0xC009 , 0x00FF , 0xC009 , 0x00FF , 0xC009 , 0x00FF , 0xE06F , // 0x03D0  ......o.......mow.............o.
0xE1ED , 0xC001 , 0x03D6 , 0x6F6D , 0x315F , 0x0000 , 0xC009 , 0x0001 , 0xE1E6 , 0xC009 , 0x0000 , 0xC009 , 0x0000 , 0xC009 , 0x0000 , 0xE06F , // 0x03E0  ......mo_1....................o.
0xE1F5 , 0xC009 , 0x0000 , 0xE1FE , 0xC001 , 0x03E2 , 0x6F6D , 0x325F , 0x0000 , 0xE309 , 0xE334 , 0xE35F , 0xE38A , 0xE276 , 0xC009 , 0x0001 , // 0x03F0  ............mo_2....4._...v.....
0xC027 , 0xC01E , 0xC009 , 0x000A , 0xC032 , 0xC073 , 0xE40B , 0xC022 , 0xC009 , 0x0001 , 0xC075 , 0xE1E6 , 0xC001 , 0x03F5 , 0x6F6D , 0x0000 , // 0x0400  '.......2.s...".....u.......mo..
0xE3E6 , 0xC076 , 0xE3D9 , 0xE3F9 , 0xE3B5 , 0xE3F9 , 0xE3C1 , 0xE3F9 , 0xE3CD , 0xE3F9 , 0xE28E , 0xC077 , 0xE412 , 0xC001 , 0x040D , 0x6F74 , // 0x0410  ..v...................w.......to
0x0000 , 0xC076 , 0xE131 , 0x8752 , 0xC077 , 0xE422 , 0xC001 , 0x041E , 0x636C , 0x5364 , 0x6174 , 0x7472 , 0x0000 , 0x600B , 0xC009 , 0x0BB8 , // 0x0420  ..v.1.R.w.".....lcdStart...`....
0xC009 , 0x0007 , 0x6009 , 0xE120 , 0xE131 , 0xC009 , 0x03E8 , 0xC047 , 0xE120 , 0xE141 , 0xC009 , 0x0BB8 , 0xC047 , 0xE120 , 0xE159 , 0xC001 , // 0x0430  .....` .1.....G. .A.....G. .Y...


};
static CompiledDictionary compiledExtensionDictionary( compiledExtensionDictionaryData, 0x0427 , 0x0440 , YRSHELL_DICTIONARY_EXTENSION_COMPILED);

CT_0>

CT_0>


*/
use super::{ComDictionary, DictionaryMask};

static COMPILED_SHELL_DICTIONARY_DATA: [u16; 1088] = [
    0xFFFF , 0x6461 , 0x3163 , 0x0000 , 0xC01E , 0x6008 , 0xC00C , 0xC009 , 0x0001 , 0xC027 , 0xC001 , 0x0000 , 0x6461 , 0x3263 , 0x0000 , 0xC009 , // 0x0000  ..adc1.....`......'.....adc2....
    0x0000 , 0xE004 , 0xE004 , 0xE004 , 0xE004 , 0xE004 , 0xE004 , 0xE004 , 0xE004 , 0xC022 , 0xC003 , 0xC001 , 0x000B , 0x6461 , 0x0063 , 0xC076 , // 0x0010  ..................".......adc.v.
    0xE00F , 0xC009 , 0x03E8 , 0xC047 , 0x8752 , 0xC077 , 0xE020 , 0xC001 , 0x001C , 0x7473 , 0x0000 , 0xC009 , 0x0000 , 0xC009 , 0x0000 , 0xC076 , // 0x0020  ......G.R.w. .....st..........v.
    0xC01E , 0xC009 , 0x0000 , 0x600A , 0xC040 , 0xC009 , 0x0001 , 0x600A , 0xC009 , 0x0064 , 0xC027 , 0xC01E , 0xC009 , 0x0FA0 , 0xC032 , 0xC073 , // 0x0030  .......`@......`..d.'.......2.s.
    0xE045 , 0xC022 , 0xC009 , 0x0000 , 0xC075 , 0xC01F , 0xC009 , 0x00C8 , 0xC027 , 0xC01E , 0xC009 , 0x0FA0 , 0xC032 , 0xC073 , 0xE053 , 0xC022 , // 0x0040  E.".....u.......'.......2.s.S.".
    0xC009 , 0x0000 , 0xC075 , 0xC01F , 0x8752 , 0xC077 , 0xE030 , 0xC001 , 0x0028 , 0x6433 , 0x7075 , 0x0000 , 0xC035 , 0xC035 , 0xC035 , 0xC009 , // 0x0050  ....u...R.w.0...(.3dup..5.5.5...
    0x0000 , 0xC03C , 0xC009 , 0x0001 , 0xC03C , 0xC009 , 0x0002 , 0xC03C , 0xC036 , 0xC036 , 0xC036 , 0xC001 , 0x0058 , 0x636C , 0x0031 , 0xC009 , // 0x0060  ..<.....<.....<.6.6.6...X.lc1...
    0x0003 , 0xC06D , 0xC023 , 0xC009 , 0x00F8 , 0xC02C , 0xC009 , 0x0008 , 0xC06B , 0xC023 , 0xC009 , 0x00FC , 0xC02C , 0xC009 , 0x0003 , 0xC06B , // 0x0070  ..m.#.....,.....k.#.....,.....k.
    0xC02D , 0xC02D , 0xC001 , 0x006C , 0x6772 , 0x0062 , 0xE06F , 0x600D , 0xC001 , 0x0083 , 0x6374 , 0x0031 , 0xE05C , 0x600C , 0xC009 , 0x0001 , // 0x0080  -.-...l.rgb.o..`....tc1.\..`....
    0xC027 , 0xC01E , 0xC009 , 0x007E , 0xC033 , 0xC073 , 0xE09B , 0xC022 , 0xC009 , 0x0021 , 0xC075 , 0xC01F , 0xC009 , 0x0001 , 0xC027 , 0xC01F , // 0x0090  '.....~.3.s..."...!.u.......'...
    0xC001 , 0x0089 , 0x6374 , 0x0035 , 0xE08C , 0xE08C , 0xE08C , 0xE08C , 0xE08C , 0xC001 , 0x00A1 , 0x6374 , 0x3032 , 0x0000 , 0xE0A4 , 0xE0A4 , // 0x00A0  ....tc5...............tc20......
    0xE0A4 , 0xE0A4 , 0xC001 , 0x00AA , 0x6C74 , 0x6463 , 0x694C , 0x656E , 0x0000 , 0xC009 , 0x0000 , 0xC023 , 0xE0AE , 0xC020 , 0xC020 , 0xC001 , // 0x00B0  ........tlcdLine......#... . ...
    0x00B3 , 0x636C , 0x3064 , 0x0000 , 0xC009 , 0x00FF , 0xC009 , 0x00FF , 0xC009 , 0x00FF , 0xE086 , 0xC009 , 0x0000 , 0xE0B9 , 0xC001 , 0x00C0 , // 0x00C0  ..lcd0..........................
    0x636C , 0x3164 , 0x0000 , 0xC009 , 0x00FF , 0xC009 , 0x0000 , 0xC009 , 0x0000 , 0xE086 , 0xC009 , 0x0001 , 0xE0B9 , 0xC001 , 0x00CF , 0x636C , // 0x00D0  lcd1..........................lc
    0x3264 , 0x0000 , 0xC009 , 0x0000 , 0xC009 , 0x00FF , 0xC009 , 0x0000 , 0xE086 , 0xC009 , 0x0002 , 0xE0B9 , 0xC001 , 0x00DE , 0x636C , 0x3364 , // 0x00E0  d2..........................lcd3
    0x0000 , 0xC009 , 0x0000 , 0xC009 , 0x0000 , 0xC009 , 0x00FF , 0xE086 , 0xC009 , 0x0003 , 0xE0B9 , 0xC001 , 0x00ED , 0x636C , 0x3464 , 0x0000 , // 0x00F0  ..........................lcd4..
    0xC009 , 0x00FF , 0xC009 , 0x00FF , 0xC009 , 0x0000 , 0xE086 , 0xC009 , 0x0004 , 0xE0B9 , 0xC001 , 0x00FC , 0x636C , 0x3564 , 0x0000 , 0xC009 , // 0x0100  ........................lcd5....
    0x0000 , 0xC009 , 0x00FF , 0xC009 , 0x00FF , 0xE086 , 0xC009 , 0x0005 , 0xE0B9 , 0xC001 , 0x010B , 0x636C , 0x4364 , 0x656C , 0x7261 , 0x0000 , // 0x0110  ......................lcdClear..
    0xC009 , 0x0000 , 0x600D , 0xC009 , 0x0000 , 0xC009 , 0x0000 , 0xC009 , 0x00A0 , 0xC009 , 0x0050 , 0x600E , 0xC001 , 0x011A , 0x636C , 0x7464 , // 0x0120  .....`..............P..`....lcdt
    0x0000 , 0xE120 , 0xC009 , 0x0021 , 0xE0C4 , 0xE0D3 , 0xE0E2 , 0xE0F1 , 0xE100 , 0xE10F , 0xC022 , 0xC001 , 0x012D , 0x636C , 0x4864 , 0x6C65 , // 0x0130  .. ...!............."...-.lcdHel
    0x006F , 0xE120 , 0xC009 , 0x00FF , 0xC009 , 0x00FF , 0xC009 , 0x00FF , 0xE086 , 0xC009 , 0x0002 , 0xC009 , 0x0008 , 0xC016 , 0x6568 , 0x6F6C , // 0x0140  o. .........................helo
    0x0000 , 0x6011 , 0xC001 , 0x013C , 0x636C , 0x4764 , 0x6572 , 0x7465 , 0x0000 , 0xE120 , 0xC009 , 0x0000 , 0xC009 , 0x00FF , 0xC009 , 0x0000 , // 0x0150  ...`..<.lcdGreet.. .............
    0xE086 , 0xC009 , 0x0002 , 0xC009 , 0x0001 , 0xC016 , 0x6F47 , 0x646F , 0x4D20 , 0x726F , 0x696E , 0x676E , 0x4420 , 0x7661 , 0x3E65 , 0x0000 , // 0x0160  ............Good Morning Dave>..
    0x6011 , 0xC001 , 0x0153 , 0x3178 , 0x0021 , 0xC009 , 0x0000 , 0xC063 , 0xC001 , 0x0172 , 0x3278 , 0x0021 , 0xC009 , 0x0001 , 0xC063 , 0xC001 , // 0x0170  .`..S.x1!.....c...r.x2!.....c...
    0x0179 , 0x3179 , 0x0021 , 0xC009 , 0x0002 , 0xC063 , 0xC001 , 0x0180 , 0x3279 , 0x0021 , 0xC009 , 0x0003 , 0xC063 , 0xC001 , 0x0187 , 0x6478 , // 0x0180  y.y1!.....c.....y2!.....c.....xd
    0x0021 , 0xC009 , 0x0004 , 0xC063 , 0xC001 , 0x018E , 0x6479 , 0x0021 , 0xC009 , 0x0005 , 0xC063 , 0xC001 , 0x0195 , 0x216D , 0x0000 , 0xC009 , // 0x0190  !.....c.....yd!.....c.....m!....
    0x0006 , 0xC063 , 0xC001 , 0x019C , 0x786D , 0x2131 , 0x0000 , 0xC009 , 0x0007 , 0xC063 , 0xC001 , 0x01A3 , 0x786D , 0x2132 , 0x0000 , 0xC009 , // 0x01A0  ..c.....mx1!......c.....mx2!....
    0x0008 , 0xC063 , 0xC001 , 0x01AB , 0x796D , 0x2131 , 0x0000 , 0xC009 , 0x0009 , 0xC063 , 0xC001 , 0x01B3 , 0x796D , 0x2132 , 0x0000 , 0xC009 , // 0x01B0  ..c.....my1!......c.....my2!....
    0x000A , 0xC063 , 0xC001 , 0x01BB , 0x7863 , 0x2131 , 0x0000 , 0xC009 , 0x000B , 0xC063 , 0xC001 , 0x01C3 , 0x7863 , 0x2132 , 0x0000 , 0xC009 , // 0x01C0  ..c.....cx1!......c.....cx2!....
    0x000C , 0xC063 , 0xC001 , 0x01CB , 0x7963 , 0x2131 , 0x0000 , 0xC009 , 0x000D , 0xC063 , 0xC001 , 0x01D3 , 0x7963 , 0x2132 , 0x0000 , 0xC009 , // 0x01D0  ..c.....cy1!......c.....cy2!....
    0x000E , 0xC063 , 0xC001 , 0x01DB , 0x736D , 0x0021 , 0xC009 , 0x000F , 0xC063 , 0xC001 , 0x01E3 , 0x636D , 0x0021 , 0xC009 , 0x0010 , 0xC063 , // 0x01E0  ..c.....ms!.....c.....mc!.....c.
    0xC001 , 0x01EA , 0x636D , 0x2163 , 0x0000 , 0xC009 , 0x0011 , 0xC063 , 0xC001 , 0x01F1 , 0x736D , 0x6F74 , 0x2170 , 0x0000 , 0xC009 , 0x0012 , // 0x01F0  ....mcc!......c.....mstop!......
    0xC063 , 0xC001 , 0x01F9 , 0x3178 , 0x0040 , 0xC009 , 0x0000 , 0xC064 , 0xC001 , 0x0202 , 0x3278 , 0x0040 , 0xC009 , 0x0001 , 0xC064 , 0xC001 , // 0x0200  c.....x1@.....d.....x2@.....d...
    0x0209 , 0x3179 , 0x0040 , 0xC009 , 0x0002 , 0xC064 , 0xC001 , 0x0210 , 0x3279 , 0x0040 , 0xC009 , 0x0003 , 0xC064 , 0xC001 , 0x0217 , 0x6478 , // 0x0210  ..y1@.....d.....y2@.....d.....xd
    0x0040 , 0xC009 , 0x0004 , 0xC064 , 0xC001 , 0x021E , 0x6479 , 0x0040 , 0xC009 , 0x0005 , 0xC064 , 0xC001 , 0x0225 , 0x406D , 0x0000 , 0xC009 , // 0x0220  @.....d.....yd@.....d...%.m@....
    0x0006 , 0xC064 , 0xC001 , 0x022C , 0x786D , 0x4031 , 0x0000 , 0xC009 , 0x0007 , 0xC064 , 0xC001 , 0x0233 , 0x786D , 0x4032 , 0x0000 , 0xC009 , // 0x0230  ..d...,.mx1@......d...3.mx2@....
    0x0008 , 0xC064 , 0xC001 , 0x023B , 0x796D , 0x4031 , 0x0000 , 0xC009 , 0x0009 , 0xC064 , 0xC001 , 0x0243 , 0x796D , 0x4032 , 0x0000 , 0xC009 , // 0x0240  ..d...;.my1@......d...C.my2@....
    0x000A , 0xC064 , 0xC001 , 0x024B , 0x7863 , 0x4031 , 0x0000 , 0xC009 , 0x000B , 0xC064 , 0xC001 , 0x0253 , 0x7863 , 0x4032 , 0x0000 , 0xC009 , // 0x0250  ..d...K.cx1@......d...S.cx2@....
    0x000C , 0xC064 , 0xC001 , 0x025B , 0x7963 , 0x4031 , 0x0000 , 0xC009 , 0x000D , 0xC064 , 0xC001 , 0x0263 , 0x7963 , 0x4032 , 0x0000 , 0xC009 , // 0x0260  ..d...[.cy1@......d...c.cy2@....
    0x000E , 0xC064 , 0xC001 , 0x026B , 0x736D , 0x0040 , 0xC009 , 0x000F , 0xC064 , 0xC001 , 0x0273 , 0x636D , 0x0040 , 0xC009 , 0x0010 , 0xC064 , // 0x0270  ..d...k.ms@.....d...s.mc@.....d.
    0xC001 , 0x027A , 0x636D , 0x4063 , 0x0000 , 0xC009 , 0x0011 , 0xC064 , 0xC001 , 0x0281 , 0x736D , 0x6F74 , 0x4070 , 0x0000 , 0xC009 , 0x0012 , // 0x0280  ..z.mcc@......d.....mstop@......
    0xC064 , 0xC001 , 0x0289 , 0x6F6D , 0x5F31 , 0x0031 , 0xC009 , 0x003C , 0xC009 , 0x0028 , 0xC009 , 0x0000 , 0xC009 , 0x0000 , 0xE1BF , 0xE1AF , // 0x0290  d.....mo1_1...<...(.............
    0xE1B7 , 0xE1A7 , 0xC009 , 0x0064 , 0xC009 , 0x0028 , 0xC009 , 0x0000 , 0xC009 , 0x0000 , 0xE1DF , 0xE1CF , 0xE1D7 , 0xE1C7 , 0xC001 , 0x0292 , // 0x02A0  ......d...(.....................
    0x6F6D , 0x5F32 , 0x0031 , 0xC009 , 0x003C , 0xC009 , 0x0028 , 0xC009 , 0x009F , 0xC009 , 0x0000 , 0xE1BF , 0xE1AF , 0xE1B7 , 0xE1A7 , 0xC009 , // 0x02B0  mo2_1...<...(...................
    0x0064 , 0xC009 , 0x0028 , 0xC009 , 0x009F , 0xC009 , 0x0000 , 0xE1DF , 0xE1CF , 0xE1D7 , 0xE1C7 , 0xC001 , 0x02AF , 0x6F6D , 0x5F33 , 0x0031 , // 0x02C0  d...(.....................mo3_1.
    0xC009 , 0x003C , 0xC009 , 0x0028 , 0xC009 , 0x009F , 0xC009 , 0x004F , 0xE1BF , 0xE1AF , 0xE1B7 , 0xE1A7 , 0xC009 , 0x0064 , 0xC009 , 0x0028 , // 0x02D0  ..<...(.......O...........d...(.
    0xC009 , 0x009F , 0xC009 , 0x004F , 0xE1DF , 0xE1CF , 0xE1D7 , 0xE1C7 , 0xC001 , 0x02CC , 0x6F6D , 0x5F34 , 0x0031 , 0xC009 , 0x003C , 0xC009 , // 0x02E0  ......O.............mo4_1...<...
    0x0028 , 0xC009 , 0x0000 , 0xC009 , 0x004F , 0xE1BF , 0xE1AF , 0xE1B7 , 0xE1A7 , 0xC009 , 0x0064 , 0xC009 , 0x0028 , 0xC009 , 0x0000 , 0xC009 , // 0x02F0  (.......O...........d...(.......
    0x004F , 0xE1DF , 0xE1CF , 0xE1D7 , 0xE1C7 , 0xC001 , 0x02E9 , 0x6F6D , 0x0031 , 0xE296 , 0xC076 , 0xE237 , 0xE247 , 0xE23F , 0xE24F , 0xE27D , // 0x0300  O.............mo1...v.7.G.?.O.}.
    0x600D , 0x600F , 0xE257 , 0xE267 , 0xE25F , 0xE26F , 0xE285 , 0x600D , 0x600F , 0xE23F , 0xE276 , 0xC027 , 0xC01E , 0xE1AF , 0xC01E , 0xE1CF , // 0x0310  .`.`W.g._.o....`.`?.v.'.........
    0xC009 , 0x009F , 0xC032 , 0xE28E , 0xC02D , 0x8752 , 0xC01E , 0xC073 , 0xE32D , 0xC00B , 0xFFFF , 0xE1FE , 0xC075 , 0xC02D , 0xC077 , 0xE30B , // 0x0320  ....2...-.R...s.-.......u.-.w...
    0xC001 , 0x0306 , 0x6F6D , 0x0032 , 0xE2B3 , 0xC076 , 0xE237 , 0xE247 , 0xE23F , 0xE24F , 0xE27D , 0x600D , 0x600F , 0xE257 , 0xE267 , 0xE25F , // 0x0330  ....mo2...v.7.G.?.O.}..`.`W.g._.
    0xE26F , 0xE285 , 0x600D , 0x600F , 0xE24F , 0xE276 , 0xC027 , 0xC01E , 0xE1BF , 0xC01E , 0xE1DF , 0xC009 , 0x004F , 0xC032 , 0xE28E , 0xC02D , // 0x0340  o....`.`O.v.'...........O.2...-.
    0x8752 , 0xC01E , 0xC073 , 0xE358 , 0xC00B , 0xFFFF , 0xE1FE , 0xC075 , 0xC02D , 0xC077 , 0xE336 , 0xC001 , 0x0331 , 0x6F6D , 0x0033 , 0xE2D0 , // 0x0350  R...s.X.......u.-.w.6...1.mo3...
    0xC076 , 0xE237 , 0xE247 , 0xE23F , 0xE24F , 0xE27D , 0x600D , 0x600F , 0xE257 , 0xE267 , 0xE25F , 0xE26F , 0xE285 , 0x600D , 0x600F , 0xE23F , // 0x0360  v.7.G.?.O.}..`.`W.g._.o....`.`?.
    0xE276 , 0xC028 , 0xC01E , 0xE1AF , 0xC01E , 0xE1CF , 0xC009 , 0x0000 , 0xC02F , 0xE28E , 0xC02D , 0x8752 , 0xC01E , 0xC073 , 0xE383 , 0xC00B , // 0x0370  v.(............./...-.R...s.....
    0xFFFF , 0xE1FE , 0xC075 , 0xC02D , 0xC077 , 0xE361 , 0xC001 , 0x035C , 0x6F6D , 0x0034 , 0xE2ED , 0xC076 , 0xE237 , 0xE247 , 0xE23F , 0xE24F , // 0x0380  ....u.-.w.a...\.mo4...v.7.G.?.O.
    0xE27D , 0x600D , 0x600F , 0xE257 , 0xE267 , 0xE25F , 0xE26F , 0xE285 , 0x600D , 0x600F , 0xE24F , 0xE276 , 0xC028 , 0xC01E , 0xE1BF , 0xC01E , // 0x0390  }..`.`W.g._.o....`.`O.v.(.......
    0xE1DF , 0xC009 , 0x0000 , 0xC02F , 0xE28E , 0xC02D , 0x8752 , 0xC01E , 0xC073 , 0xE3AE , 0xC00B , 0xFFFF , 0xE1FE , 0xC075 , 0xC02D , 0xC077 , // 0x03A0  ....../...-.R...s.........u.-.w.
    0xE38C , 0xC001 , 0x0387 , 0x6F6D , 0x0072 , 0xC009 , 0x00FF , 0xC009 , 0x0000 , 0xC009 , 0x0000 , 0xE06F , 0xE1ED , 0xC001 , 0x03B2 , 0x6F6D , // 0x03B0  ......mor.............o.......mo
    0x0067 , 0xC009 , 0x0000 , 0xC009 , 0x00FF , 0xC009 , 0x0000 , 0xE06F , 0xE1ED , 0xC001 , 0x03BE , 0x6F6D , 0x0062 , 0xC009 , 0x0000 , 0xC009 , // 0x03C0  g.............o.......mob.......
    0x0000 , 0xC009 , 0x00FF , 0xE06F , 0xE1ED , 0xC001 , 0x03CA , 0x6F6D , 0x0077 , 0xC009 , 0x00FF , 0xC009 , 0x00FF , 0xC009 , 0x00FF , 0xE06F , // 0x03D0  ......o.......mow.............o.
    0xE1ED , 0xC001 , 0x03D6 , 0x6F6D , 0x315F , 0x0000 , 0xC009 , 0x0001 , 0xE1E6 , 0xC009 , 0x0000 , 0xC009 , 0x0000 , 0xC009 , 0x0000 , 0xE06F , // 0x03E0  ......mo_1....................o.
    0xE1F5 , 0xC009 , 0x0000 , 0xE1FE , 0xC001 , 0x03E2 , 0x6F6D , 0x325F , 0x0000 , 0xE309 , 0xE334 , 0xE35F , 0xE38A , 0xE276 , 0xC009 , 0x0001 , // 0x03F0  ............mo_2....4._...v.....
    0xC027 , 0xC01E , 0xC009 , 0x000A , 0xC032 , 0xC073 , 0xE40B , 0xC022 , 0xC009 , 0x0001 , 0xC075 , 0xE1E6 , 0xC001 , 0x03F5 , 0x6F6D , 0x0000 , // 0x0400  '.......2.s...".....u.......mo..
    0xE3E6 , 0xC076 , 0xE3D9 , 0xE3F9 , 0xE3B5 , 0xE3F9 , 0xE3C1 , 0xE3F9 , 0xE3CD , 0xE3F9 , 0xE28E , 0xC077 , 0xE412 , 0xC001 , 0x040D , 0x6F74 , // 0x0410  ..v...................w.......to
    0x0000 , 0xC076 , 0xE131 , 0x8752 , 0xC077 , 0xE422 , 0xC001 , 0x041E , 0x636C , 0x5364 , 0x6174 , 0x7472 , 0x0000 , 0x600B , 0xC009 , 0x0BB8 , // 0x0420  ..v.1.R.w.".....lcdStart...`....
    0xC009 , 0x0007 , 0x6009 , 0xE120 , 0xE131 , 0xC009 , 0x03E8 , 0xC047 , 0xE120 , 0xE141 , 0xC009 , 0x0BB8 , 0xC047 , 0xE120 , 0xE159 , 0xC001 , // 0x0430  .....` .1.....G. .A.....G. .Y...
    
    
];
pub static COMPILED_EXTENSION_DICTIONARY : ComDictionary = ComDictionary {
    table: &COMPILED_SHELL_DICTIONARY_DATA,
    last_word: 0x0427,
    mask: DictionaryMask::YRSHELL_DICTIONARY_EXTENSION_COMPILED,
};
