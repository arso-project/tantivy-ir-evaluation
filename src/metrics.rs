
    // Precision at k
    pub fn p_at_k(results: Vec<i32>, benchmark: Vec<i32>, k: usize) -> (f32) {
        let (mut counter, mut p) = (0.0,0.0);
        if k == 0{
            return 0 as f32}
        for i in 0..k{
            if results.len() > i as usize && benchmark.contains(&results[i as usize]){
                p = p + 1.0;
                counter += 1.0;
            }
            else{
                counter += 1.0;
            }}
        return p / counter
        
    }
    // Average Prescision
    pub fn ap(results: Vec<i32>, benchmark: Vec<i32>) -> (f32) {
        let mut ap_sum = 0.0;
        for i in 0..results.len(){
            if benchmark.contains(&results[i as usize]){
                ap_sum += p_at_k(results.clone(), benchmark.clone(), i+1);
            }
            //println!("REsults: {} {} {} ",ap_sum, benchmark.len(), ap_sum/benchmark.len() as f32);
        }
        ap_sum/benchmark.len() as f32
        
    }


#[test]
fn test_ap(){
     assert_eq!(
            0.525,
            ap(
                vec![7, 17, 9, 42, 5],
                vec![5, 7, 12, 42]
                )
            );

}
#[test]
fn test_p_at_k() {
        assert_eq!(
            0.5,
            p_at_k(
                vec![1,2,3,4,5,6,7,8,9,10],
                vec![2,4,6,8,10,12,14,16,18,20], 
                10
                )
            );
        assert_eq!(
            0.0,
            p_at_k(
                vec![5, 3, 6, 1, 2],
                vec![1, 2, 5, 6, 7, 8], 
                0
                )
            );
        assert_eq!(
            0.75,
            p_at_k(
                vec![5, 3, 6, 1, 2],
                vec![1, 2, 5, 6, 7, 8], 
                4
                )
            );
            assert_eq!(
            0.5,
            p_at_k(
                vec![5, 3, 6, 1, 2],
                vec![1, 2, 5, 6, 7, 8], 
                8
                )
            );
        
        
    }

    
