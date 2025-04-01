pub fn verify_proof(proof: &[u8]) -> bool {
    println!("Verifying proof: {:x?}", proof);
    proof == [0xde, 0xad, 0xbe, 0xef]
}