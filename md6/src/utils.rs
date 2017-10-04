use consts::*;
//replacing #define in md6_compress ln 98
//TODO: will need to change for non-standard word size in consts
const loop_inputs: [(i32,i32,i32);16] = [
    (10,11, 0), //RL00
    ( 5,24, 1), //RL01
    (13, 9, 2), //RL02
    (10,16, 3), //RL03
    (11,15, 4), //RL04
    (12, 9, 5), //RL05
    ( 2,27, 6), //RL06
    ( 7,15, 7), //RL07
    (14, 6, 8), //RL08
    (15, 2, 9), //RL09
    ( 7,29,10), //RL10
    (13, 8,11), //RL11
    (11,15,12), //RL12
    ( 7, 5,13), //RL13
    ( 6,31,14), //RL14
    (12, 9,15), //RL15
];

//A is a pointer to an index of md6_words, rc+n in length
fn md6_main_compression_loop(A: &mut [MD6_WORD], r: i32) {
    let mut x: MD6_WORD;
    let mut S: MD6_WORD = S0;
    let mut i = MD6_N;
    //no good stable for loop step by
    let mut j = 0;
    while j < (r * MD6_C) {
        for index in 0..16 {
            let step = loop_inputs[index].2;
            let ls = loop_inputs[index].1;
            let rs = loop_inputs[index].0;
            //from #define loop_body, md6_compress.c
            x = S;                                /* feedback constant     */
            x ^= A[(i + step - T5) as usize];                /* end-around feedback   */
            x ^= A[(i+step-T0) as usize];                    /* linear feedback       */ 
            x ^= ( A[(i+step-T1) as usize] & A[(i+step-T2) as usize] ); /* first quadratic term  */ 
            x ^= ( A[(i+step-T3) as usize] & A[(i+step-T4) as usize] ); /* second quadratic term */ 
            x ^= (x >> rs);                       /* right-shift           */ 
            A[(i+step) as usize] = x ^ (x << ls);
        }
        /* Advance round constant S to the next round constant. */
        S = (S << 1) ^ (S >> (MD6_W-1)) ^ (S & Smask);
        //increment i
        i += 16;
        j += MD6_C;
    }
}