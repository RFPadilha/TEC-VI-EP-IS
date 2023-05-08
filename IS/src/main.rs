use std::env;
use std::thread::available_parallelism;
use std::thread;
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use rand::Rng;


fn generate_random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

fn randlc(x: &mut BigDecimal, a: f64, ks: &mut i32, t23: &mut f64, t46: &mut f64, r23: &mut f64, r46: &mut f64) -> f64{
    let mut t1: f64;
    let t2: f64;
    let t3: f64;
    let t4: f64;
    let a1: f64;
    let a2: f64;
    let x1: f64;
    let x2: f64;
    let z: f64;
    let mut j: f64;
    let retval: f64;


    if *ks == 0 {
        *r23 = 1.0;
        *r46 = 1.0;
        *t23 = 1.0;
        *t46 = 1.0;

        for _i in 1..23{
        *r23 = 0.50 * *r23;
        *t23 = 2.0 * *t23;
        }
        for _i in 1..46{
        *r46 = 0.50 * *r46;
        *t46 = 2.0 * *t46;
        }
        *ks = 1;
    }
    /*  Break A into two parts such that A = 2^23 * A1 + A2 and set X = N.  */
    
    t1 = *r23 * a;
    j  = t1;
    a1 = j;
    a2 = BigDecimal::to_f64(&(BigDecimal::from_f64(a).unwrap() - BigDecimal::from_f64(*t23).unwrap() * BigDecimal::from_f64(a1).unwrap())).unwrap();
     //a - *t23 * a1;//perda de precisão de mantissa nessa operação

    /*  Break X into two parts such that X = 2^23 * X1 + X2, compute
    Z = A1 * X2 + A2 * X1  (mod 2^23), and then
    X = 2^23 * Z + A2 * X2  (mod 2^46).                            */

    t1 = BigDecimal::to_f64(&(BigDecimal::from_f64(*r23).unwrap() * x.clone())).unwrap();
    //*r23 * *x;
    j  = t1;
    x1 = j;
    x2 = BigDecimal::to_f64(&(x.clone() - BigDecimal::from_f64(*t23).unwrap() * BigDecimal::from_f64(x1).unwrap())).unwrap();
    //*x - *t23 * x1;
    t1 = a1 * x2 + a2 * x1;
    
    j  = BigDecimal::to_f64(&(BigDecimal::from_f64(*r23).unwrap() * BigDecimal::from_f64(t1).unwrap())).unwrap();
    //*r23 * t1;
    t2 = j;
    z = BigDecimal::to_f64(&(BigDecimal::from_f64(t1).unwrap() - BigDecimal::from_f64(*t23).unwrap() * BigDecimal::from_f64(t2).unwrap())).unwrap();
    //t1 - *t23 * t2;
    t3 =    BigDecimal::to_f64(&(BigDecimal::from_f64(*t23).unwrap() * BigDecimal::from_f64(z).unwrap() + 
            BigDecimal::from_f64(a2).unwrap() * BigDecimal::from_f64(x2).unwrap())).unwrap();
    //*t23 * z + a2 * x2;
    j  = *r46 * t3;
    t4 = j;
    *x = BigDecimal::from_f64(t3).unwrap() - BigDecimal::from_f64(*t46).unwrap() * BigDecimal::from_f64(t4).unwrap();
    //t3 - *t46 * t4;

    retval = *r46 * BigDecimal::to_f64(&x.clone()).unwrap();
    //*r46 * *x;
    
    retval
}

fn find_my_seed(kn: i32, np: i32, nn: i64, s: f64, a: f64, ks: &mut i32, t23: &mut f64,t46: &mut f64,r23: &mut f64,r46: &mut f64) -> f64{
    let mut t1: BigDecimal;
    let mut t2: BigDecimal;

    let mq: i64;
    let nq: i64;
    let mut kk: i64;
    let mut ik: i64;

    if kn == 0{
        return s;
    }

    mq = (nn/4 + (np as i64) - 1) / (np as i64);
    nq = mq * 4 * (kn as i64);               /* number of rans to be skipped */

    t1 = BigDecimal::from_f64(s).unwrap();
    t2 = BigDecimal::from_f64(a).unwrap();
    kk = nq;
    while kk > 1{
        ik = kk / 2;
        if 2 * ik ==  kk {
            randlc( &mut t2, a, ks, t23, t46, r23, r46);
            kk = ik;
        }
        else {
            let anew = BigDecimal::to_f64(&t2).unwrap();
            randlc( &mut t1, anew, ks,  t23, t46, r23, r46 );
            kk = kk - 1;
        }
    }
    let anew = BigDecimal::to_f64(&t2).unwrap();
    randlc( &mut t1, anew, ks,  t23, t46, r23, r46);

    return BigDecimal::to_f64(&t1).unwrap();
}

