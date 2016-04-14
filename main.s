	.text
	.file	"main.0.rs"
	.section	.text._ZN4main20h55948912334af32deaaE,"ax",@progbits
	.align	16, 0x90
	.type	_ZN4main20h55948912334af32deaaE,@function
_ZN4main20h55948912334af32deaaE:
	.cfi_startproc
	retq
.Lfunc_end0:
	.size	_ZN4main20h55948912334af32deaaE, .Lfunc_end0-_ZN4main20h55948912334af32deaaE
	.cfi_endproc

	.section	.text.main,"ax",@progbits
	.globl	main
	.align	16, 0x90
	.type	main,@function
main:
	.cfi_startproc
	subq	$24, %rsp
.Ltmp0:
	.cfi_def_cfa_offset 32
	leaq	_ZN4main20h55948912334af32deaaE(%rip), %rax
	movq	%rdi, 16(%rsp)
	movq	%rax, %rdi
	movq	16(%rsp), %rax
	movq	%rsi, 8(%rsp)
	movq	%rax, %rsi
	movq	8(%rsp), %rdx
	callq	_ZN2rt10lang_start20h0d9dcd8707a91319UoyE@PLT
	addq	$24, %rsp
	retq
.Lfunc_end1:
	.size	main, .Lfunc_end1-main
	.cfi_endproc


	.section	".note.GNU-stack","",@progbits
