// src/main.rs

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("week6/include/blobstore.h");

        type BlobstoreClient;

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;

        
    }
}

fn main() {
    let _client = ffi::new_blobstore_client();
    println!("Hello, world!");
}
