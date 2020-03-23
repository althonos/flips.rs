#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif

uint32_t crc32(const uint8_t* data, size_t len);
uint32_t crc32_update(const uint8_t* data, size_t len, uint32_t crc);

#ifdef __cplusplus
}
#endif
