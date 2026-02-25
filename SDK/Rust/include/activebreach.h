#pragma once
#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
    Launches ActiveBreach dispatcher thread.

    Returns:
        0  -> success
        !=0 -> error code
*/
uint32_t activebreach_launch(void);

/*
    Issue a syscall through ActiveBreach.

    name      -> NT syscall name (null terminated string)
    args      -> pointer to argument array
    args_len  -> number of arguments (max 16)

    Returns:
        Syscall result (NTSTATUS, handle, etc.)
*/
size_t ab_call(
    const char* name,
    const size_t* args,
    size_t args_len
);

/*
    Violation system
*/
uint32_t ab_violation_count(void);

/* Violation codes passed to ViolationHandler */
#define AB_VIOLATION_TEB_MISMATCH        0u
#define AB_VIOLATION_SUSPICIOUS_CALLER   1u
#define AB_VIOLATION_DEBUGGER_DETECTED   2u
#define AB_VIOLATION_HARDWARE_BREAKPOINT 3u

typedef void (*ViolationHandler)(uint32_t violation_type);

void ab_set_violation_handler(ViolationHandler handler);
void ab_clear_violation_handler(void);

#ifdef __cplusplus
}
#endif
