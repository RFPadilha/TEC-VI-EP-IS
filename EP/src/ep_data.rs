pub mod ep_data {
    // The following include file is generated automatically by the
    // "setparams" utility, which defines the problem size 'm'.

    // M is the Log_2 of the number of complex pairs of uniform (0, 1) random
    // numbers.  MK is the Log_2 of the size of each batch of uniform random
    // numbers.  MK can be set for convenience on a given system, since it does
    // not affect the results.
    const MK: i32 = 16;
    const MM: i32 = M - MK;
    const NN: i32 = 2_i32.pow(MM);
    const NK: i32 = 2_i32.pow(MK);
    const NQ: i32 = 10;

    const A: f64 = 1220703125.0;
    const S: f64 = 271828183.0;

    // Storage
    static mut X: [f64; 2 * NK as usize] = [0.0; 2 * NK as usize];
    static mut QQ: [f64; (NQ + 1) as usize] = [0.0; (NQ + 1) as usize];
    static mut Q: [f64; (NQ + 1) as usize] = [0.0; (NQ + 1) as usize];
    // NOTE: Rust doesn't have a direct equivalent of OpenMP's "threadprivate" directive.
    // For simplicity, we'll just use static mut variables and assume that each thread will
    // only access its own portion of the arrays.

    // Timer constants
    const T_TOTAL: usize = 1;
    const T_GPAIRS: usize = 2;
    const T_RANDN: usize = 3;
    const T_RCOMM: usize = 4;
    const T_LAST: usize = 4;
}