fn create_seq(seed: f64, a: f64, num_keys: i64, max_keys: i64, vec_size: i64) -> Vec<i64> {
    let num_threads = available_parallelism().unwrap().get();
    let chunk_size = vec_size / num_threads as i64;
    let mut ret_vec: Vec<i64> = Vec::new();

    let mut handles = Vec::with_capacity(num_threads);

    for thread_id in 0..num_threads{
        
        let myid: i64 = thread_id as i64;
        let k = max_keys / 4;
        
        let mut ks: i32 = 0;
        let mut r23: f64 = 0.0;
        let mut r46: f64 = 0.0; 
        let mut t23: f64 = 0.0;
        let mut t46: f64 = 0.0;
        let s = find_my_seed(myid as i32, num_threads as i32, (4 * num_keys) as i64, seed, a,&mut ks,&mut t23,&mut t46,&mut r23,&mut r46);

        let handle = thread::spawn(move || {
                        


            let mut s_copy = BigDecimal::from_f64(s).unwrap();//"borrow" em s, para que as threads possam operar com seu valor sem modificar
            let an = a;


            let mut sub_array: Vec<i64> = Vec::with_capacity(chunk_size as usize);

            for _i in 0..chunk_size {
                let mut x = randlc(&mut s_copy, an,&mut ks, &mut t23,&mut t46,&mut r23,&mut r46);
                x += randlc(&mut s_copy, an,&mut ks, &mut t23,&mut t46,&mut r23,&mut r46 );
                x += randlc(&mut s_copy, an,&mut ks, &mut t23,&mut t46,&mut r23,&mut r46 );
                x += randlc(&mut s_copy, an,&mut ks, &mut t23,&mut t46,&mut r23,&mut r46 );
                let mut new_element = k * x.to_bits() as i64;
                //caso randlc seja insuficiente para gerar os valores aleatórios, utiliza outra função auxiliar de uma crate importada do rust
                if new_element == 0 {
                    new_element = generate_random_f64().to_bits() as i64;
                }
                sub_array.push(new_element);
            }

            sub_array
        });
        handles.push(handle);
    }

    for handle in handles {
        //receive and concat subarrays here
        let sub_array = handle.join().unwrap();
        ret_vec.extend(sub_array);
    }
    return ret_vec;
}

fn parallel_bucket_sort(arr: Vec<i64>, num_buckets: usize) -> Vec<i64> {
    let len = arr.len();

    // create an array of empty buckets
    let mut buckets: Vec<Vec<i64>> = vec![vec![]; num_buckets];

    //this is what the "rank" function does in the C program, separating the values into the appropriate buckets
    // scatter the values from the input array into the buckets
    for i in 0..len {
        let bucket_idx = (arr[i] as f64 * num_buckets as f64 / std::i64::MAX as f64) as usize;
        buckets[bucket_idx].push(arr[i]);
    }

    // sort each bucket using multiple threads
    //this is what the "full_verify" function does in the C program, it sorts the keys, then verifies that they are in order
    let mut handles = Vec::new();
    for i in 0..num_buckets {
        let bucket = std::mem::take(&mut buckets[i]);
        let handle = thread::spawn(move || {
            let mut bucket = bucket;
            bucket.sort();
            bucket
        });
        handles.push(handle);
    }

    // concatenate the sorted buckets back into a single array
    let mut sorted_vec = Vec::with_capacity(len);
    for handle in handles {
        let bucket = handle.join().unwrap();
        sorted_vec.extend(bucket);
    }

    
    
    for i in 0..sorted_vec.len(){
        println!("sorted_vec: {}", sorted_vec[i as usize]);
    }

    return sorted_vec;
}

//function used to verify the if the sorting was correct
fn verify_sort(vec: &Vec<i64>) -> i64 {
    let mut count = 0;

    for i in 1..vec.len() {
        if vec[i - 1] > vec[i] {
            count += 1;
        }
    }

    count
}


