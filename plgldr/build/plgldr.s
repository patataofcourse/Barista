	.arch armv6k
	.fpu vfp
	.eabi_attribute 28, 1
	.eabi_attribute 20, 1
	.eabi_attribute 21, 1
	.eabi_attribute 23, 3
	.eabi_attribute 24, 1
	.eabi_attribute 25, 1
	.eabi_attribute 26, 1
	.eabi_attribute 30, 2
	.eabi_attribute 34, 1
	.eabi_attribute 18, 4
	.file	"plgldr.c"
	.text
.Ltext0:
	.cfi_sections	.debug_frame
	.file 1 "/home/jmvieitez/Documents/Proyectos/SpiceRack/Barista/Barista/plgldr/src/plgldr.c"
	.section	.rodata.plgLdrInit.str1.4,"aMS",%progbits,1
	.align	2
.LC0:
	.ascii	"plg:ldr\000"
	.section	.text.plgLdrInit,"ax",%progbits
	.align	2
	.global	plgLdrInit
	.syntax unified
	.arm
	.type	plgLdrInit, %function
plgLdrInit:
.LFB97:
	.loc 1 8 1 view -0
	.cfi_startproc
	@ args = 0, pretend = 0, frame = 0
	@ frame_needed = 0, uses_anonymous_args = 0
	@ link register save eliminated.
	.loc 1 9 5 view .LVU1
.LVL0:
	.loc 1 11 5 view .LVU2
	.loc 1 11 9 is_stmt 0 view .LVU3
	mcr	p15, 0, r0, c7, c10, 5
	ldr	r3, .L6
.L3:
	ldrex	r2, [r3]
	add	r1, r2, #1
	strex	r0, r1, [r3]
	cmp	r0, #0
	bne	.L3
	.loc 1 11 8 view .LVU4
	cmp	r2, #0
	.loc 1 11 9 view .LVU5
	mcr	p15, 0, r0, c7, c10, 5
	.loc 1 11 8 view .LVU6
	beq	.L5
	.loc 1 13 5 is_stmt 1 view .LVU7
	.loc 1 14 1 is_stmt 0 view .LVU8
	mov	r0, #0
	bx	lr
.L5:
	.loc 1 12 9 is_stmt 1 view .LVU9
	.loc 1 12 15 is_stmt 0 view .LVU10
	ldr	r1, .L6+4
	ldr	r0, .L6+8
	b	svcConnectToPort
.LVL1:
.L7:
	.align	2
.L6:
	.word	plgLdrRefCount
	.word	.LC0
	.word	plgLdrHandle
	.cfi_endproc
.LFE97:
	.size	plgLdrInit, .-plgLdrInit
	.section	.text.plgLdrExit,"ax",%progbits
	.align	2
	.global	plgLdrExit
	.syntax unified
	.arm
	.type	plgLdrExit, %function
plgLdrExit:
.LFB98:
	.loc 1 17 1 is_stmt 1 view -0
	.cfi_startproc
	@ args = 0, pretend = 0, frame = 0
	@ frame_needed = 0, uses_anonymous_args = 0
	@ link register save eliminated.
	.loc 1 18 5 view .LVU12
	.loc 1 18 9 is_stmt 0 view .LVU13
	mcr	p15, 0, r0, c7, c10, 5
	ldr	r3, .L11
.L10:
	ldrex	r2, [r3]
	sub	r2, r2, #1
	strex	r1, r2, [r3]
	cmp	r1, #0
	bne	.L10
	.loc 1 18 8 view .LVU14
	cmp	r2, #0
	.loc 1 18 9 view .LVU15
	mcr	p15, 0, r0, c7, c10, 5
	.loc 1 18 8 view .LVU16
	bxne	lr
	.loc 1 20 5 is_stmt 1 view .LVU17
	ldr	r3, .L11+4
	ldr	r0, [r3]
	b	svcCloseHandle
.LVL2:
.L12:
	.align	2
.L11:
	.word	plgLdrRefCount
	.word	plgLdrHandle
	.cfi_endproc
.LFE98:
	.size	plgLdrExit, .-plgLdrExit
	.section	.text.PLGLDR__IsPluginLoaderEnabled,"ax",%progbits
	.align	2
	.global	PLGLDR__IsPluginLoaderEnabled
	.syntax unified
	.arm
	.type	PLGLDR__IsPluginLoaderEnabled, %function
PLGLDR__IsPluginLoaderEnabled:
.LVL3:
.LFB99:
	.loc 1 24 1 view -0
	.cfi_startproc
	@ args = 0, pretend = 0, frame = 0
	@ frame_needed = 0, uses_anonymous_args = 0
	.loc 1 25 5 view .LVU19
	.loc 1 27 5 view .LVU20
.LBB26:
.LBI26:
	.file 2 "/opt/devkitpro/libctru/include/3ds/svc.h"
	.loc 2 543 20 view .LVU21
	.loc 2 545 2 view .LVU22
.LBB27:
.LBI27:
	.loc 2 532 21 view .LVU23
.LBB28:
	.loc 2 534 2 view .LVU24
	.loc 2 535 2 view .LVU25
.LBE28:
.LBE27:
.LBE26:
	.loc 1 29 15 is_stmt 0 view .LVU26
	mov	r2, #131072
	.loc 1 30 18 view .LVU27
	ldr	r1, .L16
	.loc 1 24 1 view .LVU28
	push	{r4, r5, r6, lr}
	.cfi_def_cfa_offset 16
	.cfi_offset 4, -16
	.cfi_offset 5, -12
	.cfi_offset 6, -8
	.cfi_offset 14, -4
.LBB31:
.LBB30:
.LBB29:
	.loc 2 535 2 view .LVU29
	.syntax divided
@ 535 "/opt/devkitpro/libctru/include/3ds/svc.h" 1
	mrc p15, 0, r5, c13, c0, 3
@ 0 "" 2
.LVL4:
	.loc 2 536 2 is_stmt 1 view .LVU30
	.loc 2 536 2 is_stmt 0 view .LVU31
	.arm
	.syntax unified
