pub fn vranlc(n: i32, mut x: f64, a: f64, y: &mut [f64]) {
    let r23: f64 = 2.0_f64.powi(-23);
    let r46: f64 = r23 * r23;
    let t23: f64 = 2.0_f64.powi(23);
    let t46: f64 = t23 * t23;
    let mut xv: [f64; 64] = [0.0; 64];
    let mut t1: f64;
    let mut t2: f64;
    let mut t3: f64;
    let mut t4: f64;
    let an: f64;
    let mut a1: f64 = 1.0;
    let mut a2: f64 = 1.0;
    let mut x1: i64;
    let mut x2: f64;
    let mut yy: f64;
    let mut n1: i32;

    t1 = x;
    n1 = std::cmp::min(n, 64);

    for i in 0..n1 {
        xv[i as usize] = t46 * randlc(&mut t1, a);
    }

    if n > 64 {
        t1 = a;
        t2 = r46 * a;

        for _i in 0..63 {
            t2 = randlc(&mut t1, a);
        }

        an = t46 * t2;

        t1 = r23 * an;
        a1 = t1.trunc();
        a2 = an - t23 * a1 as f64;
    }

    for j in (0..n).step_by(64) {
        n1 = std::cmp::min(64, n - j);

        for i in 0..n1 {
            y[j as usize + i as usize] = r46 * xv[i as usize];
        }

        if j + n1 == n {
            x = xv[(n1-1) as usize];
            break;
        }

        for i in 0..64 {
            t1 = r23 * xv[i as usize];
            x1 = t1.trunc() as i64;
            x2 = xv[i as usize] - t23 * x1 as f64;
            t1 = a1 * x2 + a2 * x1 as f64;
            t2 = t1.trunc();
            yy = t1 - t23 * t2 as f64;
            t3 = t23 * yy + a2 * x2;
            t4 = t3.trunc();
            xv[i as usize] = t3 - t46 * t4 as f64;
        }
    }
}
//.try_into().unwrap()
pub fn randlc(x: &mut f64, a: f64) -> f64 {
    let r23: f64 = 0.5_f64.powi(23);
    let r46: f64 = r23.powi(2);
    let t23: f64 = 2_f64.powi(23);
    let t46: f64 = t23.powi(2);
    
    let t1 = r23 * a;
    let a1 = t1.floor();
    let a2 = a - t23 * a1;
    
    let t1 = r23 * *x;
    let x1 = t1.floor();
    let x2 = *x - t23 * x1;
    
    let t1 = a1 * x2 + a2 * x1;
    let t2 = (r23 * t1).floor();
    let z = t1 - t23 * t2;
    let t3 = t23 * z + a2 * x2;
    let t4 = (r46 * t3).floor();
    *x = t3 - t46 * t4;
    
    r46 * *x
}

pub fn verify(m: i32, sx: f64, sy: f64, gc: f64) -> (bool, char) {
    let mut verified = true;
    let class;
    
    let mut sx_verify_value: f64 = 0.0;
    let mut sy_verify_value: f64 = 0.0;
    let mut gc_verify_value: f64 = 0.0;
    let sx_err: f64;
    let sy_err: f64;
    let gc_err: f64;
    
    let epsilon = 1.0e-8;

    if m == 24 {
        class = 'S';
        sx_verify_value = 1.051299420395306e7;
        sy_verify_value = 1.051517131857535e7;
        gc_verify_value = 13176389.0;
    } else if m == 25 {
        class = 'W';
        sx_verify_value = 2.102505525182392e7;
        sy_verify_value = 2.103162209578822e7;
        gc_verify_value = 26354769.0;
    } else if m == 28 {
        class = 'A';
        sx_verify_value = 1.682235632304711e8;
        sy_verify_value = 1.682195123368299e8;
        gc_verify_value = 210832767.0;
    } else if m == 30 {
        class = 'B';
        sx_verify_value = 6.728927543423024e8;
        sy_verify_value = 6.728951822504275e8;
        gc_verify_value = 843345606.0;
    } else if m == 32 {
        class = 'C';
        sx_verify_value = 2.691444083862931e9;
        sy_verify_value = 2.691519118724585e9;
        gc_verify_value = 3373275903.0;
    } else if m == 36 {
        class = 'D';
        sx_verify_value = 4.306350280812112e10;
        sy_verify_value = 4.306347571859157e10;
        gc_verify_value = 53972171957.0;
    } else if m == 40 {
        class = 'E';
        sx_verify_value = 6.890169663167274e11;
        sy_verify_value = 6.890164670688535e11;
        gc_verify_value = 863554308186.0;
    } else if m == 44 {
        class = 'F';
        sx_verify_value = 1.102426773788175e13;
        sy_verify_value = 1.102426773787993e13;
        gc_verify_value = 13816870608324.0;
    } else {
        class = 'U';
        verified = false;
    }

    if verified {
        sx_err = (sx - sx_verify_value).abs() / sx_verify_value;
        sy_err = (sy - sy_verify_value).abs() / sy_verify_value;
        if sx_err.is_nan() || sy_err.is_nan() {
            verified = false;
        } else {
            verified = sx_err.le(&epsilon) && sy_err.le(&epsilon);
        }
    }
    
    if verified {
        gc_err = (gc - gc_verify_value).abs() / gc_verify_value;
        if gc_err.is_nan() || gc_err.gt(&epsilon) {
            verified = false;
        }
    }

    return (verified, class);
}
