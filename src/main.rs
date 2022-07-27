use rand::Rng;
use std::arch::x86_64::_pdep_u64;
use std::arch::x86_64::_popcnt64;
use std::arch::x86_64::_tzcnt_u64;

fn main() {
    test();
}

fn test() {
    let mut counts: [i32; 64] = [0; 64];

    for _i in 0..100000 {
        let idx = get_random_bit_index();
        counts[idx] += 1;
    }
    for i in 0..64 {
        println!("{}: {}", i, counts[i]);
    }
}

fn get_random_bit_index() -> usize {
    let mut rng = rand::thread_rng();
    let mask = 0xdddddddddddddddd as u64;   // 0b1101...

    let rand_num: u32 = rng.gen();
    unsafe {
        let cnt = _popcnt64(mask as i64);
        let rand_pos: u32 = rand_num % cnt as u32;
        let res = _tzcnt_u64(_pdep_u64((1u64 << rand_pos) as u64, mask)) as usize;
        return res;
    }
}
