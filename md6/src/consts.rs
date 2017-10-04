//Types
pub type MD6_WORD = u64;
pub type MD6_Control_word = u64;
pub type MD6_NodeID = u64;
//Constants

//from md6.h:

//defined for standard variant. If this changes, type MD6_WORD
//needs to change
pub const MD6_W: i32 = 64;
pub const MD6_N: i32 = 89; /* size of compression input block, in words  */
pub const MD6_C: i32 = 16; /* size of compression output, in words       */
                        /* a c-word block is also called a "chunk"    */
const MD6_MAX_R: i32 = 255; /* max allowable value for number r of rounds */
pub const MD6_Q: i32 = 15;                      /* # Q words in compression block (>=0)       */
pub const MD6_K: i32 = 8;                       /* # key words per compression block (>=0)    */
pub const MD6_U: i32 = (64/(MD6_W as i32));     /* # words for unique node ID (0 or 64/w)     */
pub const MD6_V: i32 = (64/(MD6_W as i32));     /* # words for control word (0 or 64/w)       */
pub const MD6_B: i32 = 64;                      /* # data words per compression block (>0)    */
pub const MD6_DEFAULT_L: i32 = 64;              /* large so that MD6 is fully hierarchical */
pub const MD6_MAX_STACK_HEIGHT: i32 = 29;
    /* max_stack_height determines the maximum number of bits that
    ** can be processed by this implementation (with default L) to be:
    **    (b*w) * ((b/c) ** (max_stack_height-3)
    **    = 2 ** 64  for b = 64, w = 64, c = 16, and  max_stack_height = 29
    ** (We lose three off the height since level 0 is unused,
    ** level 1 contains the input data, and C has 0-origin indexing.)
    ** The smallest workable value for MD6_max_stack_height is 3.
    ** (To avoid stack overflow for non-default L values, 
    ** we should have max_stack_height >= L + 2.)
    ** (One level of storage could be saved by letting st->N[] use
    ** 1-origin indexing, since st->N[0] is now unused.)
    */

//from md6_compress.c
//TODO: make changeablee based on non-standard word size
pub const S0: MD6_WORD = 0x01234567;
pub const Smask: MD6_WORD = 0x7311c281;
pub const T0: i32 = 17;
pub const T1: i32 = 18;
pub const T2: i32 = 21;
pub const T3: i32 = 31;
pub const T4: i32 = 67;
pub const T5: i32 = 89;