.LBE29:
.LBE30:
.LBE31:
	.loc 1 29 5 is_stmt 1 view .LVU32
	.loc 1 24 1 is_stmt 0 view .LVU33
	mov	r4, r0
	.loc 1 29 15 view .LVU34
	str	r2, [r5, #128]
	.loc 1 30 5 is_stmt 1 view .LVU35
	.loc 1 30 18 is_stmt 0 view .LVU36
	ldr	r0, [r1]
.LVL5:
	.loc 1 30 18 view .LVU37
	bl	svcSendSyncRequest
.LVL6:
	.loc 1 30 8 view .LVU38
	cmp	r0, #0
	.loc 1 30 8 view .LVU39
	poplt	{r4, r5, r6, pc}
	.loc 1 32 9 is_stmt 1 view .LVU40
	.loc 1 33 28 is_stmt 0 view .LVU41
	ldr	r3, [r5, #136]
	.loc 1 32 13 view .LVU42
	ldr	r0, [r5, #132]
.LVL7:
	.loc 1 33 9 is_stmt 1 view .LVU43
	.loc 1 33 28 is_stmt 0 view .LVU44
	subs	r3, r3, #0
	movne	r3, #1
	strb	r3, [r4]
	.loc 1 35 5 is_stmt 1 view .LVU45
	.loc 1 36 1 is_stmt 0 view .LVU46
	pop	{r4, r5, r6, pc}
.L17:
	.align	2
.L16:
	.word	plgLdrHandle
	.cfi_endproc
.LFE99:
	.size	PLGLDR__IsPluginLoaderEnabled, .-PLGLDR__IsPluginLoaderEnabled
	.section	.text.PLGLDR__SetPluginLoaderState,"ax",%progbits
	.align	2
	.global	PLGLDR__SetPluginLoaderState
	.syntax unified
	.arm
	.type	PLGLDR__SetPluginLoaderState, %function
PLGLDR__SetPluginLoaderState:
.LVL8:
.LFB100:
	.loc 1 39 1 is_stmt 1 view -0
	.cfi_startproc
	@ args = 0, pretend = 0, frame = 0
	@ frame_needed = 0, uses_anonymous_args = 0
	.loc 1 40 5 view .LVU48
	.loc 1 42 5 view .LVU49
.LBB32:
.LBI32:
	.loc 2 543 20 view .LVU50
	.loc 2 545 2 view .LVU51
.LBB33:
.LBI33:
	.loc 2 532 21 view .LVU52
.LBB34:
	.loc 2 534 2 view .LVU53
	.loc 2 535 2 view .LVU54
.LBE34:
.LBE33:
.LBE32:
	.loc 1 39 1 is_stmt 0 view .LVU55
	mov	r3, r0
	.loc 1 44 15 view .LVU56
	ldr	r2, .L21
	.loc 1 47 18 view .LVU57
	ldr	r1, .L21+4
	.loc 1 39 1 view .LVU58
	push	{r4, lr}
	.cfi_def_cfa_offset 8
	.cfi_offset 4, -8
	.cfi_offset 14, -4
.LBB37:
.LBB36:
.LBB35:
	.loc 2 535 2 view .LVU59
	.syntax divided
@ 535 "/opt/devkitpro/libctru/include/3ds/svc.h" 1
	mrc p15, 0, r4, c13, c0, 3
@ 0 "" 2
.LVL9:
	.loc 2 536 2 is_stmt 1 view .LVU60
	.loc 2 536 2 is_stmt 0 view .LVU61
	.arm
	.syntax unified
.LBE35:
.LBE36:
.LBE37:
	.loc 1 44 5 is_stmt 1 view .LVU62
	.loc 1 47 18 is_stmt 0 view .LVU63
	ldr	r0, [r1]
.LVL10:
	.loc 1 45 17 view .LVU64
	strd	r2, [r4, #128]
	.loc 1 47 5 is_stmt 1 view .LVU65
	.loc 1 47 18 is_stmt 0 view .LVU66
	bl	svcSendSyncRequest
.LVL11:
	.loc 1 47 8 view .LVU67
	cmp	r0, #0
	.loc 1 49 9 is_stmt 1 view .LVU68
	.loc 1 49 13 is_stmt 0 view .LVU69
	ldrge	r0, [r4, #132]
.LVL12:
	.loc 1 51 5 is_stmt 1 view .LVU70
	.loc 1 52 1 is_stmt 0 view .LVU71
	pop	{r4, pc}
.L22:
	.align	2
.L21:
	.word	196672
	.word	plgLdrHandle
	.cfi_endproc
.LFE100:
	.size	PLGLDR__SetPluginLoaderState, .-PLGLDR__SetPluginLoaderState
	.section	.text.PLGLDR__SetPluginLoadParameters,"ax",%progbits
	.align	2
	.global	PLGLDR__SetPluginLoadParameters
	.syntax unified
	.arm
	.type	PLGLDR__SetPluginLoadParameters, %function
PLGLDR__SetPluginLoadParameters:
.LVL13:
.LFB101:
	.loc 1 55 1 is_stmt 1 view -0
	.cfi_startproc
	@ args = 0, pretend = 0, frame = 0
	@ frame_needed = 0, uses_anonymous_args = 0
	.loc 1 56 5 view .LVU73
	.loc 1 58 5 view .LVU74
.LBB38:
.LBI38:
	.loc 2 543 20 view .LVU75
	.loc 2 545 2 view .LVU76
.LBB39:
.LBI39:
	.loc 2 532 21 view .LVU77
.LBB40:
	.loc 2 534 2 view .LVU78
	.loc 2 535 2 view .LVU79
.LBE40:
.LBE39:
.LBE38:
	.loc 1 55 1 is_stmt 0 view .LVU80
	mov	r3, r0
	.loc 1 68 18 view .LVU81
	ldr	r1, .L26
	.loc 1 60 15 view .LVU82
	ldr	r2, .L26+4
	.loc 1 68 18 view .LVU83
	ldr	r0, [r1]
.LVL14:
	.loc 1 61 17 view .LVU84
	ldrb	r1, [r3]	@ zero_extendqisi2
	.loc 1 55 1 view .LVU85
	push	{r4, lr}
	.cfi_def_cfa_offset 8
	.cfi_offset 4, -8
	.cfi_offset 14, -4
.LBB43:
.LBB42:
.LBB41:
	.loc 2 535 2 view .LVU86
	.syntax divided
@ 535 "/opt/devkitpro/libctru/include/3ds/svc.h" 1
	mrc p15, 0, r4, c13, c0, 3
@ 0 "" 2
.LVL15:
	.loc 2 536 2 is_stmt 1 view .LVU87
	.loc 2 536 2 is_stmt 0 view .LVU88
	.arm
	.syntax unified
.LBE41:
.LBE42:
.LBE43:
	.loc 1 60 5 is_stmt 1 view .LVU89
	.loc 1 60 15 is_stmt 0 view .LVU90
	str	r2, [r4, #128]
	.loc 1 61 5 is_stmt 1 view .LVU91
	.loc 1 61 17 is_stmt 0 view .LVU92
	str	r1, [r4, #132]
	.loc 1 62 5 is_stmt 1 view .LVU93
	.loc 1 65 15 is_stmt 0 view .LVU94
	ldr	r2, .L26+8
	.loc 1 62 27 view .LVU95
	ldr	r1, [r3, #4]
	.loc 1 63 15 view .LVU96
	ldr	ip, .L26+12
	.loc 1 65 15 view .LVU97
	str	r2, [r4, #148]
	.loc 1 66 22 view .LVU98
	add	r2, r3, #264
	.loc 1 64 22 view .LVU99
	add	r3, r3, #8
.LVL16:
	.loc 1 63 15 view .LVU100
	str	ip, [r4, #140]
	.loc 1 62 15 view .LVU101
	str	r1, [r4, #136]
	.loc 1 63 5 is_stmt 1 view .LVU102
	.loc 1 64 5 view .LVU103
	.loc 1 64 15 is_stmt 0 view .LVU104
	str	r3, [r4, #144]
	.loc 1 65 5 is_stmt 1 view .LVU105
	.loc 1 66 5 view .LVU106
	.loc 1 66 15 is_stmt 0 view .LVU107
	str	r2, [r4, #152]
	.loc 1 68 5 is_stmt 1 view .LVU108
	.loc 1 68 18 is_stmt 0 view .LVU109
	bl	svcSendSyncRequest
.LVL17:
	.loc 1 68 8 view .LVU110
	cmp	r0, #0
	.loc 1 70 9 is_stmt 1 view .LVU111
	.loc 1 70 13 is_stmt 0 view .LVU112
	ldrge	r0, [r4, #132]
.LVL18:
	.loc 1 72 5 is_stmt 1 view .LVU113
	.loc 1 73 1 is_stmt 0 view .LVU114
	pop	{r4, pc}
.L27:
	.align	2
.L26:
	.word	plgLdrHandle
	.word	262276
	.word	2058
	.word	4106
	.cfi_endproc
.LFE101:
	.size	PLGLDR__SetPluginLoadParameters, .-PLGLDR__SetPluginLoadParameters
	.section	.bss.plgLdrRefCount,"aw",%nobits
	.align	2
	.type	plgLdrRefCount, %object
	.size	plgLdrRefCount, 4
plgLdrRefCount:
	.space	4
	.section	.bss.plgLdrHandle,"aw",%nobits
	.align	2
	.type	plgLdrHandle, %object
	.size	plgLdrHandle, 4
plgLdrHandle:
	.space	4
	.text
.Letext0:
	.file 3 "/opt/devkitpro/devkitARM/arm-none-eabi/include/machine/_default_types.h"
	.file 4 "/opt/devkitpro/devkitARM/arm-none-eabi/include/sys/_stdint.h"
	.file 5 "/opt/devkitpro/devkitARM/lib/gcc/arm-none-eabi/12.1.0/include/stddef.h"
	.file 6 "/opt/devkitpro/libctru/include/3ds/types.h"
	.file 7 "/opt/devkitpro/libctru/include/3ds/ipc.h"
	.file 8 "/opt/devkitpro/libctru/include/3ds/services/csnd.h"
	.file 9 "/opt/devkitpro/libctru/include/3ds/services/ndm.h"
	.file 10 "/opt/devkitpro/libctru/include/3ds/gpu/enums.h"
	.file 11 "/opt/devkitpro/libctru/include/3ds/ndsp/channel.h"
	.file 12 "/opt/devkitpro/libctru/include/3ds/applets/error.h"
	.file 13 "/home/jmvieitez/Documents/Proyectos/SpiceRack/Barista/Barista/plgldr/include/plgldr.h"
	.section	.debug_info,"",%progbits
.Ldebug_info0:
	.4byte	0x5b9
	.2byte	0x5
	.byte	0x1
	.byte	0x4
	.4byte	.Ldebug_abbrev0
	.uleb128 0x19
	.4byte	.LASF73
	.byte	0x1d
	.4byte	.LASF74
	.4byte	.LASF75
	.4byte	.LLRL15
	.4byte	0
	.4byte	.Ldebug_line0
	.uleb128 0x2
	.byte	0x1
	.byte	0x6
	.4byte	.LASF0
	.uleb128 0x3
	.4byte	.LASF3
	.byte	0x3
	.byte	0x2b
	.byte	0x17
	.4byte	0x39
	.uleb128 0x2
	.byte	0x1
	.byte	0x8
	.4byte	.LASF1
	.uleb128 0x2
	.byte	0x2
	.byte	0x5
	.4byte	.LASF2
	.uleb128 0x3
	.4byte	.LASF4
	.byte	0x3
	.byte	0x39
	.byte	0x1c
	.4byte	0x53
	.uleb128 0x2
	.byte	0x2
	.byte	0x7
	.4byte	.LASF5
	.uleb128 0x3
	.4byte	.LASF6
	.byte	0x3
	.byte	0x4d
	.byte	0x12
	.4byte	0x66
	.uleb128 0x2
	.byte	0x4
	.byte	0x5
	.4byte	.LASF7
	.uleb128 0x3
	.4byte	.LASF8
	.byte	0x3
	.byte	0x4f
	.byte	0x1b
	.4byte	0x79
	.uleb128 0x2
	.byte	0x4
	.byte	0x7
	.4byte	.LASF9
	.uleb128 0x2
	.byte	0x8
	.byte	0x5
	.4byte	.LASF10
	.uleb128 0x2
	.byte	0x8
	.byte	0x7
	.4byte	.LASF11
	.uleb128 0x1a
	.byte	0x4
	.byte	0x5
	.ascii	"int\000"
	.uleb128 0x2
	.byte	0x4
	.byte	0x7
	.4byte	.LASF12
	.uleb128 0x3
	.4byte	.LASF13
	.byte	0x4
	.byte	0x18
	.byte	0x13
	.4byte	0x2d
	.uleb128 0x3
	.4byte	.LASF14
	.byte	0x4
	.byte	0x24
	.byte	0x14
	.4byte	0x47
	.uleb128 0x3
	.4byte	.LASF15
	.byte	0x4
	.byte	0x2c
	.byte	0x13
	.4byte	0x5a
	.uleb128 0x3
	.4byte	.LASF16
	.byte	0x4
	.byte	0x30
	.byte	0x14
	.4byte	0x6d
	.uleb128 0x3
	.4byte	.LASF17
	.byte	0x5
	.byte	0xd6
	.byte	0x16
	.4byte	0x95
	.uleb128 0x2
	.byte	0x8
	.byte	0x4
	.4byte	.LASF18
	.uleb128 0x7
	.ascii	"u8\000"
	.byte	0x15
	.byte	0x11
	.4byte	0x9c
	.uleb128 0x7
	.ascii	"u16\000"
	.byte	0x16
	.byte	0x12
	.4byte	0xa8
	.uleb128 0x7
	.ascii	"u32\000"
	.byte	0x17
	.byte	0x12
	.4byte	0xc0
	.uleb128 0x7
	.ascii	"s32\000"
	.byte	0x1c
	.byte	0x11
	.4byte	0xb4
	.uleb128 0x3
	.4byte	.LASF19
	.byte	0x6
	.byte	0x29
	.byte	0xd
	.4byte	0xf4
	.uleb128 0x1b
	.4byte	0x10a
	.uleb128 0x3
	.4byte	.LASF20
	.byte	0x6
	.byte	0x2a
	.byte	0xd
	.4byte	0xff
	.uleb128 0x1c
	.byte	0x4
	.uleb128 0x2
	.byte	0x8
	.byte	0x4
	.4byte	.LASF21
	.uleb128 0x2
	.byte	0x4
	.byte	0x4
	.4byte	.LASF22
	.uleb128 0x4
	.byte	0x1
	.4byte	0x39
	.byte	0x7
	.byte	0xb
	.byte	0x1
	.4byte	0x157
	.uleb128 0x1
	.4byte	.LASF23
	.byte	0x2
	.uleb128 0x1
	.4byte	.LASF24
	.byte	0x4
	.uleb128 0x1
	.4byte	.LASF25
	.byte	0x6
	.byte	0
	.uleb128 0x3
	.4byte	.LASF26
	.byte	0x7
	.byte	0xf
	.byte	0x3
	.4byte	0x137
	.uleb128 0x2
	.byte	0x1
	.byte	0x8
	.4byte	.LASF27
	.uleb128 0x1d
	.4byte	0x163
	.uleb128 0x2
	.byte	0x1
	.byte	0x2
	.4byte	.LASF28
	.uleb128 0x5
	.4byte	0xf4
	.uleb128 0x4
	.byte	0x1
	.4byte	0x39
	.byte	0x8
	.byte	0x28
	.byte	0x1
	.4byte	0x1a1
	.uleb128 0x1
	.4byte	.LASF29
	.byte	0
	.uleb128 0x1
	.4byte	.LASF30
	.byte	0x1
	.uleb128 0x1
	.4byte	.LASF31
	.byte	0x2
	.uleb128 0x1
	.4byte	.LASF32
	.byte	0x3
	.byte	0
	.uleb128 0x4
	.byte	0x1
	.4byte	0x39
	.byte	0x8
	.byte	0x31
	.byte	0x1
	.4byte	0x1c7
	.uleb128 0x1
	.4byte	.LASF33
	.byte	0
	.uleb128 0x1
	.4byte	.LASF34
	.byte	0x1
	.uleb128 0x1
	.4byte	.LASF35
	.byte	0x2
	.uleb128 0x1
	.4byte	.LASF36
	.byte	0x3
	.byte	0
	.uleb128 0x4
	.byte	0x1
	.4byte	0x39
	.byte	0x9
	.byte	0x21
	.byte	0xe
	.4byte	0x1ed
	.uleb128 0x1
	.4byte	.LASF37
	.byte	0
	.uleb128 0x1
	.4byte	.LASF38
	.byte	0x1
	.uleb128 0x1
	.4byte	.LASF39
	.byte	0x2
	.uleb128 0x1
	.4byte	.LASF40
	.byte	0x3
	.byte	0
	.uleb128 0x5
	.4byte	0x16a
	.uleb128 0x1e
	.byte	0x7
	.byte	0x1
	.4byte	0x39
	.byte	0xa
	.2byte	0x1f5
	.byte	0x1
	.4byte	0x20e
	.uleb128 0x1
	.4byte	.LASF41
	.byte	0
	.uleb128 0x1
	.4byte	.LASF42
	.byte	0x1
	.byte	0
	.uleb128 0x4
	.byte	0x1
	.4byte	0x39
	.byte	0xb
	.byte	0xb
	.byte	0x1
	.4byte	0x22e
	.uleb128 0x1
	.4byte	.LASF43
	.byte	0
	.uleb128 0x1
	.4byte	.LASF44
	.byte	0x1
	.uleb128 0x1
	.4byte	.LASF45
	.byte	0x2
	.byte	0
	.uleb128 0x4
	.byte	0x2
	.4byte	0x53
	.byte	0xc
	.byte	0x9
	.byte	0x1
	.4byte	0x24a
	.uleb128 0x14
	.4byte	.LASF46
	.2byte	0x100
	.uleb128 0x14
	.4byte	.LASF47
	.2byte	0x200
	.byte	0
	.uleb128 0x1f
	.2byte	0x188
	.byte	0xd
	.byte	0x5
	.byte	0x9
	.4byte	0x287
	.uleb128 0xa
	.4byte	.LASF48
	.byte	0x7
	.byte	0xc
	.4byte	0x16f
	.byte	0
	.uleb128 0xa
	.4byte	.LASF49
	.byte	0x8
	.byte	0x9
	.4byte	0xf4
	.byte	0x4
	.uleb128 0xa
	.4byte	.LASF50
	.byte	0x9
	.byte	0xa
	.4byte	0x287
	.byte	0x8
	.uleb128 0x20
	.4byte	.LASF51
	.byte	0xd
	.byte	0xa
	.byte	0x9
	.4byte	0x297
	.2byte	0x108
	.byte	0
	.uleb128 0x15
	.4byte	0x163
	.4byte	0x297
	.uleb128 0x16
	.4byte	0x95
	.byte	0xff
	.byte	0
	.uleb128 0x15
	.4byte	0xf4
	.4byte	0x2a7
	.uleb128 0x16
	.4byte	0x95
	.byte	0x1f
	.byte	0
	.uleb128 0x3
	.4byte	.LASF52
	.byte	0xd
	.byte	0xb
	.byte	0x3
	.4byte	0x24a
	.uleb128 0x17
	.4byte	.LASF53
	.byte	0x4
	.byte	0xf
	.4byte	0x10a
	.uleb128 0x5
	.byte	0x3
	.4byte	plgLdrHandle
	.uleb128 0x17
	.4byte	.LASF54
	.byte	0x5
	.byte	0xc
	.4byte	0x8e
	.uleb128 0x5
	.byte	0x3
	.4byte	plgLdrRefCount
	.uleb128 0xb
	.4byte	.LASF55
	.2byte	0x43a
	.4byte	0x11b
	.4byte	0x2ea
	.uleb128 0x8
	.4byte	0x10a
	.byte	0
	.uleb128 0xb
	.4byte	.LASF56
	.2byte	0x486
	.4byte	0x11b
	.4byte	0x2ff
	.uleb128 0x8
	.4byte	0x10a
	.byte	0
	.uleb128 0xb
	.4byte	.LASF57
	.2byte	0x2f4
	.4byte	0x11b
	.4byte	0x319
	.uleb128 0x8
	.4byte	0x319
	.uleb128 0x8
	.4byte	0x1ed
	.byte	0
	.uleb128 0x5
	.4byte	0x116
	.uleb128 0x9
	.4byte	.LASF59
	.byte	0x36
	.4byte	0x11b
	.4byte	.LFB101
	.4byte	.LFE101-.LFB101
	.uleb128 0x1
	.byte	0x9c
	.4byte	0x3ad
	.uleb128 0xc
	.4byte	.LASF61
	.byte	0x36
	.byte	0x3e
	.4byte	0x3ad
	.4byte	.LLST10
	.4byte	.LVUS10
	.uleb128 0xd
	.ascii	"res\000"
	.byte	0x38
	.4byte	0x11b
	.4byte	.LLST11
	.4byte	.LVUS11
	.uleb128 0xe
	.4byte	.LASF58
	.byte	0x3a
	.4byte	0x176
	.4byte	.LLST12
	.4byte	.LVUS12
	.uleb128 0xf
	.4byte	0x537
	.4byte	.LBI38
	.byte	.LVU75
	.4byte	.LLRL13
	.byte	0x3a
	.4byte	0x3a3
	.uleb128 0x10
	.4byte	0x545
	.4byte	.LBI39
	.byte	.LVU77
	.4byte	.LLRL13
	.uleb128 0x11
	.4byte	.LLRL13
	.uleb128 0x12
	.4byte	0x557
	.4byte	.LLST14
	.4byte	.LVUS14
	.byte	0
	.byte	0
	.byte	0
	.uleb128 0x13
	.4byte	.LVL17
	.4byte	0x2d5
	.byte	0
	.uleb128 0x5
	.4byte	0x2a7
	.uleb128 0x9
	.4byte	.LASF60
	.byte	0x26
	.4byte	0x11b
	.4byte	.LFB100
	.4byte	.LFE100-.LFB100
	.uleb128 0x1
	.byte	0x9c
	.4byte	0x441
	.uleb128 0xc
	.4byte	.LASF62
	.byte	0x26
	.byte	0x2a
	.4byte	0x16f
	.4byte	.LLST5
	.4byte	.LVUS5
	.uleb128 0xd
	.ascii	"res\000"
	.byte	0x28
	.4byte	0x11b
	.4byte	.LLST6
	.4byte	.LVUS6
	.uleb128 0xe
	.4byte	.LASF58
	.byte	0x2a
	.4byte	0x176
	.4byte	.LLST7
	.4byte	.LVUS7
	.uleb128 0xf
	.4byte	0x537
	.4byte	.LBI32
	.byte	.LVU50
	.4byte	.LLRL8
	.byte	0x2a
	.4byte	0x437
	.uleb128 0x10
	.4byte	0x545
	.4byte	.LBI33
	.byte	.LVU52
	.4byte	.LLRL8
	.uleb128 0x11
	.4byte	.LLRL8
	.uleb128 0x12
	.4byte	0x557
	.4byte	.LLST9
	.4byte	.LVUS9
	.byte	0
	.byte	0
	.byte	0
	.uleb128 0x13
	.4byte	.LVL11
	.4byte	0x2d5
	.byte	0
	.uleb128 0x9
	.4byte	.LASF63
	.byte	0x17
	.4byte	0x11b
	.4byte	.LFB99
	.4byte	.LFE99-.LFB99
	.uleb128 0x1
	.byte	0x9c
	.4byte	0x4d0
	.uleb128 0xc
	.4byte	.LASF64
	.byte	0x17
	.byte	0x2c
	.4byte	0x4d0
	.4byte	.LLST0
	.4byte	.LVUS0
	.uleb128 0xd
	.ascii	"res\000"
	.byte	0x19
	.4byte	0x11b
	.4byte	.LLST1
	.4byte	.LVUS1
	.uleb128 0xe
	.4byte	.LASF58
	.byte	0x1b
	.4byte	0x176
	.4byte	.LLST2
	.4byte	.LVUS2
	.uleb128 0xf
	.4byte	0x537
	.4byte	.LBI26
	.byte	.LVU21
	.4byte	.LLRL3
	.byte	0x1b
	.4byte	0x4c6
	.uleb128 0x10
	.4byte	0x545
	.4byte	.LBI27
	.byte	.LVU23
	.4byte	.LLRL3
	.uleb128 0x11
	.4byte	.LLRL3
	.uleb128 0x12
	.4byte	0x557
	.4byte	.LLST4
	.4byte	.LVUS4
	.byte	0
	.byte	0
	.byte	0
	.uleb128 0x13
	.4byte	.LVL6
	.4byte	0x2d5
	.byte	0
	.uleb128 0x5
	.4byte	0x16f
	.uleb128 0x21
	.4byte	.LASF76
	.byte	0x1
	.byte	0x10
	.byte	0x6
	.4byte	.LFB98
	.4byte	.LFE98-.LFB98
	.uleb128 0x1
	.byte	0x9c
	.4byte	0x4f5
	.uleb128 0x22
	.4byte	.LVL2
	.4byte	0x2ea
	.byte	0
	.uleb128 0x9
	.4byte	.LASF65
	.byte	0x7
	.4byte	0x11b
	.4byte	.LFB97
	.4byte	.LFE97-.LFB97
	.uleb128 0x1
	.byte	0x9c
	.4byte	0x537
	.uleb128 0x23
	.ascii	"res\000"
	.byte	0x1
	.byte	0x9
	.byte	0xc
	.4byte	0x11b
	.byte	0
	.uleb128 0x24
	.4byte	.LVL1
	.4byte	0x2ff
	.uleb128 0x18
	.uleb128 0x1
	.byte	0x50
	.uleb128 0x5
	.byte	0x3
	.4byte	plgLdrHandle
	.uleb128 0x18
	.uleb128 0x1
	.byte	0x51
	.uleb128 0x5
	.byte	0x3
	.4byte	.LC0
	.byte	0
	.byte	0
	.uleb128 0x25
	.4byte	.LASF77
	.byte	0x2
	.2byte	0x21f
	.byte	0x14
	.4byte	0x176
	.byte	0x3
	.uleb128 0x26
	.4byte	.LASF66
	.byte	0x2
	.2byte	0x214
	.byte	0x15
	.4byte	0x127
	.byte	0x3
	.4byte	0x565
	.uleb128 0x27
	.ascii	"ret\000"
	.byte	0x2
	.2byte	0x216
	.byte	0x8
	.4byte	0x127
	.byte	0
	.uleb128 0x28
	.4byte	.LASF67
	.byte	0x7
	.byte	0x73
	.byte	0x13
	.4byte	0xf4
	.byte	0x3
	.4byte	0x58d
	.uleb128 0x6
	.4byte	.LASF68
	.byte	0x73
	.byte	0x2a
	.4byte	0xcc
	.uleb128 0x6
	.4byte	.LASF69
	.byte	0x73
	.byte	0x41
	.4byte	0x157
	.byte	0
	.uleb128 0x29
	.4byte	.LASF78
	.byte	0x7
	.byte	0x1d
	.byte	0x13
	.4byte	0xf4
	.byte	0x3
	.uleb128 0x6
	.4byte	.LASF70
	.byte	0x1d
	.byte	0x26
	.4byte	0xe9
	.uleb128 0x6
	.4byte	.LASF71
	.byte	0x1d
	.byte	0x3b
	.4byte	0x95
	.uleb128 0x6
	.4byte	.LASF72
	.byte	0x1d
	.byte	0x53
	.4byte	0x95
	.byte	0
	.byte	0
	.section	.debug_abbrev,"",%progbits
.Ldebug_abbrev0:
	.uleb128 0x1
	.uleb128 0x28
	.byte	0
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x1c
	.uleb128 0xb
	.byte	0
	.byte	0
	.uleb128 0x2
	.uleb128 0x24
	.byte	0
	.uleb128 0xb
	.uleb128 0xb
	.uleb128 0x3e
	.uleb128 0xb
	.uleb128 0x3
	.uleb128 0xe
	.byte	0
	.byte	0
	.uleb128 0x3
	.uleb128 0x16
	.byte	0
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x4
	.uleb128 0x4
	.byte	0x1
	.uleb128 0x3e
	.uleb128 0x21
	.sleb128 7
	.uleb128 0xb
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x1
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x5
	.uleb128 0xf
	.byte	0
	.uleb128 0xb
	.uleb128 0x21
	.sleb128 4
	.uleb128 0x49
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x6
	.uleb128 0x5
	.byte	0
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0x21
	.sleb128 7
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x7
	.uleb128 0x16
	.byte	0
	.uleb128 0x3
	.uleb128 0x8
	.uleb128 0x3a
	.uleb128 0x21
	.sleb128 6
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x8
	.uleb128 0x5
	.byte	0
	.uleb128 0x49
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x9
	.uleb128 0x2e
	.byte	0x1
	.uleb128 0x3f
	.uleb128 0x19
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0x21
	.sleb128 1
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0x21
	.sleb128 8
	.uleb128 0x27
	.uleb128 0x19
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x11
	.uleb128 0x1
	.uleb128 0x12
	.uleb128 0x6
	.uleb128 0x40
	.uleb128 0x18
	.uleb128 0x7a
	.uleb128 0x19
	.uleb128 0x1
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0xa
	.uleb128 0xd
	.byte	0
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0x21
	.sleb128 13
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x38
	.uleb128 0xb
	.byte	0
	.byte	0
	.uleb128 0xb
	.uleb128 0x2e
	.byte	0x1
	.uleb128 0x3f
	.uleb128 0x19
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0x21
	.sleb128 2
	.uleb128 0x3b
	.uleb128 0x5
	.uleb128 0x39
	.uleb128 0x21
	.sleb128 8
	.uleb128 0x27
	.uleb128 0x19
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x3c
	.uleb128 0x19
	.uleb128 0x1
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0xc
	.uleb128 0x5
	.byte	0
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0x21
	.sleb128 1
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x2
	.uleb128 0x17
	.uleb128 0x2137
	.uleb128 0x17
	.byte	0
	.byte	0
	.uleb128 0xd
	.uleb128 0x34
	.byte	0
	.uleb128 0x3
	.uleb128 0x8
	.uleb128 0x3a
	.uleb128 0x21
	.sleb128 1
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0x21
	.sleb128 12
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x2
	.uleb128 0x17
	.uleb128 0x2137
	.uleb128 0x17
	.byte	0
	.byte	0
	.uleb128 0xe
	.uleb128 0x34
	.byte	0
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0x21
	.sleb128 1
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0x21
	.sleb128 10
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x2
	.uleb128 0x17
	.uleb128 0x2137
	.uleb128 0x17
	.byte	0
	.byte	0
	.uleb128 0xf
	.uleb128 0x1d
	.byte	0x1
	.uleb128 0x31
	.uleb128 0x13
	.uleb128 0x52
	.uleb128 0x1
	.uleb128 0x2138
	.uleb128 0xb
	.uleb128 0x55
	.uleb128 0x17
	.uleb128 0x58
	.uleb128 0x21
	.sleb128 1
	.uleb128 0x59
	.uleb128 0xb
	.uleb128 0x57
	.uleb128 0x21
	.sleb128 19
	.uleb128 0x1
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x10
	.uleb128 0x1d
	.byte	0x1
	.uleb128 0x31
	.uleb128 0x13
	.uleb128 0x52
	.uleb128 0x1
	.uleb128 0x2138
	.uleb128 0xb
	.uleb128 0x55
	.uleb128 0x17
	.uleb128 0x58
	.uleb128 0x21
	.sleb128 2
	.uleb128 0x59
	.uleb128 0x21
	.sleb128 545
	.uleb128 0x57
	.uleb128 0x21
	.sleb128 21
	.byte	0
	.byte	0
	.uleb128 0x11
	.uleb128 0xb
	.byte	0x1
	.uleb128 0x55
	.uleb128 0x17
	.byte	0
	.byte	0
	.uleb128 0x12
	.uleb128 0x34
	.byte	0
	.uleb128 0x31
	.uleb128 0x13
	.uleb128 0x2
	.uleb128 0x17
	.uleb128 0x2137
	.uleb128 0x17
	.byte	0
	.byte	0
	.uleb128 0x13
	.uleb128 0x48
	.byte	0
	.uleb128 0x7d
	.uleb128 0x1
	.uleb128 0x7f
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x14
	.uleb128 0x28
	.byte	0
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x1c
	.uleb128 0x5
	.byte	0
	.byte	0
	.uleb128 0x15
	.uleb128 0x1
	.byte	0x1
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x1
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x16
	.uleb128 0x21
	.byte	0
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x2f
	.uleb128 0xb
	.byte	0
	.byte	0
	.uleb128 0x17
	.uleb128 0x34
	.byte	0
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0x21
	.sleb128 1
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x2
	.uleb128 0x18
	.byte	0
	.byte	0
	.uleb128 0x18
	.uleb128 0x49
	.byte	0
	.uleb128 0x2
	.uleb128 0x18
	.uleb128 0x7e
	.uleb128 0x18
	.byte	0
	.byte	0
	.uleb128 0x19
	.uleb128 0x11
	.byte	0x1
	.uleb128 0x25
	.uleb128 0xe
	.uleb128 0x13
	.uleb128 0xb
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x1b
	.uleb128 0xe
	.uleb128 0x55
	.uleb128 0x17
	.uleb128 0x11
	.uleb128 0x1
	.uleb128 0x10
	.uleb128 0x17
	.byte	0
	.byte	0
	.uleb128 0x1a
	.uleb128 0x24
	.byte	0
	.uleb128 0xb
	.uleb128 0xb
	.uleb128 0x3e
	.uleb128 0xb
	.uleb128 0x3
	.uleb128 0x8
	.byte	0
	.byte	0
	.uleb128 0x1b
	.uleb128 0x35
	.byte	0
	.uleb128 0x49
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x1c
	.uleb128 0xf
	.byte	0
	.uleb128 0xb
	.uleb128 0xb
	.byte	0
	.byte	0
	.uleb128 0x1d
	.uleb128 0x26
	.byte	0
	.uleb128 0x49
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x1e
	.uleb128 0x4
	.byte	0x1
	.uleb128 0x3e
	.uleb128 0xb
	.uleb128 0xb
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0x5
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x1
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x1f
	.uleb128 0x13
	.byte	0x1
	.uleb128 0xb
	.uleb128 0x5
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x1
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x20
	.uleb128 0xd
	.byte	0
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x38
	.uleb128 0x5
	.byte	0
	.byte	0
	.uleb128 0x21
	.uleb128 0x2e
	.byte	0x1
	.uleb128 0x3f
	.uleb128 0x19
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x27
	.uleb128 0x19
	.uleb128 0x11
	.uleb128 0x1
	.uleb128 0x12
	.uleb128 0x6
	.uleb128 0x40
	.uleb128 0x18
	.uleb128 0x7a
	.uleb128 0x19
	.uleb128 0x1
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x22
	.uleb128 0x48
	.byte	0
	.uleb128 0x7d
	.uleb128 0x1
	.uleb128 0x82
	.uleb128 0x19
	.uleb128 0x7f
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x23
	.uleb128 0x34
	.byte	0
	.uleb128 0x3
	.uleb128 0x8
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x1c
	.uleb128 0xb
	.byte	0
	.byte	0
	.uleb128 0x24
	.uleb128 0x48
	.byte	0x1
	.uleb128 0x7d
	.uleb128 0x1
	.uleb128 0x82
	.uleb128 0x19
	.uleb128 0x7f
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x25
	.uleb128 0x2e
	.byte	0
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0x5
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x27
	.uleb128 0x19
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x20
	.uleb128 0xb
	.byte	0
	.byte	0
	.uleb128 0x26
	.uleb128 0x2e
	.byte	0x1
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0x5
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x27
	.uleb128 0x19
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x20
	.uleb128 0xb
	.uleb128 0x1
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x27
	.uleb128 0x34
	.byte	0
	.uleb128 0x3
	.uleb128 0x8
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0x5
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x28
	.uleb128 0x2e
	.byte	0x1
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x27
	.uleb128 0x19
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x20
	.uleb128 0xb
	.uleb128 0x1
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x29
	.uleb128 0x2e
	.byte	0x1
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x27
	.uleb128 0x19
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x20
	.uleb128 0xb
	.byte	0
	.byte	0
	.byte	0
	.section	.debug_loclists,"",%progbits
	.4byte	.Ldebug_loc3-.Ldebug_loc2
.Ldebug_loc2:
	.2byte	0x5
	.byte	0x4
	.byte	0
	.4byte	0
.Ldebug_loc0:
.LVUS10:
	.uleb128 0
	.uleb128 .LVU84
	.uleb128 .LVU84
	.uleb128 .LVU100
	.uleb128 .LVU100
	.uleb128 .LVU110
	.uleb128 .LVU110
	.uleb128 0
.LLST10:
	.byte	0x6
	.4byte	.LVL13
	.byte	0x4
	.uleb128 .LVL13-.LVL13
	.uleb128 .LVL14-.LVL13
	.uleb128 0x1
	.byte	0x50
	.byte	0x4
	.uleb128 .LVL14-.LVL13
	.uleb128 .LVL16-.LVL13
	.uleb128 0x1
	.byte	0x53
	.byte	0x4
	.uleb128 .LVL16-.LVL13
	.uleb128 .LVL17-1-.LVL13
	.uleb128 0x3
	.byte	0x73
	.sleb128 -8
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL17-1-.LVL13
	.uleb128 .LFE101-.LVL13
	.uleb128 0x4
	.byte	0xa3
	.uleb128 0x1
	.byte	0x50
	.byte	0x9f
	.byte	0
.LVUS11:
	.uleb128 .LVU74
	.uleb128 .LVU110
	.uleb128 .LVU110
	.uleb128 0
.LLST11:
	.byte	0x6
	.4byte	.LVL13
	.byte	0x4
	.uleb128 .LVL13-.LVL13
	.uleb128 .LVL17-.LVL13
	.uleb128 0x2
	.byte	0x30
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL17-.LVL13
	.uleb128 .LFE101-.LVL13
	.uleb128 0x1
	.byte	0x50
	.byte	0
.LVUS12:
	.uleb128 .LVU88
	.uleb128 0
.LLST12:
	.byte	0x8
	.4byte	.LVL15
	.uleb128 .LFE101-.LVL15
	.uleb128 0x4
	.byte	0x74
	.sleb128 128
	.byte	0x9f
	.byte	0
.LVUS14:
	.uleb128 .LVU87
	.uleb128 .LVU88
.LLST14:
	.byte	0x8
	.4byte	.LVL15
	.uleb128 .LVL15-.LVL15
	.uleb128 0x1
	.byte	0x54
	.byte	0
.LVUS5:
	.uleb128 0
	.uleb128 .LVU64
	.uleb128 .LVU64
	.uleb128 0
.LLST5:
	.byte	0x6
	.4byte	.LVL8
	.byte	0x4
	.uleb128 .LVL8-.LVL8
	.uleb128 .LVL10-.LVL8
	.uleb128 0x1
	.byte	0x50
	.byte	0x4
	.uleb128 .LVL10-.LVL8
	.uleb128 .LFE100-.LVL8
	.uleb128 0x4
	.byte	0xa3
	.uleb128 0x1
	.byte	0x50
	.byte	0x9f
	.byte	0
.LVUS6:
	.uleb128 .LVU49
	.uleb128 .LVU67
	.uleb128 .LVU67
	.uleb128 0
.LLST6:
	.byte	0x6
	.4byte	.LVL8
	.byte	0x4
	.uleb128 .LVL8-.LVL8
	.uleb128 .LVL11-.LVL8
	.uleb128 0x2
	.byte	0x30
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL11-.LVL8
	.uleb128 .LFE100-.LVL8
	.uleb128 0x1
	.byte	0x50
	.byte	0
.LVUS7:
	.uleb128 .LVU61
	.uleb128 0
.LLST7:
	.byte	0x8
	.4byte	.LVL9
	.uleb128 .LFE100-.LVL9
	.uleb128 0x4
	.byte	0x74
	.sleb128 128
	.byte	0x9f
	.byte	0
.LVUS9:
	.uleb128 .LVU60
	.uleb128 .LVU61
.LLST9:
	.byte	0x8
	.4byte	.LVL9
	.uleb128 .LVL9-.LVL9
	.uleb128 0x1
	.byte	0x54
	.byte	0
.LVUS0:
	.uleb128 0
	.uleb128 .LVU37
	.uleb128 .LVU37
	.uleb128 0
.LLST0:
	.byte	0x6
	.4byte	.LVL3
	.byte	0x4
	.uleb128 .LVL3-.LVL3
	.uleb128 .LVL5-.LVL3
	.uleb128 0x1
	.byte	0x50
	.byte	0x4
	.uleb128 .LVL5-.LVL3
	.uleb128 .LFE99-.LVL3
	.uleb128 0x1
	.byte	0x54
	.byte	0
.LVUS1:
	.uleb128 .LVU20
	.uleb128 .LVU38
	.uleb128 .LVU38
	.uleb128 0
.LLST1:
	.byte	0x6
	.4byte	.LVL3
	.byte	0x4
	.uleb128 .LVL3-.LVL3
	.uleb128 .LVL6-.LVL3
	.uleb128 0x2
	.byte	0x30
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL6-.LVL3
	.uleb128 .LFE99-.LVL3
	.uleb128 0x1
	.byte	0x50
	.byte	0
.LVUS2:
	.uleb128 .LVU31
	.uleb128 0
.LLST2:
	.byte	0x8
	.4byte	.LVL4
	.uleb128 .LFE99-.LVL4
	.uleb128 0x4
	.byte	0x75
	.sleb128 128
	.byte	0x9f
	.byte	0
.LVUS4:
	.uleb128 .LVU30
	.uleb128 .LVU31
.LLST4:
	.byte	0x8
	.4byte	.LVL4
	.uleb128 .LVL4-.LVL4
	.uleb128 0x1
	.byte	0x55
	.byte	0
.Ldebug_loc3:
	.section	.debug_aranges,"",%progbits
	.4byte	0x3c
	.2byte	0x2
	.4byte	.Ldebug_info0
	.byte	0x4
	.byte	0
	.2byte	0
	.2byte	0
	.4byte	.LFB97
	.4byte	.LFE97-.LFB97
	.4byte	.LFB98
	.4byte	.LFE98-.LFB98
	.4byte	.LFB99
	.4byte	.LFE99-.LFB99
	.4byte	.LFB100
	.4byte	.LFE100-.LFB100
	.4byte	.LFB101
	.4byte	.LFE101-.LFB101
	.4byte	0
	.4byte	0
	.section	.debug_rnglists,"",%progbits
.Ldebug_ranges0:
	.4byte	.Ldebug_ranges3-.Ldebug_ranges2
.Ldebug_ranges2:
	.2byte	0x5
	.byte	0x4
	.byte	0
	.4byte	0
.LLRL3:
	.byte	0x5
	.4byte	.LBB26
	.byte	0x4
	.uleb128 .LBB26-.LBB26
	.uleb128 .LBE26-.LBB26
	.byte	0x4
	.uleb128 .LBB31-.LBB26
	.uleb128 .LBE31-.LBB26
	.byte	0
.LLRL8:
	.byte	0x5
	.4byte	.LBB32
	.byte	0x4
	.uleb128 .LBB32-.LBB32
	.uleb128 .LBE32-.LBB32
	.byte	0x4
	.uleb128 .LBB37-.LBB32
	.uleb128 .LBE37-.LBB32
	.byte	0
.LLRL13:
	.byte	0x5
	.4byte	.LBB38
	.byte	0x4
	.uleb128 .LBB38-.LBB38
	.uleb128 .LBE38-.LBB38
	.byte	0x4
	.uleb128 .LBB43-.LBB38
	.uleb128 .LBE43-.LBB38
	.byte	0
.LLRL15:
	.byte	0x7
	.4byte	.LFB97
	.uleb128 .LFE97-.LFB97
	.byte	0x7
	.4byte	.LFB98
	.uleb128 .LFE98-.LFB98
	.byte	0x7
	.4byte	.LFB99
	.uleb128 .LFE99-.LFB99
	.byte	0x7
	.4byte	.LFB100
	.uleb128 .LFE100-.LFB100
	.byte	0x7
	.4byte	.LFB101
	.uleb128 .LFE101-.LFB101
	.byte	0
.Ldebug_ranges3:
	.section	.debug_line,"",%progbits
.Ldebug_line0:
	.section	.debug_str,"MS",%progbits,1
.LASF18:
	.ascii	"long double\000"
.LASF19:
	.ascii	"Handle\000"
.LASF70:
	.ascii	"command_id\000"
.LASF27:
	.ascii	"char\000"
.LASF33:
	.ascii	"CSND_LOOPMODE_MANUAL\000"
.LASF30:
	.ascii	"CSND_ENCODING_PCM16\000"
.LASF2:
	.ascii	"short int\000"
.LASF41:
	.ascii	"GPU_VERTEX_SHADER\000"
.LASF20:
	.ascii	"Result\000"
.LASF38:
	.ascii	"NDM_DAEMON_BOSS\000"
.LASF62:
	.ascii	"enabled\000"
.LASF26:
	.ascii	"IPC_BufferRights\000"
.LASF8:
	.ascii	"__uint32_t\000"
.LASF4:
	.ascii	"__uint16_t\000"
.LASF53:
	.ascii	"plgLdrHandle\000"
.LASF51:
	.ascii	"config\000"
.LASF43:
	.ascii	"NDSP_ENCODING_PCM8\000"
.LASF13:
	.ascii	"uint8_t\000"
.LASF47:
	.ascii	"ERROR_WORD_WRAP_FLAG\000"
.LASF22:
	.ascii	"float\000"
.LASF78:
	.ascii	"IPC_MakeHeader\000"
.LASF10:
	.ascii	"long long int\000"
.LASF23:
	.ascii	"IPC_BUFFER_R\000"
.LASF48:
	.ascii	"noFlash\000"
.LASF46:
	.ascii	"ERROR_LANGUAGE_FLAG\000"
.LASF35:
	.ascii	"CSND_LOOPMODE_ONESHOT\000"
.LASF7:
	.ascii	"long int\000"
.LASF24:
	.ascii	"IPC_BUFFER_W\000"
.LASF3:
	.ascii	"__uint8_t\000"
.LASF34:
	.ascii	"CSND_LOOPMODE_NORMAL\000"
.LASF61:
	.ascii	"parameters\000"
.LASF52:
	.ascii	"PluginLoadParameters\000"
.LASF1:
	.ascii	"unsigned char\000"
.LASF77:
	.ascii	"getThreadCommandBuffer\000"
.LASF45:
	.ascii	"NDSP_ENCODING_ADPCM\000"
.LASF76:
	.ascii	"plgLdrExit\000"
.LASF0:
	.ascii	"signed char\000"
.LASF11:
	.ascii	"long long unsigned int\000"
.LASF16:
	.ascii	"uint32_t\000"
.LASF66:
	.ascii	"getThreadLocalStorage\000"
.LASF74:
	.ascii	"/home/jmvieitez/Documents/Proyectos/SpiceRack/Baris"
	.ascii	"ta/Barista/plgldr/src/plgldr.c\000"
.LASF14:
	.ascii	"uint16_t\000"
.LASF58:
	.ascii	"cmdbuf\000"
.LASF57:
	.ascii	"svcConnectToPort\000"
.LASF32:
	.ascii	"CSND_ENCODING_PSG\000"
.LASF56:
	.ascii	"svcCloseHandle\000"
.LASF55:
	.ascii	"svcSendSyncRequest\000"
.LASF60:
	.ascii	"PLGLDR__SetPluginLoaderState\000"
.LASF15:
	.ascii	"int32_t\000"
.LASF25:
	.ascii	"IPC_BUFFER_RW\000"
.LASF5:
	.ascii	"short unsigned int\000"
.LASF17:
	.ascii	"size_t\000"
.LASF28:
	.ascii	"_Bool\000"
.LASF39:
	.ascii	"NDM_DAEMON_NIM\000"
.LASF67:
	.ascii	"IPC_Desc_Buffer\000"
.LASF75:
	.ascii	"/home/jmvieitez/Documents/Proyectos/SpiceRack/Baris"
	.ascii	"ta/Barista/plgldr/build\000"
.LASF65:
	.ascii	"plgLdrInit\000"
.LASF49:
	.ascii	"lowTitleId\000"
.LASF31:
	.ascii	"CSND_ENCODING_ADPCM\000"
.LASF9:
	.ascii	"long unsigned int\000"
.LASF42:
	.ascii	"GPU_GEOMETRY_SHADER\000"
.LASF63:
	.ascii	"PLGLDR__IsPluginLoaderEnabled\000"
.LASF68:
	.ascii	"size\000"
.LASF71:
	.ascii	"normal_params\000"
.LASF6:
	.ascii	"__int32_t\000"
.LASF72:
	.ascii	"translate_params\000"
.LASF50:
	.ascii	"path\000"
.LASF36:
	.ascii	"CSND_LOOPMODE_NORELOAD\000"
.LASF37:
	.ascii	"NDM_DAEMON_CEC\000"
.LASF59:
	.ascii	"PLGLDR__SetPluginLoadParameters\000"
.LASF54:
	.ascii	"plgLdrRefCount\000"
.LASF64:
	.ascii	"isEnabled\000"
.LASF21:
	.ascii	"double\000"
.LASF29:
	.ascii	"CSND_ENCODING_PCM8\000"
.LASF12:
	.ascii	"unsigned int\000"
.LASF69:
	.ascii	"rights\000"
.LASF44:
	.ascii	"NDSP_ENCODING_PCM16\000"
.LASF40:
	.ascii	"NDM_DAEMON_FRIENDS\000"
.LASF73:
	.ascii	"GNU C17 12.1.0 -mword-relocations -mtune=mpcore -mf"
	.ascii	"loat-abi=hard -mtp=soft -marm -march=armv6k -g -O2 "
	.ascii	"-ffunction-sections -fdata-sections -fomit-frame-po"
	.ascii	"inter\000"
	.ident	"GCC: (devkitARM release 58) 12.1.0"
