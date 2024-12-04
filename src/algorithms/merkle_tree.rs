use rs_merkle::algorithms::Sha256;
use rs_merkle::{Hasher, MerkleProof, MerkleTree};

pub fn merkle() {
    // Create tree
    let leaf_values = ["a", "b", "c", "d"];

    let leaves: Vec<[u8; 32]> = leaf_values
        .iter()
        .map(|x| Sha256::hash(x.as_bytes()))
        .collect();

    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
    let merkle_root = merkle_tree
        .root()
        .ok_or("couldn't get the merkle root")
        .unwrap();

    println!("The root of the merkle tree is: {:?}", merkle_root);

    // Generate and verify proof
    let index_to_prove = [1];
    let leaf_to_prove = *leaves.get(1).ok_or("Cant get leaf at index").unwrap();
    let merkle_proof = merkle_tree.proof(&index_to_prove);

    let proof_bytes = merkle_proof.to_bytes();

    let proof = MerkleProof::<Sha256>::try_from(proof_bytes).unwrap();

    let verified = proof.verify(merkle_root, &index_to_prove, &[leaf_to_prove], leaves.len());

    assert!(verified);

    println!("Merkle proof for leaf 'b' was verified in the merkle root as {verified}");
}
