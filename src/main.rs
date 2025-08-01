use num_traits::ops::checked::CheckedMul;
use num_traits::ops::checked::CheckedAdd;

struct Vector32 {
    x: u32,
    y: u32,
    z: u32,
}

struct Vector64 {
    x: u64,
    y: u64,
    z: u64,
}

struct Vector3d32 {
    v1: Vector32,
    v2: Vector32,
    v3: Vector32,
}

struct Vector3d64 {
    v1: Vector64,
    v2: Vector64,
    v3: Vector64,
}

fn dot(v1: Vector32, v2: Vector32) -> (Option<u32>) {
    let x: Option<u32> = v1.x.checked_mul(v2.x);
    let y: Option<u32> = v1.y.checked_mul(v2.y);
    let z: Option<u32> = v1.z.checked_mul(v2.z);
    let sum1: Option<u32> = x?.checked_add(y?);
    return sum1?.checked_add(z?);
}

fn dot64(v1: Vector64, v2: Vector64) -> (Option<u64>) {
    let x: Option<u64> = v1.x.checked_mul(v2.x);
    let y: Option<u64> = v1.y.checked_mul(v2.y);
    let z: Option<u64> = v1.z.checked_mul(v2.z);
    let sum1: Option<u64> = x?.checked_add(y?);
    return sum1?.checked_add(z?);
}

fn dot3d32(v1: Vector3d32, v2: Vector3d32) -> (Option<Vector32>){

    // v2 will need to be transposed
    //| ax1 ay1 az1 |     | bx1 bx2 bx3 |    | (ax1 * bx1) + (ay1 * by1) + (az1 * bz1) |
    //| ax2 ay2 az2 | dot | by1 by2 by3 | =  | (ax2 * bx2) + (ay2 * by2) + (az2 * bz2) |
    //| ax3 ay3 az3 |     | bz1 bz2 bz3 |    | (ax3 * bx3) + (ay3 * by3) + (az3 * bz3) |

    let x1: Option<u32> = v1.v1.x.checked_mul(v1.v1.x);
    let x2: Option<u32> = v1.v1.y.checked_mul(v1.v1.y);
    let x3: Option<u32> = v1.v1.z.checked_mul(v1.v1.z);
    let mut sumx = x1?.checked_add(x2?);
    sumx = sumx?.checked_add(x3?);

    let y1: Option<u32> = v1.v2.x.checked_mul(v2.v2.x);
    let y2: Option<u32> = v1.v2.y.checked_mul(v2.v2.y);
    let y3: Option<u32> = v1.v2.z.checked_mul(v2.v2.z);
    let mut sumy: Option<u32> = y1?.checked_add(y2?);
    sumy = sumy?.checked_add(y3?);

    let z1: Option<u32> = v1.v3.x.checked_mul(v2.v3.x);
    let z2: Option<u32> = v1.v3.y.checked_mul(v2.v3.y);
    let z3: Option<u32> = v1.v3.z.checked_mul(v2.v3.z);
    let mut sumz: Option<u32> = z1?.checked_add(z2?);
    sumz = sumz?.checked_add(z3?);

    if (sumx != None) && (sumy != None) || (sumz != None) {

        Some(Vector32 {
            x: sumx?,
            y: sumy?,
            z: sumz?
        })
    }
    else {
        None
    }
}

fn dot3d64(v1: Vector3d64, v2: Vector3d64) -> (Option<Vector64>){

    // v2 will need to be transposed
    //| ax1 ay1 az1 |     | bx1 bx2 bx3 |    | (ax1 * bx1) + (ay1 * by1) + (az1 * bz1) |
    //| ax2 ay2 az2 | dot | by1 by2 by3 | =  | (ax2 * bx2) + (ay2 * by2) + (az2 * bz2) |
    //| ax3 ay3 az3 |     | bz1 bz2 bz3 |    | (ax3 * bx3) + (ay3 * by3) + (az3 * bz3) |

    let x1: Option<u64> = v1.v1.x.checked_mul(v1.v1.x);
    let x2: Option<u64> = v1.v1.y.checked_mul(v1.v1.y);
    let x3: Option<u64> = v1.v1.z.checked_mul(v1.v1.z);
    let mut sumx = x1?.checked_add(x2?);
    sumx = sumx?.checked_add(x3?);

    let y1: Option<u64> = v1.v2.x.checked_mul(v2.v2.x);
    let y2: Option<u64> = v1.v2.y.checked_mul(v2.v2.y);
    let y3: Option<u64> = v1.v2.z.checked_mul(v2.v2.z);
    let mut sumy: Option<u64> = y1?.checked_add(y2?);
    sumy = sumy?.checked_add(y3?);

    let z1: Option<u64> = v1.v3.x.checked_mul(v2.v3.x);
    let z2: Option<u64> = v1.v3.y.checked_mul(v2.v3.y);
    let z3: Option<u64> = v1.v3.z.checked_mul(v2.v3.z);
    let mut sumz: Option<u64> = z1?.checked_add(z2?);
    sumz = sumz?.checked_add(z3?);

    if (sumx != None) && (sumy != None) || (sumz != None) {

        Some(Vector64 {
            x: sumx?,
            y: sumy?,
            z: sumz?
        })
    }
    else {
        None
    }
}

fn main() {
    let c1: Vector32 = Vector32 {x: 1, y: 2, z:3};
    println!("x: {}, y: {}, z: {}", c1.x, c1.y, c1.z);
}
