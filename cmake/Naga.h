#pragma once

#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif

// Compile SPIR-V to WGSL. Data is allocated with malloc and must be freed with free().
char* NagaSpirvToWgsl(uint32_t* SpirvData, size_t SpirvSize);

#ifdef __cplusplus
}
#endif
