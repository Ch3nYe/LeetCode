pub mod problem_0164_maximum_gap;
pub mod problem_xxxx_Count_Integer_Partitions;

fn main() {

    let res = problem_0164_maximum_gap::bukets_and_pigeonhole_v2::Solution::maximum_gap(Vec::from([3, 6, 9, 1]));
    println!("res: {:?}",res);
    println!("[+]main.rs| Compile Success");
}