fn print_results(total_keys: i64, class: char, total_ks1: i64, passed_verification: i64, num_threads: usize, tm0: f64, tm1: f64, tm2: f64, mut tm3: f64){
    println!( "\n\n NAS Parallel Benchmarks (NPB3.4-OMP) - IS Benchmark\n\n" );
    println!( " Size:  {}  (class {})\n", total_keys, class );
    println!( " Iterations:  {}\n", 10 );
    println!( " Number of available threads:  {}\n", num_threads );
    println!( "\n" );

    if passed_verification != 5*(10 as i64)+1{
        println!("Values out of order: {}", passed_verification);
    }

    let mops: f64 = 1.0e-6*(total_keys as f64)*(10 as f64 );


    println!("\n\n IS Benchmark Completed\n" );
    println!( " Class           =                        {}\n", class );
    println!(" Size            =             {}\n", total_ks1);
    println!(" Iterations      =             {}\n", 10);
    println!(" Time in seconds =             {}\n", tm0);
    println!(" Total threads   =             {}\n", num_threads);
    println!(" Mop/s total     =             {}\n", mops);
    println!(" Mop/s/thread    =             {}\n", mops/(num_threads as f64));
    println!(" Operation type  = keys ranked\n");
    /*
    if passed_verification < 0 {
        println!( " Verification    =            NOT PERFORMED\n" );
    }
    else if passed_verification > 0 {
        println!( " Verification    =               SUCCESSFUL\n" );
    }
    else{
        println!( " Verification    =             UNSUCCESSFUL\n" );
    }
    */
    let mut time_percent: f64;

    println!("\nAdditional timers -\n");
    println!(" Total execution :{} \n", tm3);
    if tm3 == 0.0{
        tm3 = 1.0;
    }
    time_percent = tm1/tm3*100.0;
    println!(" Initialization : {} ({}%)", tm1, time_percent);
    time_percent = tm0/tm3*100.0;
    println!(" Benchmarking : {} ({}%)", tm0, time_percent);
    time_percent = tm2/tm3*100.0;
    println!("Sorting : {} ({}%)", tm2, time_percent);

}


