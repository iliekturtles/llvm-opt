# llvm-opt
Given a struct `struct P<D> { value: f32, d: PhantomData<D>, }` that implements `std::ops::Div` why does llvm not optimize the struct as an `f32` for a simple function to divide values?

Given a raw `f32` or a struct `struct V { value: f32, }` the function is optimized. The `f32` and `struct V` variants generate the same llvm-ir and asm. The `struct P<D>` has a return type of `i32` and an extra `bitcast` instruction in the llvm-ir and an extra `movd` instruction in the asm.

Code to reproduce llvm-ir and asm is included in the repository. Run `build.sh` to generate .ll and .s files in the `target/release/examples/` folder. Output below generated using `rustc 1.13.0 (2c6933acc 2016-11-07)`.

`f32` function implemention. `struct V` and `struct P<D>` implementations are nearly identical:
```rust
fn calc(v: f32) -> f32 {
    let a = v * 1.234_f32;
    let b = 5.0_f32;

    a / b
}
```


f32:
```llvm
; Function Attrs: noinline norecurse nounwind readnone uwtable
define internal fastcc float @_ZN7opt_f324calc17h83aad1073897de13E(float) unnamed_addr #2 {
entry-block:
  %1 = fmul float %0, 0x3FF3BE76C0000000
  %2 = fdiv float %1, 5.000000e+00
  ret float %2
```
```asm
	.seh_endprologue
	mulss	__real@3f9df3b6(%rip), %xmm0
	divss	__real@40a00000(%rip), %xmm0
	retq
	.seh_handlerdata
```


struct V:
```llvm
; Function Attrs: noinline norecurse nounwind readnone uwtable
define internal fastcc float @_ZN7opt_v324calc17hbc6edc82fc0fb0dcE(float) unnamed_addr #2 {
entry-block:
  %1 = fmul float %0, 0x3FF3BE76C0000000
  %2 = fdiv float %1, 5.000000e+00
  ret float %2
```
```asm
	.seh_endprologue
	mulss	__real@3f9df3b6(%rip), %xmm0
	divss	__real@40a00000(%rip), %xmm0
	retq
	.seh_handlerdata
```

  
struct P\<D\>:
```llvm
; Function Attrs: noinline norecurse nounwind readnone uwtable
define internal fastcc i32 @_ZN7opt_p324calc17h0c0275d6390115c6E(float) unnamed_addr #2 {
entry-block:
  %1 = fmul float %0, 0x3FF3BE76C0000000
  %2 = fdiv float %1, 5.000000e+00
  %3 = bitcast float %2 to i32
  ret i32 %3
````
```asm
	.seh_endprologue
	mulss	__real@3f9df3b6(%rip), %xmm0
	divss	__real@40a00000(%rip), %xmm0
	movd	%xmm0, %eax
	retq
	.seh_handlerdata
```
  
