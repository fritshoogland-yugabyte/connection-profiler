use crate::password;

#[test]
fn test_encrypt_scram_sha_256() {
    // Specify the salt to make the test deterministic. Any bytes will do.
    let salt: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    assert_eq!(
        password::scram_sha_256_salt(b"secret", salt),
        "SCRAM-SHA-256$4096:AQIDBAUGBwgJCgsMDQ4PEA==$8rrDg00OqaiWXJ7p+sCgHEIaBSHY89ZJl3mfIsf32oY=:05L1f+yZbiN8O0AnO40Og85NNRhvzTS57naKRWCcsIA="
    );
}

#[test]
fn test_encrypt_md5() {
    assert_eq!(
        password::md5(b"secret", "foo"),
        "md54ab2c5d00339c4b2a4e921d2dc4edec7"
    );
}
