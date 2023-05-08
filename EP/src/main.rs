/*Ricardo Ferreira Padilha
Implementação do benchmark NPB 3.4.2 "Embarrasingly Paralel" (EP)
*/
use std::f64;
use std::env;
use std::thread::available_parallelism;
use std::time::{Instant};
use std::sync::{Mutex, Once};
use std::thread;
use std::sync::mpsc;


mod auxfunctions;

fn main() {
    //"M" deve ser lido pela linha de comando, representando a potência que escala o tamanho do problema
    // S = 24, W = 25, A = 28,B = 30,C = 32,D = 36,E = 40,F = 44
    static mut M: i32 = 44;
    static INIT: Once = Once::new();//used to initialize M only once
    lazy_static::lazy_static! {
        static ref M_MUTEX: Mutex<i32> = Mutex::new(unsafe { M });
    }
    // Get the command line arguments
    let class;//default value
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {//determines problem size according to selected class, and attributes m value and class accordingly
        let arg = &args[1];
        if let Some(first_char) = arg.chars().next() {
            if first_char == 'S' {
                INIT.call_once(|| {
                    unsafe { M = 24 };
                });
            } else if first_char == 'W' {
                INIT.call_once(|| {
                    unsafe { M = 25 };
                });
            } else if first_char == 'A' {
                INIT.call_once(|| {
                    unsafe { M = 28 };
                });
            } else if first_char == 'B' {
                INIT.call_once(|| {
                    unsafe { M = 30 };
                });
            } else if first_char == 'C' {
                INIT.call_once(|| {
                    unsafe { M = 32 };
                });
            } else if first_char == 'D' {
                INIT.call_once(|| {
                    unsafe { M = 36 };
                });
            } else if first_char == 'E' {
                INIT.call_once(|| {
                    unsafe { M = 40 };
                });
            } else if first_char == 'F' {
                INIT.call_once(|| {
                    unsafe { M = 44 };
                });
            } else {
                panic!("Stopping execution, problem class not specified.");
            }
        }
    }else{
        panic!("Stopping execution, problem class not specified.");
    }

    const MK: i32 = 8;//original is 16
    const NK: usize = 2i32.pow(MK as u32) as usize;// atualmente 256 com MK = 8
    const NQ: usize = 10;

    let mops: f64;//milhoes de operacoes, calculado ao final

    let num_threads: usize = available_parallelism().unwrap().get();//numero de threads disponivel
    let base = 2;
    let base = unsafe{i32::pow(base,(M+1) as u32)};//quantidade de numeros aleatorios gerados

    let nit: i32 = 0;//numero de iteracoes, calculado ao final

    let mut x = [-1.0; 2 * NK];//vetor de tamanho 512, iniciado com -1 em todas as posicoes

    let tm: f64;//tempo decorrido da execucao das threads

    let mut gc: f64 = 0.0;//numero de pares

    let mut _dum1: f64 = 1.0;
    let mut dum2: f64 = 1.0;
    let dum3: f64 = 1.0;

    let verified: bool;

    let mut q: [f64; NQ] = [0.0; NQ];//vetor com a soma das quantidades de pares gerados pelas threads
    //cada thread possui um vetor "qq" do mesmo tamanho de q, que calcula a quantidade de pares gerados a cada iteracao

    let mut threads = Vec::with_capacity(num_threads);//quantas threads serão geradas


    println!("NAS Parallel Benchmarks (NPB3.4-OMP) - EP Benchmark");
    println!("Number of random numbers generated: {}", base);
    println!("Maximum number of threads available: {}", num_threads);

    _dum1 = auxfunctions::randlc(&mut dum2, dum3);//como dum é um vetor de 1s, segue com 1s

    let (tx2, rx2) = mpsc::channel();//canal de comunicação para o timer 2
    let (tx1, rx1) = mpsc::channel();//canal de comunicação para o timer 1


    let start_time = std::time::Instant::now();//inicia timer

    let mut sx = 0.0;
    let mut sy = 0.0;//somatorios

    

    for tid in 0..num_threads {
        let nk = (NK / num_threads) +1;//tamanho dos 'chunks'
        let start = tid * nk;//offset do chunk de cada thread
        let end = (tid+1) * nk;//offset do chunk de cada thread
        let tx2_clone = tx2.clone();
        let tx1_clone = tx1.clone();

        threads.push(thread::spawn(move || {
            let mut sx = 0.0;
            let mut sy = 0.0;

            let s = 31269.0;
            let an = 132608.0;
            let mut t1;
            let mut t2;
            let mut t3;
            let mut t4;
            let mut ik;
            let mut x1;
            let mut x2;
            let mut l;
            let mut qq: [f64; NQ] = [0.0; NQ];

            for k in start..end {
                let mut kk = start + k;
                t1 = s;
                t2 = an;

                // Find starting seed t1 for this kk.
                //timer 2 starts here
                let start_time2 = Instant::now();
                for _i in 1..=100 {
                    ik = kk / 2;
                    if 2 * ik != kk {
                        t3 = auxfunctions::randlc(&mut t1, t2);
                    }
                    if ik == 0 {
                        break;
                    }
                    let t2copy = t2;
                    t3 = auxfunctions::randlc(&mut t2, t2copy);
                    kk = ik;
                }

                // Compute uniform pseudorandom numbers.
                auxfunctions::vranlc(2 * nk as i32, t1, an,  &mut x);

                //timer 2 ends here
                let elapsed_time2 = start_time2.elapsed().as_secs_f64();
                tx2_clone.send(elapsed_time2).unwrap();

                //timer 1 starts here
                let start_time1 = Instant::now();
                // Compute Gaussian deviates by acceptance-rejection method and 
                // tally counts in concentric square annuli.
                for i in 0..nk {
                    x1 = 2.0 * x[2 * i] - 1.0;
                    x2 = 2.0 * x[2 * i + 1] - 1.0;
                    t1 = x1.powi(2) + x2.powi(2);
                    
                    if t1 <= 1.0 {
                        t2 = (-2.0 * t1.ln() / t1).sqrt();
                        t3 = x1 * t2;
                        t4 = x2 * t2;
                        l = t3.abs().max(t4.abs()) as usize;
                        qq[l] += 1.0;
                        sx += t3;
                        sy += t4;
                    }
                }
                
                //timer 1 ends here
                let elapsed_time1 = start_time1.elapsed().as_secs_f64();
                tx1_clone.send(elapsed_time1).unwrap();
            }

            (sx, sy, qq)
        }));
    }
    for thread in threads {
        let (sx_thread, sy_thread, qq_thread) = thread.join().unwrap();
        sx += sx_thread;
        sy += sy_thread;
        for i in 0..NQ {
            q[i] += qq_thread[i];
        }
    }
    for i in 0..NQ {
        gc += q[i];
    }
    
    // Receive the elapsed time from the spawned threads
    let elapsed_time2 = rx2.recv().unwrap();
    let elapsed_time1 = rx1.recv().unwrap();

    let end_time = std::time::Instant::now();//encerra timer

    tm = end_time.duration_since(start_time).as_secs_f64();
    let mut tt = tm;

    let m = unsafe{M};

    (verified, class) = auxfunctions::verify(m, sx, sy, gc);

    mops = 2.0_f64.powi(m + 1) / tm / 1_000_000.0;

    println!("EP Benchmark Results:\n
    CPU Time = {:.3}\n
    N = 2^{}\n
    No. Gaussian Pairs = {:.0}\n
    Sums = {:25.15} {:25.15}", tm, m, gc, sx, sy);

    println!("\nCounts:");
    for i in 0..NQ {
        println!("{:3} {:.15}", i, q[i]);
    }

    println!("EP");
    println!("Class = {}", class);
    println!("Size = {}",base);
    println!("Iterations = {}",nit);
    println!("Time in seconds = {}",tm);//time in seconds
    println!("Number of threads = {}",num_threads);
    println!("Mop/s total = {}",mops);
    println!("Mop/s per thread = {}",mops/(num_threads as f64));//mops per thread
    println!("\nRandom Numbers Generated");
    println!("Verification = {}",if verified{"Successful"} else {"Unsuccessful"});
    println!("Total time:    {:.3} ({:.2}%)", tt, tt*100.0/tm);
    tt = elapsed_time2;
    println!("Gaussian pairs: {:.3} ({:.2}%)", tt, tt*100.0/tm);
    tt = elapsed_time1;
    println!("Random numbers: {:.3} ({:.2}%)", tt, tt*100.0/tm);
}