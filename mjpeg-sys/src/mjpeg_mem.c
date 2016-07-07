#include "mjpeg_mem.h"
#include <stdlib.h>

struct mjpeg_decoder *mjpeg_decoder_alloc(void) {
  struct mjpeg_decoder *mj = calloc(1, sizeof(struct mjpeg_decoder));
  //printf("mjpeg_decoder_alloc() returning %p\n", mj);
  return mj;
}

void mjpeg_decoder_free(struct mjpeg_decoder *mj) {
    //printf("mjpeg_decoder_free(%p)\n", mj);
    free(mj);
}
