; ModuleID = 'probe8.9d0448b46baa2d41-cgu.0'
source_filename = "probe8.9d0448b46baa2d41-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

; core::f64::<impl f64>::to_ne_bytes
; Function Attrs: inlinehint uwtable
define internal i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$11to_ne_bytes17h1397939104655694E"(double %self) unnamed_addr #0 {
start:
  %_0 = alloca [8 x i8], align 1
  store double %self, ptr %_0, align 1
  %0 = load i64, ptr %_0, align 1
  ret i64 %0
}

; probe8::probe
; Function Attrs: uwtable
define void @_ZN6probe85probe17h76c0f7eb977177b9E() unnamed_addr #1 {
start:
  %0 = alloca [8 x i8], align 8
  %_1 = alloca [8 x i8], align 1
; call core::f64::<impl f64>::to_ne_bytes
  %1 = call i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$11to_ne_bytes17h1397939104655694E"(double 3.140000e+00)
  store i64 %1, ptr %0, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %_1, ptr align 8 %0, i64 8, i1 false)
  ret void
}

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #2

attributes #0 = { inlinehint uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse3,+sahf" }
attributes #1 = { uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse3,+sahf" }
attributes #2 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.88.0 (6b00bc388 2025-06-23)"}
