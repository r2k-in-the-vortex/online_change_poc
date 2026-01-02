; ModuleID = 'got_redirection_demo'
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

; --- The Internal GOT ---
; Initialized: index 0 = main, index 1 = foo
@__custom_got = weak_odr global [2 x i8*] [
    i8* bitcast (i32 ()* @main to i8*),
    i8* bitcast (i32 ()* @foo to i8*)
], align 16

; --- The Redirectable Pointer ---
; By default, this points to the internal array above.
; Your Rust loader should find this symbol and overwrite it with the mmap address.
@__custom_got_pointer = global [2 x i8*]* @__custom_got, align 8

define i32 @foo() {
entry:
  ; Returns 67 to signify change
  ret i32 67
}

define i32 @main() {
entry:
  ; 1. Load the current address of the GOT table from the redirectable pointer
  %got_base = load [2 x i8*]*, [2 x i8*]** @__custom_got_pointer, align 8

  ; 2. Get the address of index 1 (the 'foo' slot) within that table
  %foo_slot = getelementptr inbounds [2 x i8*], [2 x i8*]* %got_base, i32 0, i32 1

  ; 3. Load the function pointer from the slot
  %foo_raw = load i8*, i8** %foo_slot, align 8

  ; 4. Cast the raw pointer to the correct function signature
  %foo_fn = bitcast i8* %foo_raw to i32 ()*

  ; 5. Call foo and return its result
  %result = call i32 %foo_fn()
  ret i32 %result
}
