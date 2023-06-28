#include <stdint.h>

// SPI
#ifdef PIN_SPI_MISO
#undef PIN_SPI_MISO
#undef PIN_SPI_MOSI
#undef PIN_SPI_SCK
#undef PIN_SPI_SS
#endif

#define PIN_SPI_MISO  (4u)
#define PIN_SPI_MOSI  (3u)
#define PIN_SPI_SCK   (2u)
#define PIN_SPI_SS    (5u)