fn s_class(){
    let passed_verification: i64;
    let class = 'S';

    //variables that depend on the above class dependant variables
    let total_keys: i64 = 1 << 16;
    let total_ks1: i64 = total_keys;
    let max_key: i64 = 1 << 11;
    let num_buckets: usize = 1 << 9;//entre 512 e 1024 buckets
    let num_keys: i64 = total_keys;
    let buffer_size: i64 = num_keys;
    //non-mutable variables
    const MAX_ITER: usize = 10;

    //main arrays
    let mut key_array: Vec<i64> = Vec::with_capacity(buffer_size as usize);
    //different key array for each class, to be known at compile time

    let num_threads: usize = available_parallelism().unwrap().get();//numero de threads disponivel
    
    let c1: f64 = 314159265.0;//parametros iniciais de create_seq
    let c2: f64 = 1220703125.0;
    
    
    
    let start_time3 = std::time::Instant::now();//inicia timer
    //key_array deve ter tamanho conhecido em tempo de compilação, por consequencia, vetores diferentes para cada classe

    //--------------------------------------------------------------------------------------
    //preenche keyArray com valores aleatórios
    let start_time1 = std::time::Instant::now();//inicia timer
    key_array = create_seq(c1,c2, num_keys, max_key, key_array.len() as i64);
    
    let end_time1 = std::time::Instant::now();//encerra timer
    let tm1 = end_time1.duration_since(start_time1).as_secs_f64();
    //--------------------------------------------------------------------------------------

    if class != 'S'{
        println!("\n iteration\n");
    }

    //--------------------------------------------------------------------------------------
    let start_time0 = std::time::Instant::now();//inicia timer
    for iter in 1..MAX_ITER{
        if class != 'S'{
            println!("\n {}\n", iter);
        }
        key_array = parallel_bucket_sort(key_array, num_buckets);
    }
    let end_time0 = std::time::Instant::now();//encerra timer
    let tm0 = end_time0.duration_since(start_time0).as_secs_f64();
    //--------------------------------------------------------------------------------------


    //--------------------------------------------------------------------------------------
    let start_time2 = std::time::Instant::now();//inicia timer
    //fullVerify();
    passed_verification = verify_sort(&key_array);
    let end_time2 = std::time::Instant::now();//encerra timer
    let tm2 = end_time2.duration_since(start_time2).as_secs_f64();
    //--------------------------------------------------------------------------------------

    let end_time3 = std::time::Instant::now();//encerra timer
    let tm3 = end_time3.duration_since(start_time3).as_secs_f64();
    //fim da execução, restante é somente verificação e impressão de resultados

    print_results(total_keys, class, total_ks1, passed_verification, num_threads, tm0, tm1, tm2, tm3);
    

}
fn w_class(){
    let passed_verification: i64;
    let class = 'W';
    let total_keys: i64 = 1 << 20;
    let total_ks1: i64 = total_keys;
    let max_key: i64 = 1 << 16;
    let num_buckets: usize = 1 << 10;//entre 512 e 1024 buckets
    let num_keys: i64 = total_keys;
    let buffer_size: i64 = num_keys;
    //non-mutable variables
    const MAX_ITER: usize = 10;

    //main arrays
    let mut key_array: Vec<i64> = Vec::with_capacity(buffer_size as usize);
    //different key array for each class, to be known at compile time

    let num_threads: usize = available_parallelism().unwrap().get();//numero de threads disponivel
    
    let c1: f64 = 314159265.0;//parametros iniciais de create_seq
    let c2: f64 = 1220703125.0;
    
    
    
    let start_time3 = std::time::Instant::now();//inicia timer
    //key_array deve ter tamanho conhecido em tempo de compilação, por consequencia, vetores diferentes para cada classe

    //--------------------------------------------------------------------------------------
    //preenche keyArray com valores aleatórios
    let start_time1 = std::time::Instant::now();//inicia timer
    key_array = create_seq(c1,c2, num_keys, max_key, key_array.len() as i64);
    
    let end_time1 = std::time::Instant::now();//encerra timer
    let tm1 = end_time1.duration_since(start_time1).as_secs_f64();
    //--------------------------------------------------------------------------------------

    if class != 'S'{
        println!("\n iteration\n");
    }

    //--------------------------------------------------------------------------------------
    let start_time0 = std::time::Instant::now();//inicia timer
    for iter in 1..MAX_ITER{
        if class != 'S'{
            println!("\n {}\n", iter);
        }
        key_array = parallel_bucket_sort(key_array, num_buckets);
    }
    let end_time0 = std::time::Instant::now();//encerra timer
    let tm0 = end_time0.duration_since(start_time0).as_secs_f64();
    //--------------------------------------------------------------------------------------


    //--------------------------------------------------------------------------------------
    let start_time2 = std::time::Instant::now();//inicia timer
    //fullVerify();
    passed_verification = verify_sort(&key_array);
    let end_time2 = std::time::Instant::now();//encerra timer
    let tm2 = end_time2.duration_since(start_time2).as_secs_f64();
    //--------------------------------------------------------------------------------------

    let end_time3 = std::time::Instant::now();//encerra timer
    let tm3 = end_time3.duration_since(start_time3).as_secs_f64();
    //fim da execução, restante é somente verificação e impressão de resultados

    print_results(total_keys, class, total_ks1, passed_verification, num_threads, tm0, tm1, tm2, tm3);
    
}
fn a_class(){
    let passed_verification: i64;
    let class = 'A';
    let total_keys: i64 = 1 << 23;
    let total_ks1: i64 = total_keys;
    let max_key: i64 = 1 << 19;
    let num_buckets: usize = 1 << 10;//entre 512 e 1024 buckets
    let num_keys: i64 = total_keys;
    let buffer_size: i64 = num_keys;
    //non-mutable variables
    const MAX_ITER: usize = 10;

    //main arrays
    let mut key_array: Vec<i64> = Vec::with_capacity(buffer_size as usize);
    //different key array for each class, to be known at compile time

    let num_threads: usize = available_parallelism().unwrap().get();//numero de threads disponivel
    
    let c1: f64 = 314159265.0;//parametros iniciais de create_seq
    let c2: f64 = 1220703125.0;
    
    
    
    let start_time3 = std::time::Instant::now();//inicia timer
    //key_array deve ter tamanho conhecido em tempo de compilação, por consequencia, vetores diferentes para cada classe

    //--------------------------------------------------------------------------------------
    //preenche keyArray com valores aleatórios
    let start_time1 = std::time::Instant::now();//inicia timer
    key_array = create_seq(c1,c2, num_keys, max_key, key_array.len() as i64);
    
    let end_time1 = std::time::Instant::now();//encerra timer
    let tm1 = end_time1.duration_since(start_time1).as_secs_f64();
    //--------------------------------------------------------------------------------------

    if class != 'S'{
        println!("\n iteration\n");
    }

    //--------------------------------------------------------------------------------------
    let start_time0 = std::time::Instant::now();//inicia timer
    for iter in 1..MAX_ITER{
        if class != 'S'{
            println!("\n {}\n", iter);
        }
        key_array = parallel_bucket_sort(key_array, num_buckets);
    }
    let end_time0 = std::time::Instant::now();//encerra timer
    let tm0 = end_time0.duration_since(start_time0).as_secs_f64();
    //--------------------------------------------------------------------------------------


    //--------------------------------------------------------------------------------------
    let start_time2 = std::time::Instant::now();//inicia timer
    //fullVerify();
    passed_verification = verify_sort(&key_array);
    let end_time2 = std::time::Instant::now();//encerra timer
    let tm2 = end_time2.duration_since(start_time2).as_secs_f64();
    //--------------------------------------------------------------------------------------

    let end_time3 = std::time::Instant::now();//encerra timer
    let tm3 = end_time3.duration_since(start_time3).as_secs_f64();
    //fim da execução, restante é somente verificação e impressão de resultados

    print_results(total_keys, class, total_ks1, passed_verification, num_threads, tm0, tm1, tm2, tm3);
    
}
fn b_class(){
    let passed_verification: i64;
    let class = 'B';
    //variables that depend on the above class dependant variables
    let total_keys: i64 = 1 << 25;
    let total_ks1: i64 = total_keys;
    let max_key: i64 = 1 << 21;
    let num_buckets: usize = 1 << 10;//entre 512 e 1024 buckets
    let num_keys: i64 = total_keys;
    let buffer_size: i64 = num_keys;
    //non-mutable variables
    const MAX_ITER: usize = 10;

    //main arrays
    let mut key_array: Vec<i64> = Vec::with_capacity(buffer_size as usize);
    //different key array for each class, to be known at compile time

    let num_threads: usize = available_parallelism().unwrap().get();//numero de threads disponivel
    
    let c1: f64 = 314159265.0;//parametros iniciais de create_seq
    let c2: f64 = 1220703125.0;
    
    
    
    let start_time3 = std::time::Instant::now();//inicia timer
    //key_array deve ter tamanho conhecido em tempo de compilação, por consequencia, vetores diferentes para cada classe

    //--------------------------------------------------------------------------------------
    //preenche keyArray com valores aleatórios
    let start_time1 = std::time::Instant::now();//inicia timer
    key_array = create_seq(c1,c2, num_keys, max_key, key_array.len() as i64);
    
    let end_time1 = std::time::Instant::now();//encerra timer
    let tm1 = end_time1.duration_since(start_time1).as_secs_f64();
    //--------------------------------------------------------------------------------------

    if class != 'S'{
        println!("\n iteration\n");
    }

    //--------------------------------------------------------------------------------------
    let start_time0 = std::time::Instant::now();//inicia timer
    for iter in 1..MAX_ITER{
        if class != 'S'{
            println!("\n {}\n", iter);
        }
        key_array = parallel_bucket_sort(key_array, num_buckets);
    }
    let end_time0 = std::time::Instant::now();//encerra timer
    let tm0 = end_time0.duration_since(start_time0).as_secs_f64();
    //--------------------------------------------------------------------------------------


    //--------------------------------------------------------------------------------------
    let start_time2 = std::time::Instant::now();//inicia timer
    //fullVerify();
    passed_verification = verify_sort(&key_array);
    let end_time2 = std::time::Instant::now();//encerra timer
    let tm2 = end_time2.duration_since(start_time2).as_secs_f64();
    //--------------------------------------------------------------------------------------

    let end_time3 = std::time::Instant::now();//encerra timer
    let tm3 = end_time3.duration_since(start_time3).as_secs_f64();
    //fim da execução, restante é somente verificação e impressão de resultados

    print_results(total_keys, class, total_ks1, passed_verification, num_threads, tm0, tm1, tm2, tm3);
    
}
fn c_class(){
    let passed_verification: i64;
    let class = 'C';
    let total_keys: i64 = 1 << 27;
    let total_ks1: i64 = total_keys;
    let max_key: i64 = 1 << 23;
    let num_buckets: usize = 1 << 10;//entre 512 e 1024 buckets
    let num_keys: i64 = total_keys;
    let buffer_size: i64 = num_keys;
    //non-mutable variables
    const MAX_ITER: usize = 10;

    //main arrays
    let mut key_array: Vec<i64> = Vec::with_capacity(buffer_size as usize);
    //different key array for each class, to be known at compile time

    let num_threads: usize = available_parallelism().unwrap().get();//numero de threads disponivel
    
    let c1: f64 = 314159265.0;//parametros iniciais de create_seq
    let c2: f64 = 1220703125.0;
    
    
    
    let start_time3 = std::time::Instant::now();//inicia timer
    //key_array deve ter tamanho conhecido em tempo de compilação, por consequencia, vetores diferentes para cada classe

    //--------------------------------------------------------------------------------------
    //preenche keyArray com valores aleatórios
    let start_time1 = std::time::Instant::now();//inicia timer
    key_array = create_seq(c1,c2, num_keys, max_key, key_array.len() as i64);
    
    let end_time1 = std::time::Instant::now();//encerra timer
    let tm1 = end_time1.duration_since(start_time1).as_secs_f64();
    //--------------------------------------------------------------------------------------

    if class != 'S'{
        println!("\n iteration\n");
    }

    //--------------------------------------------------------------------------------------
    let start_time0 = std::time::Instant::now();//inicia timer
    for iter in 1..MAX_ITER{
        if class != 'S'{
            println!("\n {}\n", iter);
        }
        key_array = parallel_bucket_sort(key_array, num_buckets);
    }
    let end_time0 = std::time::Instant::now();//encerra timer
    let tm0 = end_time0.duration_since(start_time0).as_secs_f64();
    //--------------------------------------------------------------------------------------


    //--------------------------------------------------------------------------------------
    let start_time2 = std::time::Instant::now();//inicia timer
    //fullVerify();
    passed_verification = verify_sort(&key_array);
    let end_time2 = std::time::Instant::now();//encerra timer
    let tm2 = end_time2.duration_since(start_time2).as_secs_f64();
    //--------------------------------------------------------------------------------------

    let end_time3 = std::time::Instant::now();//encerra timer
    let tm3 = end_time3.duration_since(start_time3).as_secs_f64();
    //fim da execução, restante é somente verificação e impressão de resultados

    print_results(total_keys, class, total_ks1, passed_verification, num_threads, tm0, tm1, tm2, tm3);
    
}
fn d_class(){
    let passed_verification: i64;
    let class = 'D';
    let total_keys: i64 = 1 << 31;
    let total_ks1: i64 = total_keys;
    let max_key: i64 = 1 << 27;
    let num_buckets: usize = 1 << 10;//entre 512 e 1024 buckets
    let num_keys: i64 = total_keys;
    let buffer_size: i64 = num_keys;
    //non-mutable variables
    const MAX_ITER: usize = 10;

    //main arrays
    let mut key_array: Vec<i64> = Vec::with_capacity(buffer_size as usize);
    //different key array for each class, to be known at compile time

    let num_threads: usize = available_parallelism().unwrap().get();//numero de threads disponivel
    
    let c1: f64 = 314159265.0;//parametros iniciais de create_seq
    let c2: f64 = 1220703125.0;
    
    
    
    let start_time3 = std::time::Instant::now();//inicia timer
    //key_array deve ter tamanho conhecido em tempo de compilação, por consequencia, vetores diferentes para cada classe

    //--------------------------------------------------------------------------------------
    //preenche keyArray com valores aleatórios
    let start_time1 = std::time::Instant::now();//inicia timer
    key_array = create_seq(c1,c2, num_keys, max_key, key_array.len() as i64);
    
    let end_time1 = std::time::Instant::now();//encerra timer
    let tm1 = end_time1.duration_since(start_time1).as_secs_f64();
    //--------------------------------------------------------------------------------------

    if class != 'S'{
        println!("\n iteration\n");
    }

    //--------------------------------------------------------------------------------------
    let start_time0 = std::time::Instant::now();//inicia timer
    for iter in 1..MAX_ITER{
        if class != 'S'{
            println!("\n {}\n", iter);
        }
        key_array = parallel_bucket_sort(key_array, num_buckets);
    }
    let end_time0 = std::time::Instant::now();//encerra timer
    let tm0 = end_time0.duration_since(start_time0).as_secs_f64();
    //--------------------------------------------------------------------------------------


    //--------------------------------------------------------------------------------------
    let start_time2 = std::time::Instant::now();//inicia timer
    //fullVerify();
    passed_verification = verify_sort(&key_array);
    let end_time2 = std::time::Instant::now();//encerra timer
    let tm2 = end_time2.duration_since(start_time2).as_secs_f64();
    //--------------------------------------------------------------------------------------

    let end_time3 = std::time::Instant::now();//encerra timer
    let tm3 = end_time3.duration_since(start_time3).as_secs_f64();
    //fim da execução, restante é somente verificação e impressão de resultados

    print_results(total_keys, class, total_ks1, passed_verification, num_threads, tm0, tm1, tm2, tm3);
    
}
fn e_class(){
    let passed_verification: i64;
    let class = 'E';
    let total_keys: i64 = 1 << 35;
    let total_ks1: i64 = total_keys;
    let max_key: i64 = 1 << 31;
    let num_buckets: usize = 1 << 10;//entre 512 e 1024 buckets
    let num_keys: i64 = total_keys;
    let buffer_size: i64 = num_keys;
    //non-mutable variables
    const MAX_ITER: usize = 10;

    //main arrays
    let mut key_array: Vec<i64> = Vec::with_capacity(buffer_size as usize);
    //different key array for each class, to be known at compile time

    let num_threads: usize = available_parallelism().unwrap().get();//numero de threads disponivel
    
    let c1: f64 = 314159265.0;//parametros iniciais de create_seq
    let c2: f64 = 1220703125.0;
    
    
    
    let start_time3 = std::time::Instant::now();//inicia timer
    //key_array deve ter tamanho conhecido em tempo de compilação, por consequencia, vetores diferentes para cada classe

    //--------------------------------------------------------------------------------------
    //preenche keyArray com valores aleatórios
    let start_time1 = std::time::Instant::now();//inicia timer
    key_array = create_seq(c1,c2, num_keys, max_key, key_array.len() as i64);
    
    let end_time1 = std::time::Instant::now();//encerra timer
    let tm1 = end_time1.duration_since(start_time1).as_secs_f64();
    //--------------------------------------------------------------------------------------

    if class != 'S'{
        println!("\n iteration\n");
    }

    //--------------------------------------------------------------------------------------
    let start_time0 = std::time::Instant::now();//inicia timer
    for iter in 1..MAX_ITER{
        if class != 'S'{
            println!("\n {}\n", iter);
        }
        key_array = parallel_bucket_sort(key_array, num_buckets);
    }
    let end_time0 = std::time::Instant::now();//encerra timer
    let tm0 = end_time0.duration_since(start_time0).as_secs_f64();
    //--------------------------------------------------------------------------------------


    //--------------------------------------------------------------------------------------
    let start_time2 = std::time::Instant::now();//inicia timer
    //fullVerify();
    passed_verification = verify_sort(&key_array);
    let end_time2 = std::time::Instant::now();//encerra timer
    let tm2 = end_time2.duration_since(start_time2).as_secs_f64();
    //--------------------------------------------------------------------------------------

    let end_time3 = std::time::Instant::now();//encerra timer
    let tm3 = end_time3.duration_since(start_time3).as_secs_f64();
    //fim da execução, restante é somente verificação e impressão de resultados

    print_results(total_keys, class, total_ks1, passed_verification, num_threads, tm0, tm1, tm2, tm3);
    
}



fn main(){
    let args: Vec<String> = env::args().collect();
    
    if args.len() <= 1 {//determines problem size according to selected class and attributes variable values accordingly
        panic!("Class not specified");
    }else{// if args.len() > 1 
        let arg = &args[1];
        if let Some(first_char) = arg.chars().next() {
            if first_char == 'S' {
                s_class();
            }else if first_char == 'W' {
                w_class();
            }else if first_char == 'A' {
                a_class();
            }else if first_char == 'B' {
                b_class();
            }else if first_char == 'C' {
                c_class();
            }else if first_char == 'D' {
                d_class();
            }else if first_char == 'E' {
                e_class();
            }else  {//classe não definida
                panic!("Class not specified");
            }
        }
    }
}